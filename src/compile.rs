mod app;
pub(crate) mod compile_server;
mod eaf_builder;
mod translate;

use std::cell::Cell;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use rustc_ast::Crate;
use rustc_ast_pretty::pprust;
use rustc_data_structures::fx::{FxHashSet, FxIndexMap, FxIndexSet};
use rustc_errors::DiagCtxtHandle;
use rustc_feature::UnstableFeatures;
use rustc_hir_pretty as pprust_hir;
use rustc_interface::util::DummyCodegenBackend;
use rustc_interface::{interface, passes};
use rustc_middle::bug;
use rustc_middle::dep_graph::{DepKind, DepNode, WorkProduct, WorkProductId};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use rustc_parse::lexer::StripTokens;
use rustc_parse::{new_parser_from_source_str, unwrap_or_emit_fatal};
use rustc_session::config::{Externs, PanicStrategy};
use rustc_session::search_paths::{PathKind, SearchPath};
use rustc_session::{Session, config, lint};
use rustc_span::def_id::LOCAL_CRATE;
use rustc_span::edition::Edition;
use rustc_span::{ErrorGuaranteed, FileName, RealFileName, RemapPathScopeComponents};

use crate::Stage;
use crate::compile::compile_server::SyncServer;
use crate::package::PackageInfo;

static INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

const PRELUDE: &str = r#"
#![allow(internal_features)]
#![feature(no_core, rustc_attrs, decl_macro, prelude_import)]
#![no_std]
#![no_core]

#[allow(unused_imports)]
#[macro_use]
extern pkg core;

#[allow(unused_imports)]
#[prelude_import]
use core::prelude::*;
"#;

struct NoAnn;
impl pprust::PpAnn for NoAnn {}

struct TypedAnn<'tcx> {
    tcx: TyCtxt<'tcx>,
    maybe_typeck_results: Cell<Option<&'tcx TypeckResults<'tcx>>>,
}

impl pprust_hir::PpAnn for TypedAnn<'_> {
    fn nested(&self, state: &mut pprust_hir::State<'_>, nested: pprust_hir::Nested) {
        let old = self.maybe_typeck_results.get();
        if let pprust_hir::Nested::Body(id) = nested {
            self.maybe_typeck_results.set(Some(self.tcx.typeck_body(id)));
        }
        self.tcx.nested(state, nested);
        self.maybe_typeck_results.set(old);
    }

    fn pre(&self, state: &mut pprust_hir::State<'_>, node: pprust_hir::AnnNode<'_>) {
        if let pprust_hir::AnnNode::Expr(_) = node {
            state.popen();
        }
    }

    fn post(&self, state: &mut pprust_hir::State<'_>, node: pprust_hir::AnnNode<'_>) {
        if let pprust_hir::AnnNode::Expr(expr) = node {
            let typeck_results = self.maybe_typeck_results.get().or_else(|| {
                self.tcx
                    .hir_maybe_body_owned_by(expr.hir_id.owner.def_id)
                    .map(|body_id| self.tcx.typeck_body(body_id.id()))
            });

            if let Some(typeck_results) = typeck_results {
                state.s.space();
                state.s.word("as");
                state.s.space();
                state.s.word(typeck_results.expr_ty(expr).to_string());
            }

            state.pclose();
        }
    }
}

macro_rules! consume_stage {
    ($stages_left:expr, $stage:expr, $body:block) => {
        if $stages_left.remove(&$stage) {
            let result = $body;
            if $stages_left.is_empty() {
                return result;
            }
        }
    };
}

#[derive(Clone)]
pub(crate) struct CompileOptions {
    pub base_dir: PathBuf,
    pub remap_path_prefix: Vec<(PathBuf, PathBuf)>,
    pub otp_version: String,
    pub out_dir: PathBuf,
    pub incremental_dir: PathBuf,
    pub lib_dirs: Vec<PathBuf>,
    pub package: PackageInfo,
    pub externs: Externs,
    pub allowed_lints: Vec<String>,
    pub error_format: config::ErrorOutputType,
}

impl CompileOptions {
    fn code_path(&self) -> FxIndexSet<PathBuf> {
        let rmeta_files = self.externs.iter().filter_map(|(_, entry)| entry.files()).flatten();
        let rmeta_dirs = rmeta_files
            .filter_map(|file| file.original().parent())
            .map(|dir| if dir.as_os_str().is_empty() { Path::new(".") } else { dir });
        rmeta_dirs.chain(self.lib_dirs.iter().map(PathBuf::as_path)).map(Path::to_owned).collect()
    }
}

pub(crate) fn compile(
    stages: &[Stage],
    filename: &Path,
    src: &str,
    opts: &CompileOptions,
    server: &Arc<SyncServer>,
) {
    let CompileOptions { out_dir, package: pkg, .. } = opts;
    let filename_stem = filename
        .file_stem()
        .expect("filename to have a stem")
        .to_str()
        .expect("filename to be valid UTF-8");

    let mut stages_left = FxHashSet::from_iter(stages.iter());
    if stages_left.is_empty() {
        return;
    }

    std::fs::create_dir_all(out_dir).unwrap_or_else(|e| {
        panic!("failed to create output directory: {e}");
    });

    consume_stage!(stages_left, Stage::Tokens, {
        let tokens = render_tokens(src);
        write_file(out_dir, filename_stem, "tokens", tokens.as_bytes()).unwrap_or_else(|e| {
            panic!("failed to write tokens file: {e}");
        });
    });

    consume_stage!(stages_left, Stage::Ast, {
        let config = make_config(filename, opts.clone(), server);
        interface::run_compiler(config, |compiler| {
            let krate = parse_with_prelude(&compiler.sess, "");

            let ast = pprust::print_crate(
                compiler.sess.source_map(),
                &krate,
                compiler.sess.io.input.file_name(&compiler.sess),
                src.to_string(),
                &NoAnn,
                false,
                compiler.sess.edition(),
                &compiler.sess.psess.attr_id_generator,
            );

            write_file(out_dir, filename_stem, "ast", ast.as_bytes()).unwrap_or_else(|e| {
                panic!("failed to write ast file: {e}");
            });
        });
    });

    let config = make_config(filename, opts.clone(), server);
    let prelude = if pkg.is_core() { "" } else { PRELUDE };

    interface::run_compiler(config, |compiler| {
        let krate = parse_with_prelude(&compiler.sess, prelude);
        let input_src =
            if prelude.is_empty() { src.to_string() } else { format!("{prelude}\n{src}") };

        let crate_hash = passes::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            // Only a build that emits beams keeps its session for the next build.
            // Any other session is left unfinished and deleted.
            let work_products =
                run_stages(&mut stages_left, filename_stem, input_src, opts, server, tcx)?;

            tcx.sess.dcx().has_errors_or_delayed_bugs().is_none().then(|| {
                rustc_incremental::save_work_product_index(tcx.sess, &tcx.dep_graph, work_products);
                tcx.crate_hash(LOCAL_CRATE)
            })
        });

        if let Some(crate_hash) = crate_hash {
            rustc_incremental::finalize_session_directory(&compiler.sess, Some(crate_hash));
        }
    });
}

struct StagedOutDir<'a> {
    dcx: DiagCtxtHandle<'a>,
    out_dir: PathBuf,
    staging_dir: tempfile::TempDir,
    emitted_filenames: FxIndexSet<PathBuf>,
}

impl<'a> StagedOutDir<'a> {
    fn new(dcx: DiagCtxtHandle<'a>, out_dir: PathBuf) -> io::Result<Self> {
        let staging_dir =
            tempfile::Builder::new().prefix(".patina-").suffix(".tmp").tempdir_in(&out_dir)?;

        Ok(Self { dcx, out_dir, staging_dir, emitted_filenames: FxIndexSet::default() })
    }

    // Outputs are written or linked to `staging_dir` first, on the same filesystem  as `out_dir`,
    // then renamed into place to get atomic changes in the output dir.
    fn try_with_atomic_file(
        &mut self,
        file_name: &str,
        write: impl FnOnce(&Path) -> io::Result<()>,
    ) -> io::Result<PathBuf> {
        let temp = self.staging_dir.path().join(file_name);
        write(&temp)?;

        let out = self.out_dir.join(file_name);
        std::fs::rename(&temp, &out)?;

        self.emitted_filenames.insert(file_name.into());
        Ok(out)
    }

    fn with_atomic_file(
        &mut self,
        file_name: &str,
        write: impl FnOnce(&Path) -> io::Result<()>,
    ) -> PathBuf {
        self.try_with_atomic_file(file_name, write).unwrap_or_else(|err| {
            let out = self.out_dir.join(file_name);
            self.dcx.fatal(format!("failed to write {}: {err}", out.display()))
        })
    }

    fn emitted_filenames(&self) -> &FxIndexSet<PathBuf> {
        &self.emitted_filenames
    }
}

fn run_stages(
    stages_left: &mut FxHashSet<&Stage>,
    filename_stem: &str,
    input_src: String,
    opts: &CompileOptions,
    server: &Arc<SyncServer>,
    tcx: TyCtxt<'_>,
) -> Option<FxIndexMap<WorkProductId, WorkProduct>> {
    let dcx = tcx.sess.dcx();
    let CompileOptions { out_dir, package: pkg, .. } = opts;

    let mut staged_out_dir = StagedOutDir::new(dcx, out_dir.to_owned()).unwrap_or_else(|e| {
        dcx.fatal(format!("failed to create a staging directory in {}: {e}", out_dir.display()))
    });

    // Expansion runs proc macros, so this has to happen before it.
    let code_path: Vec<_> = opts.code_path().into_iter().collect();
    server.with(dcx, |server| server.extend_code_path(dcx, code_path));

    consume_stage!(stages_left, Stage::Expanded, {
        let expanded = pprust::print_crate(
            tcx.sess.source_map(),
            &tcx.resolver_for_lowering().borrow().1,
            tcx.sess.io.input.file_name(tcx.sess),
            input_src.clone(),
            &NoAnn,
            true,
            tcx.sess.edition(),
            &tcx.sess.psess.attr_id_generator,
        );

        write_file(out_dir, filename_stem, "expanded", expanded.as_bytes()).unwrap_or_else(|e| {
            dcx.fatal(format!("failed to write expanded file: {e}"));
        });

        None
    });

    consume_stage!(stages_left, Stage::Typed, {
        tcx.hir_body_owners().for_each(|def_id| {
            tcx.ensure_ok().typeck(def_id);
            if !tcx.is_typeck_child(def_id.to_def_id()) {
                tcx.ensure_ok().check_unsafety(def_id);
                tcx.ensure_ok().check_transmutes(def_id);
            }
        });

        let typed = pprust_hir::print_crate(
            tcx.sess.source_map(),
            tcx.hir_root_module(),
            tcx.sess.io.input.file_name(tcx.sess),
            input_src,
            &|id| tcx.hir_attrs(id),
            &TypedAnn { tcx, maybe_typeck_results: Cell::new(None) },
        );

        write_file(out_dir, filename_stem, "typed", typed.as_bytes()).unwrap_or_else(|e| {
            dcx.fatal(format!("failed to write typed file: {e}"));
        });

        None
    });

    tcx.ensure_ok().analysis(());
    if let Some(guar) = tcx.sess.dcx().has_errors_or_delayed_bugs() {
        guar.raise_fatal();
    }

    let mut metadata_work_product = None;
    consume_stage!(stages_left, Stage::Meta, {
        let metadata_file_name = format!("lib{}.rmeta", pkg.name);
        let metadata_path = staged_out_dir.with_atomic_file(&metadata_file_name, |temp| {
            rustc_metadata::encode_metadata(tcx, temp, None);
            Ok(())
        });

        // The work product's name is hardcoded in `rustc_metadata::encode_metadata`.
        let wp_name = "metadata";

        let known_links: Vec<_> = tcx
            .dep_graph
            .previous_work_product(&WorkProductId::from_cgu_name(wp_name))
            .filter(|_| tcx.dep_graph.is_green(&tcx.metadata_dep_node()))
            .and_then(|wp| {
                let saved_file = wp.saved_files.get("rmeta")?;
                Some(rustc_incremental::in_incr_comp_dir_sess(tcx.sess, saved_file))
            })
            .into_iter()
            .collect();

        metadata_work_product = rustc_incremental::copy_cgu_workproduct_to_incr_comp_cache_dir(
            tcx.sess,
            wp_name,
            &[("rmeta", &metadata_path)],
            &known_links,
        );

        None
    });

    consume_stage!(stages_left, Stage::App, {
        let Some(version) = &pkg.version else {
            dcx.fatal("emitting an `.app` requires a package version; set one with --pkg-version");
        };
        let term = app::app_term(tcx, version, &pkg.description).unwrap_or_else(|e| {
            dcx.fatal(format!("failed to build the application specification: {e}"));
        });
        let mut rendered = server.with(dcx, |server| server.render_term(dcx, &term));
        rendered.push_str(".\n");
        let app_file_name = format!("{}.app", pkg.otp_app_name);
        staged_out_dir.with_atomic_file(&app_file_name, |temp| std::fs::write(temp, rendered));

        None
    });

    let serialized_modules = tcx
        .erl_local_modules(())
        .keys()
        .map(|&module| (module, tcx.erl_module_name(module), tcx.erl_module_eaf(module)))
        .collect::<Vec<_>>();

    consume_stage!(stages_left, Stage::Eaf, {
        for (_, mod_name, bytes) in &serialized_modules {
            write_file(out_dir, mod_name.as_str(), "eaf", bytes).unwrap_or_else(|e| {
                dcx.fatal(format!("failed to write eaf file: {e}"));
            });
        }

        None
    });

    consume_stage!(stages_left, Stage::Abstr, {
        server.with(dcx, |server| {
            for (_, mod_name, bytes) in &serialized_modules {
                let mut rendered = server.render_term(dcx, bytes);
                if !rendered.ends_with('\n') {
                    rendered.push('\n');
                }
                write_file(out_dir, mod_name.as_str(), "abstr", rendered.as_bytes())
                    .unwrap_or_else(|e| {
                        dcx.fatal(format!("failed to write abstr file: {e}"));
                    });
            }
        });

        None
    });

    consume_stage!(stages_left, Stage::Erl, {
        server.with(dcx, |server| {
            for (_, mod_name, bytes) in &serialized_modules {
                let rendered = server.render_erl(dcx, bytes);
                write_file(out_dir, mod_name.as_str(), "erl", rendered.as_bytes()).unwrap_or_else(
                    |e| {
                        dcx.fatal(format!("failed to write erl file: {e}"));
                    },
                );
            }
        });

        None
    });

    consume_stage!(stages_left, Stage::Core0, {
        server.with(dcx, |server| {
            for (_, mod_name, bytes) in &serialized_modules {
                let rendered = server.render_core0(dcx, bytes).unwrap_or_else(|e| {
                    dcx.fatal(format!("erl backend failed: {}\n{}", e.errors, e.warnings));
                });
                write_file(out_dir, mod_name.as_str(), "core0", rendered.as_bytes())
                    .unwrap_or_else(|e| {
                        dcx.fatal(format!("failed to write core0 file: {e}"));
                    });
            }
        });

        None
    });

    consume_stage!(stages_left, Stage::Core, {
        server.with(dcx, |server| {
            for (_, mod_name, bytes) in &serialized_modules {
                let rendered = server.render_core(dcx, bytes).unwrap_or_else(|e| {
                    dcx.fatal(format!("erl backend failed: {}\n{}", e.errors, e.warnings));
                });
                write_file(out_dir, mod_name.as_str(), "core", rendered.as_bytes()).unwrap_or_else(
                    |e| {
                        dcx.fatal(format!("failed to write core file: {e}"));
                    },
                );
            }
        });

        None
    });

    consume_stage!(stages_left, Stage::Beam, {
        let mut work_products = FxIndexMap::default();
        let mut beam_paths = Vec::new();
        for &(module, mod_name, eaf) in &serialized_modules {
            let mod_name = mod_name.as_str();
            let beam_file_name = format!("{mod_name}.beam");
            let id = WorkProductId::from_cgu_name(&beam_file_name);

            let (beam_path, known_links) =
                match try_use_cached_beam(tcx, &mut staged_out_dir, &module, id, &beam_file_name) {
                    Some((beam_path, cached)) => (beam_path, &[cached][..]),
                    None => {
                        let compile = server
                            .with(dcx, |server| server.compile(dcx, eaf))
                            .unwrap_or_else(|e| {
                                dcx.fatal(format!(
                                    "erl backend failed: {}\n{}",
                                    e.errors, e.warnings
                                ));
                            });

                        if !compile.warnings.is_empty() && compile.warnings != "[]" {
                            dcx.warn(compile.warnings);
                        }

                        let beam_path = staged_out_dir.with_atomic_file(&beam_file_name, |temp| {
                            std::fs::write(temp, compile.beam)
                        });
                        (beam_path, &[][..])
                    }
                };

            let (_, wp) = rustc_incremental::copy_cgu_workproduct_to_incr_comp_cache_dir(
                tcx.sess,
                &beam_file_name,
                &[("beam", &beam_path)],
                known_links,
            )
            .unwrap_or_else(|| bug!("incremental compilation is not on"));

            work_products.insert(id, wp);
            beam_paths.push(beam_path);
        }

        server.with(dcx, |server| server.load_modules(dcx, beam_paths));

        clean_up_and_update_files_list(
            &opts.incremental_dir,
            out_dir,
            staged_out_dir.emitted_filenames(),
        )
        .unwrap_or_else(|e| dcx.fatal(format!("failed to delete outdated outputs: {e}")));

        work_products.extend(metadata_work_product);
        Some(work_products)
    });

    None
}

fn try_use_cached_beam(
    tcx: TyCtxt<'_>,
    staged_out_dir: &mut StagedOutDir<'_>,
    module: &rustc_middle::erl::ErlModuleId,
    id: WorkProductId,
    file_name: &str,
) -> Option<(PathBuf, PathBuf)> {
    let eaf_node = DepNode::construct(tcx, DepKind::erl_module_eaf, &module);
    if !tcx.dep_graph.is_green(&eaf_node) {
        return None;
    };

    let wp = tcx.dep_graph.previous_work_product(&id)?;
    let saved_file = wp.saved_files.get("beam")?;
    let cached = rustc_incremental::in_incr_comp_dir_sess(tcx.sess, saved_file);

    let beam_path = match staged_out_dir.try_with_atomic_file(file_name, |temp| {
        rustc_fs_util::link_or_copy(&cached, temp)?;
        Ok(())
    }) {
        Ok(beam_path) => beam_path,

        Err(err) => {
            let out_path = staged_out_dir.out_dir.join(file_name);
            tcx.sess.dcx().warn(format!(
                "failed to restore {} from cache: {err}; recompiling",
                out_path.display()
            ));
            return None;
        }
    };

    Some((beam_path, cached))
}

pub(crate) fn files_list_path(incremental_dir: &Path) -> PathBuf {
    incremental_dir.join("files")
}

/// Deletes build artifacts that were present in the previous build but are not present now.
/// Only emitted "categories" are cleaned up, e.g. if the previous build remitted `.app`,
/// but this build doesn't, the `.app` is grandfathered-in to the current build.
/// Maintains a `files` file in the `incremental_dir`, containing the names
/// of all tracked artifacts.
fn clean_up_and_update_files_list(
    incremental_dir: &Path,
    out_dir: &Path,
    files: &FxIndexSet<PathBuf>,
) -> io::Result<()> {
    let emitted_kinds: FxHashSet<&OsStr> = files.iter().flat_map(|name| name.extension()).collect();

    let list_path = files_list_path(incremental_dir);
    let previous_files = match std::fs::read_to_string(&list_path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        other => other?,
    };

    let mut kept = Vec::new();
    for file in previous_files.lines().map(Path::new).filter(|&file| !files.contains(file)) {
        let path = out_dir.join(file);
        if file.extension().is_some_and(|kind| emitted_kinds.contains(kind)) {
            match std::fs::remove_file(&path) {
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                other => other?,
            }
        } else if path.exists() {
            kept.push(file);
        }
    }

    let names = files.iter().map(PathBuf::as_path).chain(kept).map(Path::as_os_str);
    let contents: Vec<u8> =
        names.map(OsStr::as_encoded_bytes).intersperse(b"\n").flatten().copied().collect();
    std::fs::write(&list_path, contents)
}

fn make_config(
    filename: &Path,
    opts: CompileOptions,
    server: &Arc<SyncServer>,
) -> interface::Config {
    let CompileOptions {
        base_dir,
        remap_path_prefix,
        incremental_dir,
        lib_dirs,
        package: pkg,
        externs,
        allowed_lints,
        error_format,
        ..
    } = opts;

    let mut options = config::Options {
        edition: Edition::EditionFuture,
        unstable_features: UnstableFeatures::Allow,
        unstable_opts: config::UnstableOptions {
            ui_testing: true,
            no_steal_thir: true,
            cache_proc_macros: true,
            ..Default::default()
        },
        error_format,
        crate_types: vec![config::CrateType::Rlib],
        cg: config::CodegenOptions { panic: Some(PanicStrategy::Abort), ..Default::default() },
        crate_name: Some(pkg.name.to_string()),
        otp_app_name: Some(pkg.otp_app_name),
        otp_version: Some(opts.otp_version),
        externs,
        lint_opts: allowed_lints.into_iter().map(|lint| (lint, lint::Level::Allow)).collect(),
        search_paths: lib_dirs
            .iter()
            .map(|dir| SearchPath::new(PathKind::All, dir.to_owned()))
            .collect(),
        remap_path_prefix,
        remap_path_scope: RemapPathScopeComponents::all(),
        incremental: Some(incremental_dir),
        ..Default::default()
    };

    options.working_dir =
        options.file_path_mapping().to_real_filename(&RealFileName::empty(), base_dir);

    interface::Config {
        crate_cfg: vec![],
        crate_check_cfg: vec![],
        input: config::Input::File(filename.to_owned()),
        output_dir: None,
        output_file: None,
        ice_file: None,
        file_loader: None,
        lint_caps: Default::default(),
        psess_created: None,
        track_state: None,
        register_lints: None,
        override_queries: Some(|_, providers| {
            providers.queries.erl_module_eaf = translate::erl_module_eaf;
        }),
        extra_symbols: vec![],
        make_codegen_backend: Some(Box::new(|_| {
            Box::new(DummyCodegenBackend { target_config_override: None })
        })),
        beam_macro_runner: server.clone(),
        using_internal_features: &INTERNAL_FEATURES,
        opts: options,
    }
}

fn render_tokens(src: &str) -> String {
    rustc_lexer::tokenize(src, rustc_lexer::FrontmatterAllowed::No)
        .map(|token| format!("{:?}", token.kind))
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_with_prelude(sess: &Session, prelude: &str) -> Crate {
    if prelude.is_empty() {
        return passes::parse(sess);
    }

    let mut parser = unwrap_or_emit_fatal(new_parser_from_source_str(
        &sess.psess,
        FileName::Custom("patina-prelude".to_string()),
        prelude.to_string(),
        StripTokens::Nothing,
    ));
    let prelude_krate = parser.parse_crate_mod().unwrap_or_else(|err| {
        let guar: ErrorGuaranteed = err.emit();
        guar.raise_fatal();
    });

    let mut krate = passes::parse(sess);

    let mut user_attrs = std::mem::take(&mut krate.attrs);
    krate.attrs = prelude_krate.attrs;
    krate.attrs.append(&mut user_attrs);

    let mut user_items = std::mem::take(&mut krate.items);
    krate.items = prelude_krate.items;
    krate.items.append(&mut user_items);

    krate
}

fn write_file(
    out_dir: &Path,
    filename_stem: &str,
    extension: &str,
    content: &[u8],
) -> Result<PathBuf, String> {
    let filepath = out_dir.join(filename_stem).with_added_extension(extension);

    let mut file = File::create(&filepath)
        .map_err(|e| format!("failed to create file {}: {e}", filepath.display()))?;

    file.write_all(content)
        .map_err(|e| format!("failed to write file {}: {e}", filepath.display()))?;

    Ok(filepath)
}
