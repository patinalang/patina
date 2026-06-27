mod elixir_compat;

use std::path::PathBuf;

use rustc_arena::DroplessArena;
use rustc_ast::{BindingMode, LitKind};
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefId, LocalModDefId};
use rustc_hir::{
    BodyId, Expr, ExprKind, FnSig, Generics, HirId, ImplItemKind, ItemKind, Mod, Pat, PatKind,
    QPath, Stmt, StmtKind, StructTailExpr, TyKind, VariantData,
};
use rustc_middle::erl::{ErlCallTarget, ErlModuleId};
use rustc_middle::span_bug;
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::eaf_builder::EafBuilder;
use crate::compile::impl_collector::ImplCollector;
use crate::compile::module_collector::ModuleNameCollector;
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

pub(super) struct Translator<'tcx, 'arena> {
    tcx: TyCtxt<'tcx>,
    afb: EafBuilder<'tcx, 'arena>,
    modules: ModuleNameCollector<'tcx>,
    impls: ImplCollector<'tcx>,
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub fn new(tcx: TyCtxt<'tcx>, arena: &'arena DroplessArena) -> Translator<'tcx, 'arena> {
        Translator {
            tcx,
            afb: EafBuilder::new(tcx.sess.source_map(), arena),
            modules: ModuleNameCollector::new(tcx),
            impls: ImplCollector::new(tcx),
        }
    }

    pub fn translate(&self) -> Vec<(Symbol, eaf::Module<'arena>)> {
        let mut modules = Vec::new();

        for owner in self.tcx.hir_crate_items(()).owners() {
            let def_id = owner.def_id;

            let form = match self.tcx.def_kind(def_id) {
                DefKind::Mod => self.translate_module(def_id),
                DefKind::Struct => self.translate_struct(def_id),
                _ => continue,
            };

            modules.push(form);
        }

        modules
    }

    fn translate_module(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let module_id = LocalModDefId::new_unchecked(def_id);
        let (module, _span, _hir_id) = self.tcx.hir_get_module(module_id);
        let (forms, exports) = self.collect_module_fns(module);
        let mod_name = self.modules.module_name(ErlModuleId::Mod(def_id.to_def_id()));
        self.assemble_module(module.spans.inner_span, mod_name, exports, forms)
    }

    fn assemble_module(
        &self,
        span: Span,
        mod_name: Symbol,
        exports: impl IntoIterator<Item = (Symbol, usize)>,
        fn_forms: impl IntoIterator<Item = eaf::ModuleForm<'arena>>,
    ) -> (Symbol, eaf::Module<'arena>) {
        let file = self.file_path(span);
        let file_attr = self.afb.file_attr(span, &file.to_string_lossy());
        let module_attr = self.afb.module_attr(span, mod_name);
        let compile_attr = self.afb.wild_attr(span, sym::compile, [sym::no_auto_import]);
        let export_attr = self.afb.export_attr(span, exports);

        let attrs = [file_attr, module_attr, compile_attr, export_attr];
        (mod_name, self.afb.module(attrs.into_iter().chain(fn_forms)))
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
        sig: &FnSig<'tcx>,
        generics: &Generics<'tcx>,
        ident: Ident,
        body: BodyId,
    ) -> eaf::ModuleForm<'arena> {
        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "function with generic parameters", generics);
        }

        let mut scope = Scope::new();
        let body = self.tcx.hir_body(body);

        let function_args = body
            .params
            .iter()
            .map(|param| self.translate_pat(param.pat, &mut scope))
            .collect::<SmallVec<[_; 8]>>();

        let clause_body = self.afb.clause_body(self.translate_expr(body.value, &mut scope));

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

                if exprs.is_empty() {
                    exprs.push(afb.expr_tuple(expr.span, []));
                }

                afb.expr_block(expr.span, exprs)
            }

            ExprKind::Lit(lit) => match lit.node {
                LitKind::Int(val) => afb.expr_int(lit.span, val.0),
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

            ExprKind::Path(qpath) => self.translate_qpath(&qpath, expr.hir_id, scope),

            ExprKind::Call(callee, args) => self.translate_call_expr(expr, callee, args, scope),

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
                    // A struct literal lowers to a synthetic `__struct__()` call to the
                    // struct's own module - there is no call `Expr`, so classify by def id.
                    let fn_atom = afb.expr_atom(expr.span, sym::__struct__);
                    let base = if let Some(def_id) = self.qpath_res(qpath, expr.hir_id).opt_def_id()
                        && self
                            .tcx
                            .erl_call_kind_from(
                                expr.hir_id.owner.to_def_id(),
                                ErlCallTarget::Constructor {
                                    nominal_type: def_id,
                                    variant: None,
                                    args: self
                                        .tcx
                                        .typeck(expr.hir_id.owner.def_id)
                                        .node_args(expr.hir_id),
                                },
                            )
                            .is_some_and(|kind| kind.can_be_local())
                    {
                        afb.expr_local_call(expr.span, fn_atom, [])
                    } else {
                        let mod_expr = self.translate_qpath(qpath, expr.hir_id, scope);
                        afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
                    };
                    let translated_fields_iter = fields.iter().map(|field| {
                        let key = afb.expr_atom(field.ident.span, field.ident.name);
                        (key, self.translate_expr(field.expr, scope))
                    });
                    afb.expr_map_update_exact(expr.span, base, translated_fields_iter)
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

            ExprKind::Closure(closure) => {
                let body = self.tcx.hir_body(closure.body);
                let patterns = body
                    .params
                    .iter()
                    .map(|param| self.translate_pat(param.pat, scope))
                    .collect::<SmallVec<[_; 8]>>();
                let clause_body = afb.clause_body(self.translate_expr(body.value, scope));
                let clause = afb.clause(closure.fn_decl_span, patterns, clause_body);
                afb.expr_fun(expr.span, [clause])
            }

            _ => not_implemented!(self.tcx, expr.span, "expression kind", expr.kind),
        }
    }

    fn qpath_res(&self, qpath: &QPath<'tcx>, hir_id: HirId) -> Res {
        self.tcx.typeck(hir_id.owner.def_id).qpath_res(qpath, hir_id)
    }

    fn translate_qpath(
        &self,
        qpath: &QPath<'tcx>,
        hir_id: HirId,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let span = qpath.span();
        match self.qpath_res(qpath, hir_id) {
            Res::Local(hir_id) => {
                let var = scope.get_var(hir_id).unwrap_or_else(|| {
                    span_fatal!(self.tcx, span, "variable {hir_id:?} not in scope");
                });
                self.afb.expr_var(span, var.erl_name)
            }

            Res::Def(DefKind::ExternMod, def_id) => {
                let name = self.modules.module_name(ErlModuleId::Extern(def_id));
                self.afb.expr_atom(span, name)
            }

            Res::Def(DefKind::Variant | DefKind::Ctor(..), def_id) => {
                let name = self.modules.constructor_name(def_id);
                self.afb.expr_atom(span, name)
            }

            Res::Def(DefKind::Fn, def_id) => {
                let module = self.expect_erl_definition_module(def_id);
                let mod_name = self.modules.module_name(module);
                let fn_name = self.tcx.item_name(def_id);
                let arity = self.tcx.fn_arg_idents(def_id).len();
                self.afb.expr_remote_fun(span, mod_name, fn_name, arity)
            }

            Res::Def(_, def_id) => {
                let module = self.expect_erl_definition_module(def_id);
                let name = self.modules.module_name(module);
                self.afb.expr_atom(span, name)
            }

            Res::SelfTyAlias { alias_to, .. } => {
                let module = self.expect_erl_definition_module(alias_to);
                let name = self.modules.module_name(module);
                self.afb.expr_atom(span, name)
            }

            res => not_implemented!(self.tcx, span, "path resolution", res),
        }
    }

    fn expect_erl_definition_module(&self, def_id: DefId) -> ErlModuleId {
        let Some(module) = self.tcx.erl_definition_module(def_id) else {
            let span = self.tcx.def_span(def_id);
            let kind = self.tcx.def_kind(def_id);
            span_bug!(span, "{kind:?} definition has no resolved Erlang module");
        };
        module
    }

    fn translate_call_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        callee: &'tcx Expr<'tcx>,
        args: &'tcx [Expr<'tcx>],
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let args_exprs =
            args.iter().map(|arg| self.translate_expr(arg, scope)).collect::<SmallVec<[_; 8]>>();

        // Classification is single-sourced through the callee resolver; the arms below
        // only build the corresponding Erlang node.
        let local = self
            .tcx
            .erl_call_kind(expr)
            .expect("call target to have a resolved Erlang module")
            .can_be_local();

        match callee.kind {
            ExprKind::Path(
                ref qpath @ QPath::TypeRelative(
                    rustc_hir::Ty { kind: TyKind::Path(self_ty_qpath), .. },
                    last_segment,
                ),
            ) => {
                if local
                    && let Res::Def(DefKind::AssocFn, def_id) = self.qpath_res(qpath, callee.hir_id)
                {
                    let fn_atom =
                        afb.expr_atom(last_segment.ident.span, self.tcx.item_name(def_id));
                    return afb.expr_local_call(expr.span, fn_atom, args_exprs);
                }

                let module = self.translate_qpath(self_ty_qpath, callee.hir_id, scope);
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
                rustc_hir::Path { res: Res::Def(DefKind::Fn, def_id), .. },
            )) => {
                let fn_name = self.tcx.item_name(*def_id);
                let fn_expr = afb.expr_atom(callee.span, fn_name);

                if local {
                    return afb.expr_local_call(expr.span, fn_expr, args_exprs);
                }

                let Some(module) = self.tcx.erl_definition_module(*def_id) else {
                    span_bug!(callee.span, "function has no resolved Erlang module");
                };
                let mod_name = self.modules.module_name(module);
                let mod_expr = afb.expr_atom(callee.span, mod_name);
                afb.expr_remote_call(expr.span, mod_expr, fn_expr, args_exprs)
            }

            ExprKind::Path(QPath::Resolved(
                _,
                rustc_hir::Path { res: Res::Def(DefKind::ExternMod, leaf), segments, .. },
            )) => {
                let module = self.modules.module_name(ErlModuleId::Extern(self.tcx.parent(*leaf)));
                let fn_name = self.tcx.item_name(*leaf);

                // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
                if let (sym::patina_fmt, sym::display | sym::debug) = (module, fn_name) {
                    return self.translate_format_render(expr, fn_name, &args[0], scope);
                }

                let mod_expr = afb.expr_atom(callee.span, module);

                let last_segment =
                    segments.last().expect("extern mod path to have at least one segment");
                let fn_atom = afb.expr_atom(last_segment.ident.span, fn_name);

                afb.expr_remote_call(expr.span, mod_expr, fn_atom, args_exprs)
            }

            ExprKind::Path(QPath::Resolved(
                _,
                rustc_hir::Path { res: Res::Def(DefKind::Ctor(..), _), .. },
            )) => {
                let fn_atom = afb.expr_atom(callee.span, sym::__struct__);
                let base = if local {
                    afb.expr_local_call(expr.span, fn_atom, [])
                } else {
                    let mod_expr = self.translate_expr(callee, scope);
                    afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
                };
                let updates = args_exprs.into_iter().enumerate().map(|(i, arg)| {
                    (afb.expr_atom(expr.span, Symbol::intern(&i.to_string())), arg)
                });
                afb.expr_map_update_exact(expr.span, base, updates)
            }

            _ => {
                let callee_expr = self.translate_expr(callee, scope);
                afb.expr_local_call(expr.span, callee_expr, args_exprs)
            }
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

            StmtKind::Item(item_id) => {
                let item = self.tcx.hir_item(item_id);
                match item.kind {
                    ItemKind::ExternMod(_) => None,
                    _ => not_implemented!(self.tcx, stmt.span, "statement kind", stmt.kind),
                }
            }
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

    fn collect_module_fns(
        &self,
        module: &Mod<'tcx>,
    ) -> (Vec<eaf::ModuleForm<'arena>>, SmallVec<[(Symbol, usize); 8]>) {
        let mut forms = Vec::new();
        let mut last_file = Some(self.file_path(module.spans.inner_span));
        let mut exports = SmallVec::<[(Symbol, usize); 8]>::new();

        forms.push(elixir_compat::info_function(&self.afb, module.spans.inner_span, None));
        exports.push((sym::__info__, 1));

        for item_id in module.item_ids {
            let item = self.tcx.hir_item(*item_id);
            if let ItemKind::Fn { ref sig, ident, generics, body, has_body } = item.kind
                && has_body
            {
                let function = self.translate_fun(sig, generics, ident, body);
                self.push_function(&mut forms, &mut last_file, item.span, function);

                if self.tcx.is_erl_exported_fn(item.owner_id.def_id) {
                    exports.push((ident.name, sig.decl.inputs.len()));
                }
            }
        }

        (forms, exports)
    }

    fn translate_struct(&self, def_id: LocalDefId) -> (Symbol, eaf::Module<'arena>) {
        let item = self.tcx.hir_expect_item(def_id);
        let ItemKind::Struct(_ident, generics, variant_data) = item.kind else {
            unreachable!("def_kind Struct must be an ItemKind::Struct");
        };

        if !generics.params.is_empty() {
            not_implemented!(self.tcx, generics.span, "struct with generic parameters", generics);
        }

        let mod_name = self.modules.module_name(ErlModuleId::Adt(def_id.to_def_id()));
        let (forms, exports) = self.collect_struct_fns(item.span, mod_name, def_id, variant_data);
        self.assemble_module(item.span, mod_name, exports, forms)
    }

    fn collect_struct_fns(
        &self,
        span: Span,
        mod_name: Symbol,
        def_id: rustc_hir::def_id::LocalDefId,
        variant_data: VariantData<'tcx>,
    ) -> (Vec<eaf::ModuleForm<'arena>>, SmallVec<[(Symbol, usize); 8]>) {
        let mut forms = Vec::new();
        let mut last_file = Some(self.file_path(span));
        let mut exports = SmallVec::<[(Symbol, usize); 8]>::new();

        forms.push(elixir_compat::info_function(&self.afb, span, Some(variant_data)));
        exports.push((sym::__info__, 1));

        forms.push(elixir_compat::struct0_function(&self.afb, span, mod_name, variant_data));
        exports.push((sym::__struct__, 0));

        forms.push(elixir_compat::struct1_function(&self.afb, span));
        exports.push((sym::__struct__, 1));

        if let Some(impls) = self.impls.impls_for(def_id.to_def_id()) {
            for imp in impls {
                if !imp.generics.params.is_empty() {
                    not_implemented!(
                        self.tcx,
                        imp.generics.span,
                        "impl with generic parameters",
                        imp.generics
                    );
                }
                for impl_item_id in imp.items {
                    let impl_item = self.tcx.hir_impl_item(*impl_item_id);
                    let ImplItemKind::Fn(sig, body) = impl_item.kind else {
                        continue;
                    };

                    let function =
                        self.translate_fun(&sig, impl_item.generics, impl_item.ident, body);
                    self.push_function(&mut forms, &mut last_file, impl_item.span, function);
                    if self.tcx.is_erl_exported_fn(impl_item.owner_id.def_id) {
                        exports.push((impl_item.ident.name, sig.decl.inputs.len()));
                    }
                }
            }
        }

        (forms, exports)
    }
}
