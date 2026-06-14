use std::fs::File;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

use rustc_arena::DroplessArena;
use rustc_ast::{BindingMode, LitKind};
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::LocalModDefId;
use rustc_hir::{
    BodyId, Expr, ExprKind, FnSig, Generics, HirId, ItemKind, Mod, Pat, PatKind, QPath, Stmt,
    StmtKind, StructTailExpr, TyKind,
};
use rustc_middle::ty::{TyCtxt, Visibility};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::compile_server::CompileServer;
use crate::compile::eaf_builder::EafBuilder;
use crate::compile::module_collector::ModuleNameCollector;
use crate::erlang;
use crate::erlang::abstract_format as eaf;

macro_rules! span_fatal {
    ($tcx:expr, $span:expr, $($arg:tt)*) => {
        $tcx.sess.dcx().span_fatal($span, format!($($arg)*))
    };
}

macro_rules! not_implemented {
    ($tcx:expr, $span:expr, $name:literal, $kind:expr) => {
        span_fatal!($tcx, $span, "{} not yet implemented: {:?}", $name, $kind)
    };
}

#[allow(unused)]
struct PatinaVar {
    pub vsn: u32,
    pub erl_name: Symbol,
    pub ident: Ident,
    pub is_mut: bool,
}

struct Scope {
    vars: FxHashMap<HirId, PatinaVar>,
    next_var_vsn: u32,
}

impl Scope {
    fn new() -> Self {
        Self { vars: FxHashMap::default(), next_var_vsn: 0 }
    }

    fn get_var(&self, id: HirId) -> Option<&PatinaVar> {
        self.vars.get(&id)
    }

    fn add_var(&mut self, id: HirId, is_mut: bool, ident: Ident) -> &PatinaVar {
        let vsn = self.next_var_vsn;
        self.next_var_vsn += 1;

        let erl_name = Symbol::intern(&format!("_var_{}", vsn));

        self.vars.entry(id).insert_entry(PatinaVar { vsn, erl_name, ident, is_mut }).into_mut()
    }
}

struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
    modules: ModuleNameCollector<'tcx>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        let source_map = tcx.sess.source_map();
        let afb = EafBuilder::new(source_map, arena);

        let modules = ModuleNameCollector::new(tcx);

        Translator { tcx, afb, modules }
    }

    fn translate(&self) -> Vec<(Symbol, eaf::Module<'arena>)> {
        vec![self.translate_module(LocalModDefId::CRATE_DEF_ID)]
    }

    fn translate_module(&self, module_id: LocalModDefId) -> (Symbol, eaf::Module<'arena>) {
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);

        let mod_name = self.modules.module_name(module_id.to_def_id());
        let span = module.spans.inner_span;

        let (exported_fns,) = self.gather_items(module);

        let file = self.file_path(span);
        let file_attr = self.afb.file_attr(span, &file.to_string_lossy());
        let module_attr = self.afb.module_attr(span, mod_name);
        let compile_attr = self.afb.wild_attr(span, sym::compile, [sym::no_auto_import]);
        let export_attr = self.afb.export_attr(span, exported_fns);

        let mut forms = vec![file_attr, module_attr, compile_attr, export_attr];
        let mut last_file = Some(file);
        for (span, function) in self.translate_items(module) {
            self.push_function(&mut forms, &mut last_file, span, function);
        }

        (mod_name, self.afb.module(forms))
    }

    /// The file that contains the outermost macro call site of `span`.
    fn file_path(&self, span: Span) -> PathBuf {
        let span = span.source_callsite();
        self.tcx
            .sess
            .source_map()
            .span_to_filename(span)
            .into_local_path()
            .unwrap_or_else(|| span_fatal!(self.tcx, span, "no filename found"))
    }

    fn push_function(
        &self,
        forms: &mut Vec<eaf::ModuleForm<'arena>>,
        last_file: &mut Option<PathBuf>,
        span: Span,
        function: eaf::ModuleForm<'arena>,
    ) {
        // If the function is in a different file than the previous, push a `-file` attribute
        // as well.
        let file = self.file_path(span);
        if last_file.as_ref() != Some(&file) {
            forms.push(self.afb.file_attr(span, &file.to_string_lossy()));
            *last_file = Some(file);
        }
        forms.push(function);
    }

    fn translate_fun(
        &self,
        span: Span,
        sig: &FnSig<'tcx>,
        generics: &Generics<'tcx>,
        ident: Ident,
        body: BodyId,
    ) -> eaf::ModuleForm<'arena> {
        if !generics.params.is_empty() {
            not_implemented!(self.tcx, span, "function with generic parameters", generics);
        }

        let mut scope = Scope::new();

        let body = self.tcx.hir_body(body);
        let clause_body = self.afb.clause_body(self.translate_expr(body.value, &mut scope));

        let function_args =
            body.params.iter().map(|param| self.translate_pat(param.pat, &mut scope));

        let clause = self.afb.clause(sig.span, function_args, clause_body);

        let fn_name = ident.name;
        let fn_arity = sig.decl.inputs.len();

        self.afb.function(sig.span, fn_name, fn_arity, [clause])
    }

    fn translate_expr(&self, expr: &'tcx Expr<'tcx>, scope: &mut Scope) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        match expr.kind {
            ExprKind::Block(block, label) => {
                if label.is_some() {
                    span_fatal!(self.tcx, expr.span, "labels are not supported");
                }

                let mut exprs = block
                    .stmts
                    .iter()
                    .flat_map(|stmt| self.translate_stmt(stmt, scope))
                    .collect::<Vec<_>>();

                if let Some(tail) = block.expr {
                    exprs.push(self.translate_expr(tail, scope));
                }

                afb.expr_block(expr.span, exprs)
            }

            ExprKind::Lit(lit) => match lit.node {
                LitKind::Int(val) => afb.expr_integer(lit.span, val.0),
                LitKind::Str(symbol, _style) => afb.expr_binary_string(lit.span, symbol),
                LitKind::Float(symbol, _style) => {
                    let symbol_str = symbol.as_str();
                    let float = symbol_str.parse::<f64>().unwrap_or_else(|e| {
                        span_fatal!(
                            self.tcx,
                            lit.span,
                            "failed to parse float `{symbol_str}`: {e}"
                        );
                    });
                    afb.expr_float(lit.span, float)
                }
                _ => not_implemented!(self.tcx, lit.span, "literal kind", lit.node),
            },

            ExprKind::Path(qpath) => self.translate_qpath(&qpath, scope),

            ExprKind::Call(callee, args) => {
                let args_exprs = args
                    .iter()
                    .map(|arg| self.translate_expr(arg, scope))
                    .collect::<SmallVec<[_; 8]>>();

                match callee.kind {
                    ExprKind::Path(QPath::TypeRelative(
                        rustc_hir::Ty { kind: TyKind::Path(qpath), .. },
                        last_segment,
                    )) => {
                        let module = self.translate_qpath(qpath, scope);
                        let fn_name = last_segment.ident.name;

                        // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
                        if let (
                            eaf::Expr::AtomicLiteral(eaf::AtomicLiteral::Atom(
                                _,
                                eaf::Symbol(sym::patina_fmt),
                            )),
                            sym::display | sym::debug,
                        ) = (&module, fn_name)
                        {
                            return self.translate_format_render(expr, fn_name, &args[0], scope);
                        }

                        let fn_atom = afb.expr_atom(last_segment.ident.span, fn_name);
                        afb.expr_remote_call(expr.span, module, fn_atom, args_exprs)
                    }

                    ExprKind::Path(QPath::Resolved(
                        _,
                        rustc_hir::Path { res: Res::Def(DefKind::Ctor(..), _), .. },
                    )) => {
                        let mod_expr = self.translate_expr(callee, scope);
                        let fn_name = sym::__struct__;
                        let fn_atom = afb.expr_atom(callee.span, fn_name);
                        afb.expr_remote_call(expr.span, mod_expr, fn_atom, args_exprs)
                    }

                    _ => {
                        let callee_expr = self.translate_expr(callee, scope);
                        afb.expr_local_call(expr.span, callee_expr, args_exprs)
                    }
                }
            }

            ExprKind::Array(elements) => self.afb.expr_list(
                expr.span,
                elements.iter().map(|element| self.translate_expr(element, scope)),
            ),

            ExprKind::Cast(expr, _ty) => self.translate_expr(expr, scope),

            ExprKind::Tup(elements) => {
                let elements = elements.iter().map(|element| self.translate_expr(element, scope));
                afb.expr_tuple(expr.span, elements)
            }

            ExprKind::Struct(qpath, fields, tail) => match tail {
                StructTailExpr::None => {
                    let tag = (
                        afb.expr_atom(expr.span, sym::__struct__),
                        self.translate_qpath(qpath, scope),
                    );

                    let translated_fields_iter = fields.iter().map(|field| {
                        let key = afb.expr_atom(field.ident.span, field.ident.name);
                        (key, self.translate_expr(field.expr, scope))
                    });
                    let assocs = std::iter::once(tag).chain(translated_fields_iter);
                    afb.expr_map(expr.span, assocs)
                }
                StructTailExpr::Base(base) => {
                    let base = self.translate_expr(base, scope);
                    let translated_fields_iter = fields.iter().map(|field| {
                        let key = afb.expr_atom(field.ident.span, field.ident.name);
                        (key, self.translate_expr(field.expr, scope))
                    });
                    afb.expr_map_update_exact(expr.span, base, translated_fields_iter)
                }
                StructTailExpr::DefaultFields(_) | StructTailExpr::NoneWithError(_) => {
                    not_implemented!(self.tcx, expr.span, "struct with default field values", tail)
                }
            },

            ExprKind::Field(expr, ident) => {
                let mod_name = afb.expr_atom(expr.span, sym::erlang);
                let fn_name = afb.expr_atom(expr.span, sym::map_get);
                let subject = self.translate_expr(expr, scope);
                let field_name = afb.expr_atom(ident.span, ident.name);

                afb.expr_remote_call(expr.span, mod_name, fn_name, [field_name, subject])
            }

            _ => not_implemented!(self.tcx, expr.span, "expression kind", expr.kind),
        }
    }

    fn translate_qpath(&self, qpath: &QPath<'tcx>, scope: &mut Scope) -> eaf::Expr<'arena> {
        match qpath {
            QPath::Resolved(_, path) => self.translate_path(path, scope),
            _ => not_implemented!(self.tcx, qpath.span(), "qpath kind", qpath),
        }
    }

    fn translate_path(&self, path: &rustc_hir::Path<'tcx>, scope: &mut Scope) -> eaf::Expr<'arena> {
        match path.res {
            Res::Local(hir_id) => {
                let var = scope.get_var(hir_id).unwrap_or_else(|| {
                    span_fatal!(self.tcx, path.span, "variable {hir_id:?} not in scope");
                });
                self.afb.expr_var(path.span, var.erl_name)
            }

            Res::Def(_kind, def_id) => {
                let name = self.modules.module_name(def_id);
                self.afb.expr_atom(path.span, name)
            }

            _ => not_implemented!(self.tcx, path.span, "path resolution", path.res),
        }
    }

    fn translate_format_render(
        &self,
        expr: &'tcx Expr<'tcx>,
        fn_name: Symbol,
        arg: &'tcx Expr<'tcx>,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let typeck = self.tcx.typeck(arg.hir_id.owner.def_id);
        let ty = typeck.expr_ty(arg);

        let translated = self.translate_expr(arg, scope);

        if fn_name == sym::display && ty.is_str() {
            return translated;
        }

        let format_str = self.afb.expr_string(expr.span, sym::debug_format_str);
        let args_list = self.afb.expr_list(expr.span, [translated]);

        afb.expr_remote_call(
            expr.span,
            afb.expr_atom(expr.span, sym::io_lib),
            afb.expr_atom(expr.span, sym::format),
            [format_str, args_list],
        )
    }

    fn translate_stmt(
        &self,
        stmt: &'tcx Stmt<'tcx>,
        scope: &mut Scope,
    ) -> Option<eaf::Expr<'arena>> {
        let afb = &self.afb;

        match stmt.kind {
            StmtKind::Let(let_stmt) => {
                let rhs = {
                    let init = let_stmt.init?;
                    self.translate_expr(init, scope)
                };
                let lhs = self.translate_pat(let_stmt.pat, scope);
                Some(afb.expr_match(stmt.span, lhs, rhs))
            }

            StmtKind::Semi(expr) | StmtKind::Expr(expr) => Some(self.translate_expr(expr, scope)),

            _ => not_implemented!(self.tcx, stmt.span, "statement kind", stmt.kind),
        }
    }

    fn translate_pat(&self, pat: &'tcx Pat<'tcx>, scope: &mut Scope) -> eaf::Pattern<'arena> {
        let afb = &self.afb;

        match pat.kind {
            PatKind::Wild => afb.pat_var(pat.span, kw::Underscore),

            PatKind::Binding(binding_mode, id, ident, subpat) => {
                let is_mut = match binding_mode {
                    BindingMode::NONE => false,
                    BindingMode::MUT => true,
                    _ => not_implemented!(self.tcx, pat.span, "binding mode", binding_mode),
                };

                if subpat.is_some() {
                    not_implemented!(self.tcx, pat.span, "binding with subpattern", subpat);
                }

                let var = scope.add_var(id, is_mut, ident);
                afb.pat_var(pat.span, var.erl_name)
            }

            PatKind::Struct(_qpath, fields, _tail) => {
                let translated_fields = fields.iter().map(|field| {
                    let key = afb.pat_atom(field.ident.span, field.ident.name);
                    (key, self.translate_pat(field.pat, scope))
                });

                afb.pat_map(pat.span, translated_fields)
            }

            PatKind::TupleStruct(_qpath, subpats, tail) => {
                if tail.as_opt_usize().is_some() {
                    not_implemented!(self.tcx, pat.span, "tuple struct pattern with rest", tail);
                }

                let translated_fields = subpats.iter().enumerate().map(|(i, subpat)| {
                    let key = afb.pat_atom(pat.span, Symbol::intern(&i.to_string()));
                    (key, self.translate_pat(subpat, scope))
                });

                afb.pat_map(pat.span, translated_fields)
            }

            _ => not_implemented!(self.tcx, pat.span, "pattern kind", pat.kind),
        }
    }

    fn translate_items(
        &self,
        module: &Mod<'tcx>,
    ) -> impl Iterator<Item = (Span, eaf::ModuleForm<'arena>)> {
        module.item_ids.iter().flat_map(|item_id| {
            let item = self.tcx.hir_item(*item_id);
            match item.kind {
                ItemKind::Fn { ref sig, ident, generics, body, has_body } if has_body => {
                    Some((item.span, self.translate_fun(item.span, sig, generics, ident, body)))
                }
                _ => None,
            }
        })
    }

    /// Gathers important items from a module definition.
    /// Returns a tuple of:
    /// - A vector of exported functions (name, arity)
    fn gather_items(&self, module: &Mod<'tcx>) -> (SmallVec<[(Symbol, usize); 8]>,) {
        let mut exported_fns = SmallVec::<[(Symbol, usize); 8]>::new();

        for item_id in module.item_ids {
            let item = self.tcx.hir_item(*item_id);
            match item.kind {
                ItemKind::Fn { ref sig, ident, has_body, .. } => {
                    let def_id = item.owner_id.def_id;
                    let is_visible = match self.tcx.visibility(def_id) {
                        Visibility::Public => true,
                        Visibility::Restricted(restricted_id) => {
                            restricted_id != self.tcx.parent_module_from_def_id(def_id).to_def_id()
                        }
                    };

                    if has_body && is_visible {
                        exported_fns.push((ident.name, sig.decl.inputs.len()));
                    }
                }
                _ => (),
            }
        }

        (exported_fns,)
    }
}

pub(super) enum TranslateStage {
    Eaf,
    Abstr,
    Erl,
    Beam,
}

pub(super) fn translate(tcx: TyCtxt<'_>, stage: TranslateStage, out_dir: &Path) -> Option<String> {
    let dcx = tcx.sess.dcx();
    let arena = DroplessArena::default();
    let translator = Translator::new(tcx, &arena);

    let modules = translator.translate();

    match stage {
        TranslateStage::Eaf => {
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                let eaf_path = out_dir.join(format!("{mod_name}.eaf"));
                let mut file = File::create(&eaf_path).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
                file.write_all(&bytes).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
            }
            None
        }
        TranslateStage::Abstr => {
            let mut server = CompileServer::start(dcx);
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                let mut rendered = server.render_abstr(&bytes);
                if !rendered.ends_with('\n') {
                    rendered.push('\n');
                }
                let abstr_path = out_dir.join(format!("{mod_name}.abstr"));
                let mut file = File::create(&abstr_path).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
                file.write_all(rendered.as_bytes()).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to write file: {e}"));
                });
            }
            None
        }
        TranslateStage::Erl => {
            let mut server = CompileServer::start(dcx);
            let mut rendered = String::new();
            for (_, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                if !rendered.is_empty() {
                    rendered.push('\n');
                }
                rendered.push_str(&server.render_erl(&bytes));
            }
            Some(rendered)
        }
        TranslateStage::Beam => {
            let mut server = CompileServer::start(dcx);
            for (mod_name, module) in &modules {
                let bytes = erlang::to_bytes(module).unwrap_or_else(|e| {
                    dcx.fatal(format!("failed to serialize module: {e}"));
                });
                match server.compile(&bytes) {
                    Ok(ok) => {
                        if !ok.warnings.is_empty() && ok.warnings != "[]" {
                            dcx.warn(ok.warnings);
                        }
                        let beam_path = out_dir.join(format!("{mod_name}.beam"));
                        let mut file = File::create(&beam_path).unwrap_or_else(|e| {
                            dcx.fatal(format!("failed to write file: {e}"));
                        });
                        file.write_all(&ok.beam).unwrap_or_else(|e| {
                            dcx.fatal(format!("failed to write file: {e}"));
                        });
                    }
                    Err(err) => {
                        dcx.fatal(format!("erl backend failed: {}\n{}", err.errors, err.warnings));
                    }
                }
            }
            None
        }
    }
}
