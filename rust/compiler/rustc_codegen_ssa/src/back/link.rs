use std::path::Path;
use std::{fs, io};

use rustc_errors::DiagCtxtHandle;
use rustc_metadata::EncodedMetadata;
use rustc_middle::bug;
use rustc_session::Session;
use rustc_session::config::OutputFilenames;

use super::archive::ArchiveBuilderBuilder;
use crate::{CompiledModules, CrateInfo};

pub fn ensure_removed(dcx: DiagCtxtHandle<'_>, path: &Path) {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != io::ErrorKind::NotFound {
            dcx.err(format!("failed to remove {}: {}", path.display(), e));
        }
    }
}

/// Performs the linkage portion of the compilation phase. This will generate all
/// of the requested outputs for this compilation session.
pub fn link_binary(
    _sess: &Session,
    _archive_builder_builder: &dyn ArchiveBuilderBuilder,
    _compiled_modules: CompiledModules,
    _crate_info: CrateInfo,
    _metadata: EncodedMetadata,
    _outputs: &OutputFilenames,
    _codegen_backend: &'static str,
) {
    bug!("link_binary called");
}
