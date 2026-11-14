mod app;
pub(crate) mod compile_server;
mod eaf_builder;
mod translate;

use std::cell::Cell;
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
use rustc_span::{ErrorGuaranteed, FileName, RealFileName, RemapPathScopeComponents, Symbol};

use crate::Stage;
use crate::compile::compile_server::SyncServer;
use crate::package::{PackageInfo, PackageName};

static INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

const PRELUDE: &str = r#"
#![allow(internal_features)]
#![feature(no_core, rustc_attrs, decl_macro, prelude_import)]
#![no_std]
#![no_core]

#[allow(unused_imports)]
#[macro_use]
extern crate core;

#[allow(unused_imports)]
#[prelude_import]
use core::prelude::*;

#[allow(unused)]
extern mod patina_fmt;
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

pub(crate) struct CompileOptions {
    pub base_dir: PathBuf,
    pub otp_version: String,
    pub out_dir: PathBuf,
    pub incremental_dir: PathBuf,
    pub lib_dirs: Vec<PathBuf>,
    pub package: PackageInfo,
    pub externs: Externs,
    pub allowed_lints: Vec<String>,
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
        let config = make_config(filename, opts, server);
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

    let config = make_config(filename, opts, server);
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

    // Outputs are written or linked here first, on the same filesystem  as `dir`,
    // then renamed into place to get atomic changes in the output dir.
    let staging_dir = tempfile::Builder::new()
        .prefix(".patina-")
        .suffix(".tmp")
        .tempdir_in(out_dir)
        .unwrap_or_else(|e| {
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
        let metadata_path = out_dir.join(format!("lib{}.rmeta", pkg.name));

        replace_file_via(&staging_dir, &metadata_path, |temp| {
            rustc_metadata::encode_metadata(tcx, temp, None);
            Ok(())
        })
        .unwrap_or_else(|e| {
            dcx.fatal(format!("failed to write metadata {}: {e}", metadata_path.display()));
        });

        metadata_work_product = rustc_incremental::copy_cgu_workproduct_to_incr_comp_cache_dir(
            tcx.sess,
            // "metadata" work-product name is hardcoded in `rustc_metadata::encode_metadata`
            "metadata",
            &[("rmeta", &metadata_path)],
            &[],
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
        write_file(out_dir, &pkg.otp_app_name, "app", rendered.as_bytes())
            .unwrap_or_else(|e| dcx.fatal(format!("failed to write app file: {e}")));

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
            let beam_path = out_dir.join(mod_name).with_added_extension("beam");
            let wp_name = format!("{mod_name}.beam");
            let id = WorkProductId::from_cgu_name(&wp_name);

            let known_links =
                match try_use_cached_beam(dcx, tcx, &staging_dir, &module, id, &beam_path) {
                    Some(cached) => &[cached][..],
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

                        replace_file_via(&staging_dir, &beam_path, |temp| {
                            std::fs::write(temp, compile.beam)
                        })
                        .unwrap_or_else(|e| {
                            dcx.fatal(format!(
                                "failed to write beam file {}: {e}",
                                beam_path.display()
                            ));
                        });

                        &[]
                    }
                };

            let (_, wp) = rustc_incremental::copy_cgu_workproduct_to_incr_comp_cache_dir(
                tcx.sess,
                &wp_name,
                &[("beam", &beam_path)],
                known_links,
            )
            .unwrap_or_else(|| bug!("incremental compilation is not on"));

            work_products.insert(id, wp);
            beam_paths.push(beam_path);
        }

        server.with(dcx, |server| server.load_modules(dcx, beam_paths));

        clean_up_beams_and_update_manifest(
            &opts.incremental_dir,
            &pkg.name,
            out_dir,
            tcx.beam_modules(LOCAL_CRATE),
        )
        .unwrap_or_else(|e| dcx.fatal(format!("failed to delete removed modules' beams: {e}")));

        work_products.extend(metadata_work_product);
        Some(work_products)
    });

    None
}

fn try_use_cached_beam(
    dcx: DiagCtxtHandle,
    tcx: TyCtxt<'_>,
    staging_dir: &tempfile::TempDir,
    module: &rustc_middle::erl::ErlModuleId,
    id: WorkProductId,
    out_path: &Path,
) -> Option<PathBuf> {
    let eaf_node = DepNode::construct(tcx, DepKind::erl_module_eaf, &module);
    if !tcx.dep_graph.is_green(&eaf_node) {
        return None;
    };

    let wp = tcx.dep_graph.previous_work_product(&id)?;
    let file_name = wp.saved_files.get("beam")?;
    let cached = rustc_incremental::in_incr_comp_dir_sess(tcx.sess, file_name);

    if let Err(err) = replace_file_via(staging_dir, out_path, |temp| {
        rustc_fs_util::link_or_copy(&cached, temp)?;
        Ok(())
    }) {
        dcx.warn(format!(
            "failed to restore {} from cache: {err}; recompiling",
            out_path.display()
        ));
        return None;
    }

    Some(cached)
}

fn clean_up_beams_and_update_manifest(
    incremental_dir: &Path,
    pkg_name: &PackageName,
    out_dir: &Path,
    modules: &FxIndexSet<Symbol>,
) -> io::Result<()> {
    let manifest_path = incremental_dir.join(format!("{pkg_name}.modules"));
    let modules: FxIndexSet<&str> = modules.iter().map(|symbol| symbol.as_str()).collect();

    let previous_modules = match std::fs::read_to_string(&manifest_path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        other => other,
    }?;

    for module in previous_modules.lines().filter(|&module| !modules.contains(module)) {
        match std::fs::remove_file(out_dir.join(module).with_added_extension("beam")) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            other => other,
        }?
    }

    let contents: String = modules.iter().copied().intersperse("\n").collect();
    std::fs::write(&manifest_path, contents)
}

fn replace_file_via(
    staging_dir: &tempfile::TempDir,
    target: &Path,
    write: impl FnOnce(&Path) -> io::Result<()>,
) -> io::Result<()> {
    let temp = staging_dir.path().join(target.file_name().expect("target path has a file name"));
    write(&temp)?;
    std::fs::rename(&temp, target)
}

fn make_config(
    filename: &Path,
    opts: &CompileOptions,
    server: &Arc<SyncServer>,
) -> interface::Config {
    let CompileOptions {
        base_dir,
        incremental_dir,
        lib_dirs,
        package: pkg,
        externs,
        allowed_lints,
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
        crate_types: vec![config::CrateType::Rlib],
        cg: config::CodegenOptions { panic: Some(PanicStrategy::Abort), ..Default::default() },
        crate_name: Some(pkg.name.to_string()),
        otp_app_name: Some(pkg.otp_app_name.clone()),
        otp_version: Some(opts.otp_version.clone()),
        externs: externs.clone(),
        lint_opts: allowed_lints.iter().map(|lint| (lint.clone(), lint::Level::Allow)).collect(),
        search_paths: lib_dirs
            .iter()
            .map(|dir| SearchPath::new(PathKind::All, dir.to_owned()))
            .collect(),
        remap_path_prefix: vec![(base_dir.clone(), PathBuf::new())],
        remap_path_scope: RemapPathScopeComponents::all(),
        incremental: Some(incremental_dir.clone()),
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
