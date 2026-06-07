#![recursion_limit = "256"]

//! This crate contains codegen code that is used by all codegen backends (LLVM and others).
//! The backend-agnostic functions of this crate use functions defined in various traits that
//! have to be implemented by each backend.

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_data_structures::unord::UnordMap;
use rustc_hir::CRATE_HIR_ID;
use rustc_hir::attrs::{CfgEntry, NativeLibKind, WindowsSubsystemKind};
use rustc_hir::def_id::CrateNum;
use rustc_lint_defs::builtin::LINKER_INFO;
use rustc_macros::{Decodable, Encodable};
use rustc_metadata::EncodedMetadata;
use rustc_middle::lint::LevelAndSource;
use rustc_middle::middle::debugger_visualizer::DebuggerVisualizerFile;
use rustc_middle::middle::dependency_format::Dependencies;
use rustc_middle::middle::exported_symbols::SymbolExportKind;
use rustc_middle::ty::TyCtxt;
use rustc_middle::util::Providers;
use rustc_serialize::opaque::{FileEncoder, MemDecoder};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_session::Session;
use rustc_session::config::{CrateType, OutputFilenames, OutputType, RUST_CGU_EXT};
use rustc_session::cstore::{self, CrateSource};
use rustc_session::lint::builtin::LINKER_MESSAGES;
use rustc_span::Symbol;

pub mod assert_module_sources;
pub mod back;
pub mod base;
pub mod codegen_attrs;
pub mod common;
pub mod errors;
pub mod target_features;
pub mod traits;

#[derive(Debug, Encodable, Decodable)]
pub struct CompiledModule {
    pub name: String,
    pub kind: ModuleKind,
    pub object: Option<PathBuf>,
    pub dwarf_object: Option<PathBuf>,
    pub bytecode: Option<PathBuf>,
    pub assembly: Option<PathBuf>, // --emit=asm
    pub llvm_ir: Option<PathBuf>,  // --emit=llvm-ir, llvm-bc is in bytecode
    pub links_from_incr_cache: Vec<PathBuf>,
}

impl CompiledModule {
    /// Call `emit` function with every artifact type currently compiled
    pub fn for_each_output(&self, mut emit: impl FnMut(&Path, OutputType)) {
        if let Some(path) = self.object.as_deref() {
            emit(path, OutputType::Object);
        }
        if let Some(path) = self.bytecode.as_deref() {
            emit(path, OutputType::Bitcode);
        }
        if let Some(path) = self.llvm_ir.as_deref() {
            emit(path, OutputType::LlvmAssembly);
        }
        if let Some(path) = self.assembly.as_deref() {
            emit(path, OutputType::Assembly);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Encodable, Decodable)]
pub enum ModuleKind {
    Regular,
    Allocator,
}

// This is the same as `rustc_session::cstore::NativeLib`, except:
// - (important) the `foreign_module` field is missing, because it contains a `DefId`, which can't
//   be encoded with `FileEncoder`.
// - (less important) the `verbatim` field is a `bool` rather than an `Option<bool>`, because here
//   we can treat `false` and `absent` the same.
#[derive(Clone, Debug, Encodable, Decodable)]
pub struct NativeLib {
    pub kind: NativeLibKind,
    pub name: Symbol,
    pub filename: Option<Symbol>,
    pub cfg: Option<CfgEntry>,
    pub verbatim: bool,
    pub dll_imports: Vec<cstore::DllImport>,
}

impl From<&cstore::NativeLib> for NativeLib {
    fn from(lib: &cstore::NativeLib) -> Self {
        NativeLib {
            kind: lib.kind,
            filename: lib.filename,
            name: lib.name,
            cfg: lib.cfg.clone(),
            verbatim: lib.verbatim.unwrap_or(false),
            dll_imports: lib.dll_imports.clone(),
        }
    }
}

/// Misc info we load from metadata to persist beyond the tcx.
///
/// Note: though `CrateNum` is only meaningful within the same tcx, information within `CrateInfo`
/// is self-contained. `CrateNum` can be viewed as a unique identifier within a `CrateInfo`, where
/// `used_crate_source` contains all `CrateSource` of the dependents, and maintains a mapping from
/// identifiers (`CrateNum`) to `CrateSource`. The other fields map `CrateNum` to the crate's own
/// additional properties, so that effectively we can retrieve each dependent crate's `CrateSource`
/// and the corresponding properties without referencing information outside of a `CrateInfo`.
// rustc_codegen_cranelift needs a Clone impl for its jit mode, which isn't tested in rust CI
#[derive(Clone, Debug, Encodable, Decodable)]
pub struct CrateInfo {
    pub target_cpu: String,
    pub target_features: Vec<String>,
    pub crate_types: Vec<CrateType>,
    pub exported_symbols: UnordMap<CrateType, Vec<(String, SymbolExportKind)>>,
    pub linked_symbols: FxIndexMap<CrateType, Vec<(String, SymbolExportKind)>>,
    pub local_crate_name: Symbol,
    pub compiler_builtins: Option<CrateNum>,
    pub profiler_runtime: Option<CrateNum>,
    pub is_no_builtins: FxHashSet<CrateNum>,
    pub native_libraries: FxIndexMap<CrateNum, Vec<NativeLib>>,
    pub crate_name: UnordMap<CrateNum, Symbol>,
    pub used_libraries: Vec<NativeLib>,
    pub used_crate_source: UnordMap<CrateNum, Arc<CrateSource>>,
    pub used_crates: Vec<CrateNum>,
    pub dependency_formats: Arc<Dependencies>,
    pub windows_subsystem: Option<WindowsSubsystemKind>,
    pub natvis_debugger_visualizers: BTreeSet<DebuggerVisualizerFile>,
    pub lint_levels: CodegenLintLevels,
    pub metadata_symbol: String,
}

/// Target-specific options that get set in `cfg(...)`.
///
/// RUSTC_SPECIFIC_FEATURES should be skipped here, those are handled outside codegen.
pub struct TargetConfig {
    /// Options to be set in `cfg(target_features)`.
    pub target_features: Vec<Symbol>,
    /// Options to be set in `cfg(target_features)`, but including unstable features.
    pub unstable_target_features: Vec<Symbol>,
    /// Option for `cfg(target_has_reliable_f16)`, true if `f16` basic arithmetic works.
    pub has_reliable_f16: bool,
    /// Option for `cfg(target_has_reliable_f16_math)`, true if `f16` math calls work.
    pub has_reliable_f16_math: bool,
    /// Option for `cfg(target_has_reliable_f128)`, true if `f128` basic arithmetic works.
    pub has_reliable_f128: bool,
    /// Option for `cfg(target_has_reliable_f128_math)`, true if `f128` math calls work.
    pub has_reliable_f128_math: bool,
}

#[derive(Encodable, Decodable)]
pub struct CompiledModules {
    pub modules: Vec<CompiledModule>,
    pub allocator_module: Option<CompiledModule>,
}

pub enum CodegenError {
    WrongFileType,
    EmptyVersionNumber,
    EncodingVersionMismatch { version_array: String, rlink_version: u32 },
    RustcVersionMismatch { rustc_version: String },
    CorruptFile,
}

pub fn provide(providers: &mut Providers) {
    crate::back::symbol_export::provide(providers);
    crate::base::provide(&mut providers.queries);
    crate::target_features::provide(&mut providers.queries);
    crate::codegen_attrs::provide(&mut providers.queries);
    providers.queries.global_backend_features = |_tcx: TyCtxt<'_>, ()| vec![];
}

/// Checks if the given filename ends with the `.rcgu.o` extension that `rustc`
/// uses for the object files it generates.
pub fn looks_like_rust_object_file(filename: &str) -> bool {
    let path = Path::new(filename);
    let ext = path.extension().and_then(|s| s.to_str());
    if ext != Some(OutputType::Object.extension()) {
        // The file name does not end with ".o", so it can't be an object file.
        return false;
    }

    // Strip the ".o" at the end
    let ext2 = path.file_stem().and_then(|s| Path::new(s).extension()).and_then(|s| s.to_str());

    // Check if the "inner" extension
    ext2 == Some(RUST_CGU_EXT)
}

const RLINK_VERSION: u32 = 1;
const RLINK_MAGIC: &[u8] = b"rustlink";

impl CompiledModules {
    pub fn serialize_rlink(
        sess: &Session,
        rlink_file: &Path,
        compiled_modules: &CompiledModules,
        crate_info: &CrateInfo,
        metadata: &EncodedMetadata,
        outputs: &OutputFilenames,
    ) -> Result<usize, io::Error> {
        let mut encoder = FileEncoder::new(rlink_file)?;
        encoder.emit_raw_bytes(RLINK_MAGIC);
        // `emit_raw_bytes` is used to make sure that the version representation does not depend on
        // Encoder's inner representation of `u32`.
        encoder.emit_raw_bytes(&RLINK_VERSION.to_be_bytes());
        encoder.emit_str(sess.cfg_version);
        Encodable::encode(compiled_modules, &mut encoder);
        Encodable::encode(crate_info, &mut encoder);
        Encodable::encode(metadata, &mut encoder);
        Encodable::encode(outputs, &mut encoder);
        encoder.finish().map_err(|(_path, err)| err)
    }

    pub fn deserialize_rlink(
        sess: &Session,
        data: Vec<u8>,
    ) -> Result<(Self, CrateInfo, EncodedMetadata, OutputFilenames), CodegenError> {
        // The Decodable machinery is not used here because it panics if the input data is invalid
        // and because its internal representation may change.
        if !data.starts_with(RLINK_MAGIC) {
            return Err(CodegenError::WrongFileType);
        }
        let data = &data[RLINK_MAGIC.len()..];
        if data.len() < 4 {
            return Err(CodegenError::EmptyVersionNumber);
        }

        let mut version_array: [u8; 4] = Default::default();
        version_array.copy_from_slice(&data[..4]);
        if u32::from_be_bytes(version_array) != RLINK_VERSION {
            return Err(CodegenError::EncodingVersionMismatch {
                version_array: String::from_utf8_lossy(&version_array).to_string(),
                rlink_version: RLINK_VERSION,
            });
        }

        let Ok(mut decoder) = MemDecoder::new(&data[4..], 0) else {
            return Err(CodegenError::CorruptFile);
        };
        let rustc_version = decoder.read_str();
        if rustc_version != sess.cfg_version {
            return Err(CodegenError::RustcVersionMismatch {
                rustc_version: rustc_version.to_string(),
            });
        }

        let compiled_modules = CompiledModules::decode(&mut decoder);
        let crate_info = CrateInfo::decode(&mut decoder);
        let metadata = EncodedMetadata::decode(&mut decoder);
        let outputs = OutputFilenames::decode(&mut decoder);
        Ok((compiled_modules, crate_info, metadata, outputs))
    }
}

/// A list of lint levels used in codegen.
///
/// When using `-Z link-only`, we don't have access to the tcx and must work
/// solely from the `.rlink` file. `Lint`s are defined too early to be encodeable.
/// Instead, encode exactly the information we need.
#[derive(Copy, Clone, Debug, Encodable, Decodable)]
pub struct CodegenLintLevels {
    linker_messages: LevelAndSource,
    linker_info: LevelAndSource,
}

impl CodegenLintLevels {
    pub fn from_tcx(tcx: TyCtxt<'_>) -> Self {
        Self {
            linker_messages: tcx.lint_level_at_node(LINKER_MESSAGES, CRATE_HIR_ID),
            linker_info: tcx.lint_level_at_node(LINKER_INFO, CRATE_HIR_ID),
        }
    }
}
