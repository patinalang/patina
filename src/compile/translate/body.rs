use rustc_abi::{FieldIdx, VariantIdx};
use rustc_ast::LitKind;
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{BindingMode, Lit};
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution};
use rustc_middle::mir::{AssignOp, BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::{
    self, BlockId, Expr as ThirExpr, ExprId, ExprKind, LocalVarId, Pat as ThirPat, PatKind, StmtId,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::Translator;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

pub(crate) struct TranslatedBody<'arena> {
    pub params: SmallVec<[eaf::Pattern<'arena>; 8]>,
    pub value: eaf::Expr<'arena>,
}

pub(crate) struct Scope {
    pub vars: FxHashMap<LocalVarId, PatinaVar>,
    pub next_var_vsn: u32,
}

#[allow(unused)]
pub(crate) struct PatinaVar {
    pub vsn: u32,
    pub erl_name: Symbol,
}

impl Scope {
    pub(super) fn new() -> Self {
        Self { vars: FxHashMap::default(), next_var_vsn: 0 }
    }

    fn get_var(&self, id: LocalVarId) -> Option<&PatinaVar> {
        self.vars.get(&id)
    }

    fn add_var(&mut self, id: LocalVarId, _ident: Ident) -> &PatinaVar {
        let vsn = self.next_var_vsn;
        self.next_var_vsn += 1;

        // let erl_name = Symbol::intern(&format!("_{}@{}", ident.as_str(), vsn));
        let erl_name = Symbol::intern(&format!("_var_{}", vsn));

        self.vars.entry(id).insert_entry(PatinaVar { vsn, erl_name }).into_mut()
    }
}

pub(super) struct BodyCx<'a, 'tcx, 'arena> {
    pub translator: &'a Translator<'tcx, 'arena>,
    pub owner: LocalDefId,
    pub thir: &'a thir::Thir<'tcx>,
    pub scope: &'a mut Scope,
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn tcx(&self) -> TyCtxt<'tcx> {
        self.translator.tcx
    }

    pub(super) fn translate_expr(&mut self, expr_id: ExprId) -> eaf::Expr<'arena> {
        let expr = self.resolve_expr(expr_id);

        match &expr.kind {
            ExprKind::Block { block } => self.translate_block(*block),

            ExprKind::Literal { lit, neg } => self.translate_literal(expr.span, lit, *neg),

            ExprKind::VarRef { id } => self.translate_var_ref(expr.span, *id),
            ExprKind::UpvarRef { var_hir_id, .. } => self.translate_var_ref(expr.span, *var_hir_id),

            ExprKind::Array { fields } => self.translate_list(expr.span, fields),

            ExprKind::Tuple { fields } => self.translate_tuple(expr.span, fields),

            ExprKind::Field { lhs, variant_index, name } => {
                self.translate_field(expr, *lhs, *variant_index, *name)
            }

            ExprKind::Binary { op, lhs, rhs } => self.translate_binary(expr, *op, *lhs, *rhs),

            ExprKind::LogicalOp { op, lhs, rhs } => self.translate_logical(expr, *op, *lhs, *rhs),

            ExprKind::Unary { op, arg } => self.translate_unary(expr, *op, *arg),

            ExprKind::Call { ty, fun, args, forced_extern, .. } => {
                self.translate_call(expr.span, *ty, *fun, args, *forced_extern)
            }

            ExprKind::Adt(adt_expr) => self.translate_adt(expr, adt_expr),

            ExprKind::Closure(closure) => self.translate_closure(expr, closure),

            ExprKind::ZstLiteral { .. } => self.translate_zst_literal(expr),

            ExprKind::Assign { lhs, rhs } => self.translate_assign(expr, *lhs, *rhs),
            ExprKind::AssignOp { op, lhs, rhs } => self.translate_assign_op(expr, *op, *lhs, *rhs),

            ExprKind::Match { scrutinee, arms, .. } => self.translate_match(expr, *scrutinee, arms),

            _ => not_implemented!(self.tcx(), expr.span, "THIR expression kind", expr.kind),
        }
    }

    fn translate_list(&mut self, span: Span, fields: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let items = fields.iter().map(|&id| self.translate_expr(id));
        afb.expr_list(span, items)
    }

    fn translate_tuple(&mut self, span: Span, fields: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let elems = fields.iter().map(|&id| self.translate_expr(id));
        afb.expr_tuple(span, elems)
    }

    fn translate_literal(&self, span: Span, lit: &Lit, neg: bool) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let value = match lit.node {
            LitKind::Str(symbol, _style) => afb.expr_binary_string(lit.span, symbol),
            LitKind::ByteStr(symbol, _style) => afb.expr_binary_bytes(lit.span, symbol),
            LitKind::Char(char) => afb.expr_char(lit.span, char),
            LitKind::Int(val) => afb.expr_int(lit.span, val.0),
            LitKind::Float(symbol, _style) => {
                let symbol_str = symbol.as_str();
                let float = symbol_str.parse::<f64>().unwrap_or_else(|e| {
                    span_fatal!(self.tcx(), lit.span, "failed to parse float `{symbol_str}`: {e}")
                });
                afb.expr_float(lit.span, float)
            }
            LitKind::Bool(bool) => {
                if bool {
                    afb.expr_atom(lit.span, kw::True)
                } else {
                    afb.expr_atom(lit.span, kw::False)
                }
            }
            LitKind::CStr(..) | LitKind::Byte(..) | LitKind::Err(..) => {
                span_fatal!(
                    self.tcx(),
                    lit.span,
                    "literal kind not supported: {}",
                    lit.node.to_string()
                );
            }
        };

        if neg { afb.expr_unary_op(span, eaf::UnaryOp::Neg, value) } else { value }
    }

    fn translate_var_ref(&self, span: Span, id: LocalVarId) -> eaf::Expr<'arena> {
        let ident = self.tcx().hir_ident(id.0);

        let var = self.scope.get_var(id).unwrap_or_else(|| {
            // TODO: this should be `report_use_of_uninitialized` from rustc
            span_fatal!(self.tcx(), span, "variable {} is partially initialized", ident.to_string())
        });
        self.translator.afb.expr_var(span, var.erl_name)
    }

    fn translate_field(
        &mut self,
        expr: &ThirExpr<'tcx>,
        lhs: ExprId,
        variant_index: VariantIdx,
        name: FieldIdx,
    ) -> eaf::Expr<'arena> {
        let lhs_expr = &self.thir[lhs];
        let ty = lhs_expr.ty;

        let adt_def = ty.ty_adt_def().unwrap_or_else(|| {
            not_implemented!(self.tcx(), expr.span, "field access on non-ADT type", ty);
        });

        let variant = adt_def.variant(variant_index);
        let field = &variant.fields[name];
        let key = field.name;

        if self.tcx().erl_adt_repr(adt_def.did()) != rustc_middle::erl::ErlAdtRepr::Elixir {
            not_implemented!(self.tcx(), expr.span, "field access on non-Elixir ADT", ty);
        }

        let subject = self.translate_expr(lhs);

        let afb = &self.translator.afb;
        let mod_name = afb.expr_atom(expr.span, sym::erlang);
        let fn_name = afb.expr_atom(expr.span, sym::map_get);
        let field_name = afb.expr_atom(lhs_expr.span, key);

        afb.expr_remote_call(expr.span, mod_name, fn_name, [field_name, subject])
    }

    fn translate_block(&mut self, id: BlockId) -> eaf::Expr<'arena> {
        let block = &self.thir[id];

        let mut exprs: Vec<eaf::Expr<'arena>> = Vec::new();

        for stmt_id in block.stmts.iter() {
            if let Some(expr) = self.translate_stmt(*stmt_id) {
                exprs.push(expr);
            }
        }

        if let Some(tail) = block.expr {
            exprs.push(self.translate_expr(tail));
        }

        if exprs.is_empty() {
            exprs.push(self.translator.afb.expr_tuple(block.span, []));
        }

        self.translator.afb.expr_block(block.span, exprs)
    }

    fn translate_stmt(&mut self, id: StmtId) -> Option<eaf::Expr<'arena>> {
        let stmt = &self.thir[id];

        match stmt.kind {
            thir::StmtKind::Expr { expr, .. } => Some(self.translate_expr(expr)),

            thir::StmtKind::Let { ref pattern, initializer, else_block, span, .. } => {
                if else_block.is_some() {
                    not_implemented!(self.tcx(), span, "let-else", else_block);
                }

                let rhs = self.translate_expr(initializer?);
                let lhs = self.translate_pattern(pattern);
                Some(self.translator.afb.expr_match(span, lhs, rhs))
            }
        }
    }

    fn translate_binary(
        &mut self,
        expr: &ThirExpr<'tcx>,
        op: BinOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let lhs_ty = self.thir[lhs].ty;
        let afb = &self.translator.afb;

        let left = self.translate_expr(lhs);
        let right = self.translate_expr(rhs);

        // String concatenation lowers to binary concatenation.
        if op == BinOp::Add && lhs_ty.is_str() {
            let left_bin_elem =
                afb.expr_binary_element(self.thir[lhs].span, left, None, Some([eaf::Tsl::Binary]));
            let right_bin_elem =
                afb.expr_binary_element(self.thir[rhs].span, right, None, Some([eaf::Tsl::Binary]));

            return afb.expr_binary(expr.span, [left_bin_elem, right_bin_elem]);
        }

        use BinOp::*;
        let operator = match op {
            Add if lhs_ty.is_array() || lhs_ty.is_array_slice() => eaf::BinaryOp::Concat,
            Add => eaf::BinaryOp::Add,
            Sub if lhs_ty.is_array() || lhs_ty.is_array_slice() => eaf::BinaryOp::Subtract,
            Sub => eaf::BinaryOp::Sub,
            Mul => eaf::BinaryOp::Mul,
            Div if lhs_ty.is_floating_point() => eaf::BinaryOp::Div,
            Div => eaf::BinaryOp::DivInt,
            Rem => eaf::BinaryOp::Rem,
            BitXor => eaf::BinaryOp::Bxor,
            BitAnd => eaf::BinaryOp::Band,
            BitOr => eaf::BinaryOp::Bor,
            Shl => eaf::BinaryOp::Bsl,
            Shr => eaf::BinaryOp::Bsr,
            Eq => eaf::BinaryOp::EqExact,
            Ne => eaf::BinaryOp::NeExact,
            Lt => eaf::BinaryOp::Lt,
            Le => eaf::BinaryOp::Le,
            Ge => eaf::BinaryOp::Ge,
            Gt => eaf::BinaryOp::Gt,
            Cmp => not_implemented!(self.tcx(), expr.span, "binary operator", op),
            AddUnchecked | SubUnchecked | MulUnchecked | AddWithOverflow | SubWithOverflow
            | MulWithOverflow | ShlUnchecked | ShrUnchecked | Offset => {
                span_fatal!(self.tcx(), expr.span, "binary operator not supported: {op:?}");
            }
        };

        afb.expr_binary_op(expr.span, operator, left, right)
    }

    fn translate_logical(
        &mut self,
        expr: &ThirExpr<'tcx>,
        op: rustc_middle::thir::LogicalOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let operator = match op {
            rustc_middle::thir::LogicalOp::And => eaf::BinaryOp::Andalso,
            rustc_middle::thir::LogicalOp::Or => eaf::BinaryOp::Orelse,
        };

        let left = self.translate_expr(lhs);
        let right = self.translate_expr(rhs);
        self.translator.afb.expr_binary_op(expr.span, operator, left, right)
    }

    fn translate_unary(
        &mut self,
        expr: &ThirExpr<'tcx>,
        op: UnOp,
        arg: ExprId,
    ) -> eaf::Expr<'arena> {
        let arg_ty = self.thir[arg].ty;

        let unary_op = match op {
            UnOp::Not if arg_ty.is_bool() => eaf::UnaryOp::Not,
            UnOp::Not => eaf::UnaryOp::Bnot,
            UnOp::Neg => eaf::UnaryOp::Neg,
            UnOp::PtrMetadata => {
                span_fatal!(self.tcx(), expr.span, "unary operator not supported: {op:?}");
            }
        };

        let operand = self.translate_expr(arg);
        self.translator.afb.expr_unary_op(expr.span, unary_op, operand)
    }

    fn translate_call(
        &mut self,
        span: Span,
        callable_ty: Ty<'tcx>,
        fun: ExprId,
        args: &[ExprId],
        forced_extern: bool,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let arg_exprs: SmallVec<[eaf::Expr<'arena>; 8]> =
            args.iter().map(|&id| self.translate_expr(id)).collect();

        let (erl_call_target, callee_expr) = if let ty::TyKind::FnDef(callee_def_id, generic_args) =
            callable_ty.kind()
        {
            let erl_call_target = self.tcx().erl_call_target_for_def(*callee_def_id, generic_args);

            (erl_call_target, afb.expr_atom(span, self.tcx().item_name(*callee_def_id)))
        } else if let ExprKind::ExternMod { def_id } = &self.resolve_expr(fun).kind {
            let erl_call_target =
                self.tcx().erl_call_target_for_def(*def_id, ty::GenericArgs::empty());
            (erl_call_target, afb.expr_atom(span, self.tcx().item_name(*def_id)))
        } else {
            (ErlCallTarget::Dynamic, self.translate_expr(fun))
        };

        let call_kind = self
            .tcx()
            .erl_call_kind(self.owner.to_def_id(), erl_call_target, Some(forced_extern))
            .unwrap_or_else(|| span_bug!(span, "call target couldn't be classified"));

        if matches!(call_kind, ErlCallKind::Local | ErlCallKind::Dynamic) {
            return afb.expr_local_call(span, callee_expr, arg_exprs);
        }

        let module_resolution = self
            .tcx()
            .erl_module_for_target(self.owner.to_def_id(), erl_call_target)
            .unwrap_or_else(|| span_bug!(span, "item call must have module resolution"));

        let ErlModuleResolution::Static(module_id) = module_resolution else {
            span_bug!(span, "dynamic target in static call path");
        };

        let mod_name = self.translator.modules.module_name(module_id);

        // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
        if let ErlCallTarget::Extern { function, .. } = erl_call_target {
            let fn_name = self.tcx().item_name(function);
            if let (sym::patina_fmt, sym::display | sym::debug) = (mod_name, fn_name) {
                return self.translate_format_render(span, fn_name, args[0]);
            }
        }

        let mod_expr = afb.expr_atom(span, mod_name);

        afb.expr_remote_call(span, mod_expr, callee_expr, arg_exprs)
    }

    fn translate_format_render(
        &mut self,
        span: Span,
        fn_name: Symbol,
        arg: ExprId,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let ty = self.resolve_expr(arg).ty;

        let translated = self.translate_expr(arg);
        if fn_name == sym::display && ty.is_str() {
            return translated;
        }

        let format_str = self.translator.afb.expr_string(span, sym::debug_format_str);
        let args_list = self.translator.afb.expr_list(span, [translated]);

        afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::io_lib),
            afb.expr_atom(span, sym::format),
            [format_str, args_list],
        )
    }

    fn translate_closure(
        &mut self,
        expr: &ThirExpr<'tcx>,
        closure: &thir::ClosureExpr<'tcx>,
    ) -> eaf::Expr<'arena> {
        let translated = self.translator.translate_body(closure.closure_id, self.scope);

        let afb = &self.translator.afb;
        let clause_body = afb.clause_body(translated.value);
        let clause = afb.clause(expr.span, translated.params, None, clause_body);
        afb.expr_fun(expr.span, [clause])
    }

    fn translate_zst_literal(&mut self, expr: &ThirExpr<'tcx>) -> eaf::Expr<'arena> {
        let tcx = self.tcx();
        let afb = &self.translator.afb;

        if let ty::TyKind::FnDef(def_id, args) = expr.ty.kind()
            && matches!(tcx.def_kind(*def_id), DefKind::Fn | DefKind::AssocFn)
        {
            let target = tcx.erl_call_target_for_def(*def_id, args);

            let module_resolution =
                tcx.erl_module_for_target(self.owner.to_def_id(), target).unwrap_or_else(|| {
                    span_bug!(expr.span, "function-item target must resolve to a module")
                });

            let ErlModuleResolution::Static(module_id) = module_resolution else {
                not_implemented!(tcx, expr.span, "dynamic function-item value", expr.ty);
            };

            let mod_name = self.translator.modules.module_name(module_id);
            let fn_name = tcx.item_name(*def_id);
            let arity = tcx.fn_arg_idents(*def_id).len();
            afb.expr_remote_fun(expr.span, mod_name, fn_name, arity)
        } else {
            not_implemented!(tcx, expr.span, "ZST literal", expr.ty)
        }
    }

    fn translate_assign(
        &mut self,
        expr: &ThirExpr<'tcx>,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let rhs = self.translate_expr(rhs);
        self.do_translate_assign(expr, lhs, rhs)
    }

    fn translate_assign_op(
        &mut self,
        expr: &ThirExpr<'tcx>,
        op: AssignOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let rhs = self.translate_binary(expr, op.into(), lhs, rhs);
        self.do_translate_assign(expr, lhs, rhs)
    }

    fn do_translate_assign(
        &mut self,
        expr: &ThirExpr<'tcx>,
        lhs: ExprId,
        rhs: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let lhs = self.resolve_expr(lhs);

        // In Patina, lhs of ExprKind::Assign can only ever be a var
        let var_id = match &lhs.kind {
            ExprKind::VarRef { id } => *id,

            ExprKind::UpvarRef { var_hir_id, .. } => {
                span_fatal!(
                    self.tcx(),
                    expr.span,
                    "cannot assign to variable {} from parent scope",
                    self.tcx().hir_ident(var_hir_id.0).to_string()
                )
            }

            kind => span_bug!(expr.span, "lhs of assign must be a var; got {kind:?}"),
        };

        let ident = self.tcx().hir_ident(var_id.0);
        let var_info = self.scope.add_var(var_id, ident);

        let lhs = afb.pat_var(expr.span, var_info.erl_name);
        afb.expr_match(expr.span, lhs, rhs)
    }

    fn translate_match(
        &mut self,
        expr: &ThirExpr<'tcx>,
        scrutinee: ExprId,
        arms: &[thir::ArmId],
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let scrutinee = self.translate_expr(scrutinee);
        let clauses = arms.iter().map(|&arm| self.translate_arm(arm));
        afb.expr_case(expr.span, scrutinee, clauses)
    }

    fn translate_arm(&mut self, arm_id: thir::ArmId) -> eaf::Clause<'arena> {
        let arm = &self.thir[arm_id];
        let pattern = self.translate_pattern(&arm.pattern);
        let guard = arm.guard.map(|guard| self.translate_expr(guard));
        let body = self.translate_expr(arm.body);
        let body = self.translator.afb.clause_body(body);
        self.translator.afb.clause(arm.span, [pattern], guard, body)
    }

    pub(super) fn translate_pattern(&mut self, pat: &ThirPat<'tcx>) -> eaf::Pattern<'arena> {
        match &pat.kind {
            PatKind::Wild => self.translator.afb.pat_var(pat.span, kw::Underscore),

            PatKind::Binding {
                name,
                mode,
                var,
                subpattern,
                is_primary,
                ty: _,
                is_shorthand: _,
            } => self.translate_binding_pattern(
                pat,
                *name,
                *mode,
                *var,
                subpattern.as_deref(),
                *is_primary,
            ),

            PatKind::Deref { subpattern, .. } => self.translate_pattern(subpattern),

            PatKind::Leaf { subpatterns } => self.translate_leaf_pattern(pat, subpatterns),

            PatKind::Variant { adt_def, variant_index, subpatterns, .. } => {
                let variant = adt_def.variant(*variant_index);
                self.translate_adt_pattern(pat.span, *adt_def, variant, subpatterns)
            }

            _ => not_implemented!(self.tcx(), pat.span, "THIR pattern kind", pat.kind),
        }
    }

    fn translate_binding_pattern(
        &mut self,
        pat: &ThirPat<'tcx>,
        name: Symbol,
        _mode: BindingMode,
        var: LocalVarId,
        subpattern: Option<&ThirPat<'tcx>>,
        is_primary: bool,
    ) -> eaf::Pattern<'arena> {
        let var_info = if is_primary {
            let ident = Ident::new(name, pat.span);
            self.scope.add_var(var, ident)
        } else {
            self.scope
                .get_var(var)
                .unwrap_or_else(|| span_bug!(pat.span, "variable {var:?} not in scope"))
        };

        let var_pat = self.translator.afb.pat_var(pat.span, var_info.erl_name);

        if let Some(sub) = subpattern {
            let sub_pat = self.translate_pattern(sub);
            self.translator.afb.pat_match(pat.span, var_pat, sub_pat)
        } else {
            var_pat
        }
    }

    fn translate_leaf_pattern(
        &mut self,
        pat: &ThirPat<'tcx>,
        subpatterns: &[rustc_middle::thir::FieldPat<'tcx>],
    ) -> eaf::Pattern<'arena> {
        match pat.ty.kind() {
            ty::TyKind::Tuple(_) => {
                let field_count = pat.ty.tuple_fields().len();
                let afb = &self.translator.afb;

                let elements = (0..field_count).map(|idx| {
                    match subpatterns.iter().find(|field_pat| field_pat.field.index() == idx) {
                        Some(field_pat) => self.translate_pattern(&field_pat.pattern),
                        None => afb.pat_var(pat.span, kw::Underscore),
                    }
                });

                afb.pat_tuple(pat.span, elements)
            }

            ty::TyKind::Adt(adt_def, _) => {
                let variant = adt_def.non_enum_variant();
                self.translate_adt_pattern(pat.span, *adt_def, variant, subpatterns)
            }

            _ => not_implemented!(self.tcx(), pat.span, "leaf pattern type", pat.ty),
        }
    }

    fn resolve_expr(&self, expr: ExprId) -> &'a ThirExpr<'tcx> {
        let mut expr = &self.thir[expr];
        loop {
            match &expr.kind {
                ExprKind::Scope { value, .. } => expr = &self.thir[*value],
                ExprKind::Use { source }
                | ExprKind::ByUse { expr: source, .. }
                | ExprKind::NeverToAny { source }
                | ExprKind::Cast { source, .. }
                | ExprKind::PointerCoercion { source, .. }
                | ExprKind::PlaceTypeAscription { source, .. }
                | ExprKind::ValueTypeAscription { source, .. } => expr = &self.thir[*source],

                _ => return expr,
            }
        }
    }
}
