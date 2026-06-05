use std::cell::Cell;
use std::path::Path;
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
use rustc_session::{Session, config};
use rustc_span::edition::Edition;
use rustc_span::source_map::FilePathMapping;
use rustc_span::{ErrorGuaranteed, FileName, RealFileName};
use rustc_target::spec::PanicStrategy;

use crate::{DumpStage, Stage};

static INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

const PRELUDE: &str = r#"
#![feature(no_core)]
#![no_std]
#![no_core]
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

pub fn compile(stage: Stage, filename: &Path, src: &str, _out_dir: &Path) {
    let filename =
        FileName::Real(FilePathMapping::empty().to_real_filename(&RealFileName::empty(), filename));

    let config = make_config(filename.clone(), src);
    let input_src = format!("{PRELUDE}\n{src}");

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
            with_compiler_global_ctxt(config, PRELUDE, |_compiler, tcx| {
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
            with_compiler_global_ctxt(config, PRELUDE, |_compiler, tcx| {
                tcx.hir_body_owners().for_each(|def_id| tcx.ensure_ok().typeck(def_id));

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
    };

    if let Some(render) = maybe_render {
        print!("{render}");
    }
}

fn make_config(filename: FileName, src: &str) -> interface::Config {
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
