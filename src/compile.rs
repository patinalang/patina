pub(crate) mod compile_server;
mod eaf_builder;
mod impl_collector;
mod translate;

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use rustc_arena::DroplessArena;
use rustc_ast::Crate;
use rustc_ast_pretty::pprust;
use rustc_data_structures::fx::{FxHashSet, FxIndexSet};
use rustc_feature::UnstableFeatures;
use rustc_hir_pretty as pprust_hir;
use rustc_interface::util::DummyCodegenBackend;
use rustc_interface::{interface, passes};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use rustc_parse::lexer::StripTokens;
use rustc_parse::{new_parser_from_source_str, unwrap_or_emit_fatal};
use rustc_session::config::{Externs, PanicStrategy};
use rustc_session::search_paths::{PathKind, SearchPath};
use rustc_session::{Session, config};
use rustc_span::edition::Edition;
use rustc_span::source_map::FilePathMapping;
use rustc_span::{ErrorGuaranteed, FileName, RealFileName};

use crate::compile::compile_server::SyncServer;
use crate::package::{CORE_PACKAGE_NAME, PROC_MACRO_PACKAGE_NAME, PackageDependency};
use crate::{Stage, erlang};

static INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

const PRELUDE: &str = r#"
#![allow(internal_features)]
#![feature(no_core, rustc_attrs, decl_macro)]
#![no_std]
#![no_core]

#[allow(unused_imports)]
#[macro_use]
extern crate Core;

#[allow(unused_imports)]
use Core::*;

#[allow(unused)]
extern mod io;

#[allow(unused)]
extern mod patina_fmt;
"#;

const CORE_SRC: &str = include_str!("core.pt");
const PROC_MACRO_SRC: &str = include_str!("proc_macro.pt");

fn ensure_core(out_dir: &Path, server: &Arc<SyncServer>) -> PathBuf {
    let rmeta_path = out_dir.join("libCore.rmeta");
    let beam_path = out_dir.join("Elixir.Patina.Core.beam");

    if rmeta_path.is_file() && beam_path.is_file() {
        return rmeta_path;
    }

    compile(
        &[Stage::Meta, Stage::Beam],
        Path::new("core.pt"),
        CORE_SRC,
        out_dir,
        CORE_PACKAGE_NAME,
        &[],
        server,
    );

    rmeta_path
}

fn ensure_proc_macro(out_dir: &Path, server: &Arc<SyncServer>) -> PathBuf {
    let rmeta_path = out_dir.join("libProcMacro.rmeta");
    let beam_path = out_dir.join("Elixir.Patina.ProcMacro.beam");

    if rmeta_path.is_file() && beam_path.is_file() {
        return rmeta_path;
    }

    compile(
        &[Stage::Meta, Stage::Beam],
        Path::new("proc_macro.pt"),
        PROC_MACRO_SRC,
        out_dir,
        PROC_MACRO_PACKAGE_NAME,
        &[],
        server,
    );

    rmeta_path
}

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
            $body
            if $stages_left.is_empty() {
                return;
            }
        }
    };
}

pub(crate) fn compile(
    stages: &[Stage],
    filename: &Path,
    src: &str,
    out_dir: &Path,
    pkg_name: &str,
    externs: &[PackageDependency],
    server: &Arc<SyncServer>,
) {
    let filename_stem = filename
        .file_stem()
        .expect("filename to have a stem")
        .to_str()
        .expect("filename to be valid UTF-8");

    let filename =
        FileName::Real(FilePathMapping::empty().to_real_filename(&RealFileName::empty(), filename));

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
        let config = make_config(filename.clone(), src, pkg_name, externs, out_dir, server);
        interface::run_compiler(config, |compiler| {
            let krate = parse_with_prelude(&compiler.sess, "");

            let ast = pprust::print_crate(
                compiler.sess.source_map(),
                &krate,
                filename.clone(),
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

    let config = make_config(filename.clone(), src, pkg_name, externs, out_dir, server);
    let prelude = if pkg_name == CORE_PACKAGE_NAME { "" } else { PRELUDE };

    interface::run_compiler(config, |compiler| {
        let krate = parse_with_prelude(&compiler.sess, prelude);
        let input_src =
            if prelude.is_empty() { src.to_string() } else { format!("{prelude}\n{src}") };

        passes::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            let dcx = tcx.sess.dcx();

            // Reading the metadata interns symbols, so this has to happen inside
            // the compiler session, where `rustc_span`'s globals are set.
            let mut macro_paths = FxIndexSet::from_iter([out_dir.to_owned()]);
            for dep in externs {
                if let Some(path) = dep.path.as_ref()
                    && rustc_metadata::locator::exports_proc_macros(path)
                    && let Some(parent) = path.parent()
                    && !macro_paths.contains(parent)
                {
                    macro_paths.insert(parent.to_owned());
                }
            }
            server.with(dcx, |server| server.extend_code_path(dcx, macro_paths.into_iter()));

            consume_stage!(stages_left, Stage::Expanded, {
                let expanded = pprust::print_crate(
                    tcx.sess.source_map(),
                    &tcx.resolver_for_lowering().borrow().1,
                    filename.clone(),
                    input_src.clone(),
                    &NoAnn,
                    true,
                    tcx.sess.edition(),
                    &tcx.sess.psess.attr_id_generator,
                );

                write_file(out_dir, filename_stem, "expanded", expanded.as_bytes()).unwrap_or_else(
                    |e| {
                        dcx.fatal(format!("failed to write expanded file: {e}"));
                    },
                );
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
                    filename.clone(),
                    input_src,
                    &|id| tcx.hir_attrs(id),
                    &TypedAnn { tcx, maybe_typeck_results: Cell::new(None) },
                );

                write_file(out_dir, filename_stem, "typed", typed.as_bytes()).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write typed file: {e}"));
                });
            });

            tcx.ensure_ok().analysis(());
            if let Some(guar) = tcx.sess.dcx().has_errors_or_delayed_bugs() {
                guar.raise_fatal();
            }

            consume_stage!(stages_left, Stage::Meta, {
                let metadata_path = out_dir.join(format!("lib{}.rmeta", pkg_name));
                rustc_metadata::encode_metadata(tcx, &metadata_path, None);
            });

            let arena = DroplessArena::default();
            let translator = translate::Translator::new(tcx, &arena);
            let modules = translator.translate();

            let serialized_modules = modules
                .into_iter()
                .map(|(mod_name, module)| {
                    let bytes = erlang::to_bytes(&module).unwrap_or_else(|e| {
                        dcx.fatal(format!("failed to serialize module: {e}"));
                    });
                    (mod_name, bytes)
                })
                .collect::<Vec<_>>();

            consume_stage!(stages_left, Stage::Eaf, {
                for (mod_name, bytes) in &serialized_modules {
                    write_file(out_dir, mod_name.as_str(), "eaf", bytes).unwrap_or_else(|e| {
                        dcx.fatal(format!("failed to write eaf file: {e}"));
                    });
                }
            });

            consume_stage!(stages_left, Stage::Abstr, {
                server.with(dcx, |server| {
                    for (mod_name, bytes) in &serialized_modules {
                        let mut rendered = server.render_abstr(dcx, bytes);
                        if !rendered.ends_with('\n') {
                            rendered.push('\n');
                        }
                        write_file(out_dir, mod_name.as_str(), "abstr", rendered.as_bytes())
                            .unwrap_or_else(|e| {
                                dcx.fatal(format!("failed to write abstr file: {e}"));
                            });
                    }
                });
            });

            consume_stage!(stages_left, Stage::Erl, {
                server.with(dcx, |server| {
                    for (mod_name, bytes) in &serialized_modules {
                        let rendered = server.render_erl(dcx, bytes);
                        write_file(out_dir, mod_name.as_str(), "erl", rendered.as_bytes())
                            .unwrap_or_else(|e| {
                                dcx.fatal(format!("failed to write erl file: {e}"));
                            });
                    }
                });
            });

            consume_stage!(stages_left, Stage::Beam, {
                server.with(dcx, |server| {
                    for (mod_name, bytes) in &serialized_modules {
                        let compile = server.compile(dcx, bytes).unwrap_or_else(|e| {
                            dcx.fatal(format!("erl backend failed: {}\n{}", e.errors, e.warnings));
                        });

                        if !compile.warnings.is_empty() && compile.warnings != "[]" {
                            dcx.warn(compile.warnings);
                        }

                        write_file(out_dir, mod_name.as_str(), "beam", &compile.beam)
                            .unwrap_or_else(|e| {
                                dcx.fatal(format!("failed to write beam file: {e}"));
                            });
                    }
                });
            });
        });
    });
}

fn make_config(
    filename: FileName,
    src: &str,
    pkg_name: &str,
    deps: &[PackageDependency],
    out_dir: &Path,
    server: &Arc<SyncServer>,
) -> interface::Config {
    let has_explicit_core = deps.iter().any(|dep| dep.name == CORE_PACKAGE_NAME);
    let is_core = pkg_name == CORE_PACKAGE_NAME;

    let core_dep = (!has_explicit_core && !is_core).then(|| {
        let core_rmeta = ensure_core(out_dir, server);
        PackageDependency { name: CORE_PACKAGE_NAME.to_string(), path: Some(core_rmeta) }
    });

    let wants_proc_macro =
        deps.iter().any(|dep| dep.name == PROC_MACRO_PACKAGE_NAME && dep.path.is_none())
            && pkg_name != PROC_MACRO_PACKAGE_NAME;
    let proc_macro_dep = wants_proc_macro.then(|| PackageDependency {
        name: PROC_MACRO_PACKAGE_NAME.to_string(),
        path: Some(ensure_proc_macro(out_dir, server)),
    });

    let mut externs = BTreeMap::new();
    for dep in core_dep.iter().chain(deps.iter()).chain(proc_macro_dep.iter()) {
        externs.insert(dep.name.clone(), dep.into());
    }

    interface::Config {
        crate_cfg: vec![],
        crate_check_cfg: vec![],
        input: config::Input::Str { name: filename, input: src.to_string() },
        output_dir: None,
        output_file: None,
        ice_file: None,
        file_loader: None,
        lint_caps: Default::default(),
        psess_created: None,
        track_state: None,
        register_lints: None,
        override_queries: None,
        extra_symbols: vec![],
        make_codegen_backend: Some(Box::new(|_| {
            Box::new(DummyCodegenBackend { target_config_override: None })
        })),
        beam_macro_runner: server.clone(),
        using_internal_features: &INTERNAL_FEATURES,
        opts: config::Options {
            edition: Edition::EditionFuture,
            unstable_features: UnstableFeatures::Allow,
            unstable_opts: config::UnstableOptions {
                ui_testing: true,
                no_steal_thir: true,
                ..Default::default()
            },
            crate_types: vec![config::CrateType::Rlib],
            cg: config::CodegenOptions { panic: Some(PanicStrategy::Abort), ..Default::default() },
            crate_name: Some(pkg_name.to_string()),
            externs: Externs::new(externs),
            search_paths: vec![SearchPath::new(PathKind::All, out_dir.to_owned())],
            ..Default::default()
        },
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
) -> Result<(), String> {
    let filepath = out_dir.join(filename_stem).with_added_extension(extension);

    let mut file = File::create(&filepath)
        .map_err(|e| format!("failed to create file {}: {e}", filepath.display()))?;

    file.write_all(content)
        .map_err(|e| format!("failed to write file {}: {e}", filepath.display()))?;

    Ok(())
}
