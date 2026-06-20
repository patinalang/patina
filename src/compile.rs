mod compile_server;
mod eaf_builder;
mod impl_collector;
mod module_collector;
mod translate;

use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use rustc_ast::Crate;
use rustc_ast_pretty::pprust;
use rustc_feature::UnstableFeatures;
use rustc_hir_pretty as pprust_hir;
use rustc_interface::interface::{Compiler, Config};
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

use crate::package::{CORE_PACKAGE_NAME, PackageDependency};
use crate::{DumpStage, EmitStage, Stage};

static INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

const PRELUDE: &str = r#"
#![allow(internal_features)]
#![feature(no_core, rustc_attrs)]
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

#[allow(unused_macros)]
#[rustc_builtin_macro]
macro_rules! stringify {
    () => {};
}

#[allow(unused_macros)]
#[rustc_builtin_macro]
macro_rules! concat {
    () => {};
}

#[allow(unused_macros)]
#[rustc_builtin_macro]
macro_rules! format_args {
    () => {};
}

#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => {{ io::put_chars(format_args!($($arg)*)); }};
}

#[allow(unused_macros)]
macro_rules! println {
    () => {{ io::nl(); }};
    ($($arg:tt)*) => {{ io::put_chars([format_args!($($arg)*)]); io::nl(); }};
}
"#;

const CORE_SRC: &str = include_str!("core.pt");

fn ensure_core(out_dir: &Path) -> PathBuf {
    let rmeta_path = out_dir.join("libCore.rmeta");
    if rmeta_path.is_file() {
        return rmeta_path;
    }

    compile(
        Stage::Emit(crate::EmitStage::Beam),
        Path::new("core.pt"),
        CORE_SRC,
        out_dir,
        CORE_PACKAGE_NAME,
        &[],
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

pub(crate) fn compile(
    stage: Stage,
    filename: &Path,
    src: &str,
    out_dir: &Path,
    pkg_name: &str,
    externs: &[PackageDependency],
) {
    let filename =
        FileName::Real(FilePathMapping::empty().to_real_filename(&RealFileName::empty(), filename));

    let config = make_config(filename.clone(), src, pkg_name, externs, out_dir);
    let prelude = if pkg_name == CORE_PACKAGE_NAME { "" } else { PRELUDE };
    let input_src = if prelude.is_empty() { src.to_string() } else { format!("{prelude}\n{src}") };

    let maybe_render = match stage {
        Stage::Dump(DumpStage::Tokens) => Some(render_tokens(src)),
        Stage::Dump(DumpStage::Ast) => with_compiler(config, "", |compiler, krate| {
            Some(pprust::print_crate(
                compiler.sess.source_map(),
                &krate,
                filename,
                src.to_string(),
                &NoAnn,
                false,
                compiler.sess.edition(),
                &compiler.sess.psess.attr_id_generator,
            ))
        }),
        Stage::Dump(DumpStage::Expanded) => {
            with_compiler_global_ctxt(config, prelude, |_compiler, tcx| {
                Some(pprust::print_crate(
                    tcx.sess.source_map(),
                    &tcx.resolver_for_lowering().borrow().1,
                    filename,
                    input_src,
                    &NoAnn,
                    true,
                    tcx.sess.edition(),
                    &tcx.sess.psess.attr_id_generator,
                ))
            })
        }
        Stage::Dump(DumpStage::Typed) => {
            with_compiler_global_ctxt(config, prelude, |_compiler, tcx| {
                run_body_checks(tcx);

                Some(pprust_hir::print_crate(
                    tcx.sess.source_map(),
                    tcx.hir_root_module(),
                    filename,
                    input_src,
                    &|id| tcx.hir_attrs(id),
                    &TypedAnn { tcx, maybe_typeck_results: Cell::new(None) },
                ))
            })
        }
        Stage::Dump(DumpStage::Erl) => {
            with_compiler_global_ctxt(config, prelude, |_compiler, tcx| {
                run_body_checks(tcx);
                if let Some(guar) = tcx.sess.dcx().has_errors_or_delayed_bugs() {
                    guar.raise_fatal();
                }

                translate::translate(tcx, translate::TranslateStage::Erl, out_dir)
            })
        }
        Stage::Emit(emit_stage) => with_compiler_global_ctxt(config, prelude, |_compiler, tcx| {
            run_body_checks(tcx);
            if let Some(guar) = tcx.sess.dcx().has_errors_or_delayed_bugs() {
                guar.raise_fatal();
            }

            std::fs::create_dir_all(out_dir).unwrap_or_else(|e| {
                tcx.sess.dcx().fatal(format!("failed to create output directory: {e}"));
            });

            translate::translate(tcx, emit_stage.into(), out_dir);

            let metadata_path = out_dir.join(format!("lib{pkg_name}.rmeta"));
            rustc_metadata::encode_metadata(tcx, &metadata_path, None);

            None
        }),
    };

    if let Some(render) = maybe_render {
        print!("{render}");
    }
}

fn make_config(
    filename: FileName,
    src: &str,
    pkg_name: &str,
    deps: &[PackageDependency],
    out_dir: &Path,
) -> interface::Config {
    let has_explicit_core = deps.iter().any(|dep| dep.name == CORE_PACKAGE_NAME);
    let is_core = pkg_name == CORE_PACKAGE_NAME;

    let core_dep = (!has_explicit_core && !is_core).then(|| {
        let core_rmeta = ensure_core(out_dir);
        PackageDependency { name: CORE_PACKAGE_NAME.to_string(), path: Some(core_rmeta) }
    });

    let mut externs = BTreeMap::new();
    for dep in core_dep.iter().chain(deps.iter()) {
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
        using_internal_features: &INTERNAL_FEATURES,
        opts: config::Options {
            edition: Edition::EditionFuture,
            unstable_features: UnstableFeatures::Allow,
            unstable_opts: config::UnstableOptions { ui_testing: true, ..Default::default() },
            crate_types: vec![config::CrateType::Rlib],
            cg: config::CodegenOptions { panic: Some(PanicStrategy::Abort), ..Default::default() },
            crate_name: Some(pkg_name.to_string()),
            externs: Externs::new(externs),
            search_paths: vec![SearchPath::new(PathKind::Crate, out_dir.to_owned())],
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

fn with_compiler<R>(
    config: Config,
    prelude: &'static str,
    f: impl FnOnce(&Compiler, Crate) -> R + Send,
) -> R
where
    R: Send,
{
    interface::run_compiler(config, move |compiler| {
        let krate = parse_with_prelude(&compiler.sess, prelude);
        f(compiler, krate)
    })
}

fn with_compiler_global_ctxt<T, F>(config: Config, prelude: &'static str, f: F) -> T
where
    T: Send,
    F: for<'tcx> FnOnce(&Compiler, TyCtxt<'tcx>) -> T + Send,
{
    with_compiler(config, prelude, |compiler, krate| {
        passes::create_and_enter_global_ctxt(compiler, krate, |tcx| f(compiler, tcx))
    })
}

impl From<EmitStage> for translate::TranslateStage {
    fn from(stage: EmitStage) -> Self {
        match stage {
            EmitStage::Eaf => Self::Eaf,
            EmitStage::Abstr => Self::Abstr,
            EmitStage::Beam => Self::Beam,
        }
    }
}

fn run_body_checks(tcx: TyCtxt<'_>) {
    tcx.hir_body_owners().for_each(|def_id| {
        tcx.ensure_ok().typeck(def_id);
        if !tcx.is_typeck_child(def_id.to_def_id()) {
            tcx.ensure_ok().check_unsafety(def_id);
            tcx.ensure_ok().check_transmutes(def_id);
        }
    });
}
