use rustc_abi::{FieldIdx, VariantIdx};
use rustc_ast::LitKind;
use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{BindingMode, HirId, Lit};
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution, ErlRepr};
use rustc_middle::middle::region;
use rustc_middle::mir::{AssignOp, BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::{
    self, Arm, BlockId, Expr, ExprId, ExprKind, LocalVarId, MapFieldPat, Pat, PatKind, StmtId,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::translate::loops::Loops;
use crate::compile::translate::scopes::{PatinaVar, Scope, Scopes};
use crate::compile::translate::{ModuleParts, Translator, Witness};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

/// Pattern with optional guards needed to match it.
#[derive(Debug, Clone)]
pub(super) struct GuardedPattern<'arena>(eaf::Pattern<'arena>, SmallVec<[eaf::Expr<'arena>; 1]>);
pub(super) type Patterns<'arena> = SmallVec<[GuardedPattern<'arena>; 1]>;

impl<'arena> From<eaf::Pattern<'arena>> for GuardedPattern<'arena> {
    fn from(pattern: eaf::Pattern<'arena>) -> Self {
        Self(pattern, SmallVec::new())
    }
}

impl<'arena> From<GuardedPattern<'arena>> for Patterns<'arena> {
    fn from(val: GuardedPattern<'arena>) -> Self {
        SmallVec::from_elem(val, 1)
    }
}

impl<'arena> From<eaf::Pattern<'arena>> for Patterns<'arena> {
    fn from(val: eaf::Pattern<'arena>) -> Self {
        Into::<GuardedPattern<'arena>>::into(val).into()
    }
}

pub(crate) struct TranslatedBody<'arena> {
    pub params: SmallVec<[eaf::Pattern<'arena>; 8]>,
    pub value: eaf::Expr<'arena>,
}

struct TranslatedArm<'arena> {
    span: Span,
    patterns: Patterns<'arena>,
    guard: Option<eaf::Expr<'arena>>,
    body: eaf::Expr<'arena>,
    scope: Scope,
    always_diverges: bool,
}

pub(super) struct BodyCx<'a, 'tcx, 'arena> {
    pub translator: &'a Translator<'tcx, 'arena>,
    pub owner: LocalDefId,
    pub thir: &'a thir::Thir<'tcx>,
    pub parts: &'a mut ModuleParts<'arena>,
    pub scopes: &'a mut Scopes,
    pub loops: Loops,
    pub witness_vars: &'a FxIndexMap<ty::ParamTy, Symbol>,
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn new(
        translator: &'a Translator<'tcx, 'arena>,
        owner: LocalDefId,
        thir: &'a thir::Thir<'tcx>,
        parts: &'a mut ModuleParts<'arena>,
        scopes: &'a mut Scopes,
        witness_vars: &'a FxIndexMap<ty::ParamTy, Symbol>,
    ) -> Self {
        BodyCx { translator, owner, thir, parts, scopes, witness_vars, loops: Loops::new() }
    }

    pub(super) fn tcx(&self) -> TyCtxt<'tcx> {
        self.translator.tcx
    }

    pub(super) fn translate_expr(&mut self, expr_id: ExprId) -> eaf::Expr<'arena> {
        let (expr, region_scope) = self.resolve_expr_with_scope(expr_id);

        match &expr.kind {
            ExprKind::Block { block } => self.translate_block(*block),

            ExprKind::Literal { lit, neg } => self.translate_literal(expr.span, lit, *neg),

            ExprKind::VarRef { id } => self.translate_var_ref(expr.span, *id),
            ExprKind::UpvarRef { var_hir_id, .. } => self.translate_var_ref(expr.span, *var_hir_id),

            ExprKind::Array { fields, tail } => self.translate_list(expr.span, fields, *tail),

            ExprKind::Map { fields, base } => self.translate_map(expr.span, fields, *base),

            ExprKind::Tuple { fields } => self.translate_tuple(expr.span, fields),

            ExprKind::Field { lhs, variant_index, name } => {
                self.translate_field(expr, *lhs, *variant_index, *name)
            }

            ExprKind::Binary { op, lhs, rhs } => self.translate_binary(expr, *op, *lhs, *rhs),

            ExprKind::LogicalOp { op, lhs, rhs } => self.translate_logical(expr, *op, *lhs, *rhs),

            ExprKind::Unary { op, arg } => self.translate_unary(expr, *op, *arg),

            ExprKind::Cast { source } => self.translate_cast(expr, *source),

            ExprKind::Call { ty, fun, args, forced_extern, .. } => {
                self.translate_call(expr.span, *ty, *fun, args, *forced_extern)
            }

            ExprKind::Adt(adt_expr) => self.translate_adt(expr, adt_expr),

            ExprKind::Closure(closure) => self.translate_closure(expr, closure),

            ExprKind::ZstLiteral { .. } => self.translate_zst_literal(expr),

            ExprKind::NamedConst { def_id, args, .. } => {
                self.translate_named_const(expr.span, *def_id, args)
            }

            ExprKind::Assign { lhs, rhs } => self.translate_assign(expr, *lhs, *rhs),
            ExprKind::AssignOp { op, lhs, rhs } => self.translate_assign_op(expr, *op, *lhs, *rhs),

            ExprKind::If { if_then_scope: _, cond, then, else_opt } => {
                self.translate_if(expr, *cond, *then, *else_opt)
            }
            ExprKind::Match { scrutinee, arms, .. } => self.translate_match(expr, *scrutinee, arms),

            ExprKind::Return { value } => self.translate_return(expr.span, *value),
            ExprKind::Loop { body } => self.translate_loop(expr_id, expr, *body, region_scope),
            ExprKind::Break { label, value } => self.translate_break(expr.span, *label, *value),
            ExprKind::Continue { label } => self.translate_continue(expr.span, *label),

            _ => not_implemented!(self.tcx(), expr.span, "THIR expression kind", expr.kind),
        }
    }

    fn translate_list(
        &mut self,
        span: Span,
        fields: &[ExprId],
        tail: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let tail = tail.map(|id| self.translate_expr(id));
        let items = fields.iter().map(|&id| self.translate_expr(id));
        afb.expr_list(span, items, tail)
    }

    fn translate_map(
        &mut self,
        span: Span,
        fields: &[thir::MapFieldExpr],
        base: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let base_expr = base.map(|base_id| self.translate_expr(base_id));

        let assocs = fields.iter().map(|field| {
            let key = self.translate_expr(field.key);
            let value = self.translate_expr(field.value);
            (key, value)
        });

        if let Some(base_expr) = base_expr {
            afb.expr_map_update_assoc(span, base_expr, assocs)
        } else {
            afb.expr_map(span, assocs)
        }
    }

    fn translate_tuple(&mut self, span: Span, fields: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let elems = fields.iter().map(|&id| self.translate_expr(id));
        afb.expr_tuple(span, elems)
    }

    fn translate_cast(&mut self, expr: &Expr<'tcx>, source: ExprId) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let source_ty = self.thir[source].ty;
        let source = self.translate_expr(source);

        let function = if source_ty.is_floating_point() && expr.ty.is_integral() {
            sym::trunc
        } else if source_ty.is_integral() && expr.ty.is_floating_point() {
            sym::float
        } else {
            return source;
        };

        afb.expr_remote_call(
            expr.span,
            afb.expr_atom(expr.span, sym::erlang),
            afb.expr_atom(expr.span, function),
            [source],
        )
    }

    fn translate_named_const(
        &mut self,
        span: Span,
        def_id: rustc_hir::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> eaf::Expr<'arena> {
        if !args.is_empty() {
            not_implemented!(self.tcx(), span, "generic named constants", args);
        }

        let Some(local_def_id) = def_id.as_local() else {
            not_implemented!(self.tcx(), span, "external named constants", def_id);
        };

        self.translator.translate_const(span, local_def_id, self.parts)
    }

    fn translate_literal(&self, span: Span, lit: &Lit, neg: bool) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let value = match lit.node {
            LitKind::Str(symbol, _style) => afb.expr_binary_string(lit.span, symbol),
            LitKind::Atom(symbol, _style) => afb.expr_atom(lit.span, symbol),
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

    fn translate_pin_var_ref(&self, span: Span, id: LocalVarId) -> eaf::Pattern<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.pat_var(span, var.erl_name)
    }

    fn translate_var_ref(&self, span: Span, id: LocalVarId) -> eaf::Expr<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.expr_var(span, var.erl_name)
    }

    fn var_for_read(&self, span: Span, id: LocalVarId) -> PatinaVar {
        let ident = self.tcx().hir_ident(id.0);

        *self.scopes.get_var(id).filter(|var| var.initialization_depth.is_some()).unwrap_or_else(
            || {
                // TODO: this should be `report_use_of_uninitialized` from rustc
                span_fatal!(
                    self.tcx(),
                    span,
                    "variable {} is partially initialized",
                    ident.to_string()
                )
            },
        )
    }

    fn translate_field(
        &mut self,
        expr: &Expr<'tcx>,
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
        let repr = self.tcx().erl_variant_repr(variant.def_id);
        let subject = self.translate_expr(lhs);
        let afb = &self.translator.afb;

        match repr.kind {
            ErlRepr::Elixir => {
                let field_name = afb.expr_atom(lhs_expr.span, variant.fields[name].name);
                afb.expr_remote_call(
                    expr.span,
                    afb.expr_atom(expr.span, sym::erlang),
                    afb.expr_atom(expr.span, sym::map_get),
                    [field_name, subject],
                )
            }
            ErlRepr::Tagged => afb.expr_remote_call(
                expr.span,
                afb.expr_atom(expr.span, sym::erlang),
                afb.expr_atom(expr.span, sym::element),
                [afb.expr_int(expr.span, name.index() as u128 + 2), subject],
            ),
            ErlRepr::Transparent => subject,
            ErlRepr::NativeRecord => afb.expr_record_field(
                expr.span,
                subject,
                self.record_module_for_access(repr.record_module),
                repr.tag,
                variant.fields[name].name,
            ),
            ErlRepr::Atom => span_bug!(expr.span, "field access on atom-represented ADT"),
        }
    }

    fn translate_block(&mut self, id: BlockId) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let block = &self.thir[id];

        let mut exprs: Vec<eaf::Expr<'arena>> = Vec::new();
        let mut last_stmt_guaranteed_evaluates_to_unit = false;

        for &stmt_id in block.stmts.iter() {
            if let Some((expr, guaranteed_evaluates_to_unit)) = self.translate_stmt(stmt_id) {
                exprs.push(expr);
                last_stmt_guaranteed_evaluates_to_unit = guaranteed_evaluates_to_unit;
            }
        }

        if let Some(tail) = block.expr {
            exprs.push(self.translate_expr(tail));
        } else if !last_stmt_guaranteed_evaluates_to_unit {
            exprs.push(afb.expr_unit(block.span));
        }

        afb.expr_block(block.span, exprs)
    }

    // Returns the expression and whether it's guaranteed to evaluate to unit
    fn translate_stmt(&mut self, id: StmtId) -> Option<(eaf::Expr<'arena>, bool)> {
        let stmt = &self.thir[id];
        let afb = &self.translator.afb;

        match stmt.kind {
            thir::StmtKind::Expr { expr, .. } => {
                Some((self.translate_expr(expr), self.thir[expr].ty.is_unit()))
            }

            thir::StmtKind::Let { ref pattern, initializer, else_block, span, .. } => {
                let Some(initializer) = initializer else {
                    self.declare_vars_from_pattern(pattern);
                    return None;
                };
                let rhs = self.translate_expr(initializer);

                let Some(else_block) = else_block else {
                    let lhs = self.translate_single_pattern(pattern);
                    return Some((afb.expr_match(span, lhs, rhs), false));
                };

                self.declare_vars_from_pattern(pattern);

                let else_span = self.thir[else_block].span;
                let ((success_patterns, success_body), success_scope) = self.with_scope(|cx| {
                    let patterns = cx.translate_pattern(pattern);
                    let body = afb.expr_unit(span);
                    (patterns, body)
                });
                let (else_body, else_scope) = self.with_scope(|cx| cx.translate_block(else_block));

                let mut translated = SmallVec::new();
                translated.push(TranslatedArm {
                    span,
                    patterns: success_patterns,
                    guard: None,
                    body: success_body,
                    scope: success_scope,
                    always_diverges: false,
                });
                let else_pattern = afb.pat_var(else_span, kw::Underscore);
                translated.push(TranslatedArm {
                    span: else_span,
                    patterns: else_pattern.into(),
                    guard: None,
                    body: else_body,
                    scope: else_scope,
                    always_diverges: true,
                });

                Some((self.translate_to_case(span, rhs, translated), true))
            }
        }
    }

    fn translate_binary(
        &mut self,
        expr: &Expr<'tcx>,
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
        expr: &Expr<'tcx>,
        op: rustc_middle::thir::LogicalOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let left = self.translate_expr(lhs);

        let (right, right_scope) = self.with_scope(|cx| cx.translate_expr(rhs));
        let right_always_diverges = self.is_always_diverging_expr(rhs);

        let short_scope = Scope::new();

        let changed_vars = self
            .scopes
            .modified_outer_vars([(&right_scope, right_always_diverges), (&short_scope, false)]);

        if changed_vars.is_empty() {
            self.scopes.join_branches(&changed_vars);
            let operator = match op {
                rustc_middle::thir::LogicalOp::And => eaf::BinaryOp::Andalso,
                rustc_middle::thir::LogicalOp::Or => eaf::BinaryOp::Orelse,
            };
            return self.translator.afb.expr_binary_op(expr.span, operator, left, right);
        }

        let short_expr = match op {
            rustc_middle::thir::LogicalOp::And => afb.expr_atom(expr.span, kw::False),
            rustc_middle::thir::LogicalOp::Or => afb.expr_atom(expr.span, kw::True),
        };
        let short =
            self.bind_branch_outer_variables(expr.span, short_expr, &short_scope, &changed_vars);

        let right = self.bind_branch_outer_variables(
            self.resolve_expr(rhs).span,
            right,
            &right_scope,
            &changed_vars,
        );

        self.scopes.join_branches(&changed_vars);

        let true_clause = afb.clause(
            expr.span,
            [afb.pat_atom(expr.span, kw::True)],
            None,
            match op {
                rustc_middle::thir::LogicalOp::And => right,
                rustc_middle::thir::LogicalOp::Or => short,
            },
        );
        let false_clause = afb.clause(
            expr.span,
            [afb.pat_atom(expr.span, kw::False)],
            None,
            match op {
                rustc_middle::thir::LogicalOp::And => short,
                rustc_middle::thir::LogicalOp::Or => right,
            },
        );

        afb.expr_case(expr.span, left, [true_clause, false_clause])
    }

    fn translate_unary(&mut self, expr: &Expr<'tcx>, op: UnOp, arg: ExprId) -> eaf::Expr<'arena> {
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

    fn translate_return(&mut self, span: Span, value: Option<ExprId>) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let value =
            value.map(|value| self.translate_expr(value)).unwrap_or_else(|| afb.expr_unit(span));

        if self.loops.in_loop() {
            self.loops.record_return();
            self.loop_return_expr(span, value, self.loops.current())
        } else {
            self.marker_call(span, kw::Return, [value])
        }
    }

    pub(super) fn marker_call(
        &self,
        span: Span,
        name: Symbol,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::dollar_patina_marker),
            afb.expr_atom(span, name),
            args,
        )
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

        let (erl_call_target, callee_expr) = if let ty::TyKind::FnDef(callee_def_id, generic_args) =
            callable_ty.kind()
        {
            if self.tcx().is_fn_trait(self.tcx().parent(*callee_def_id)) {
                return self.translate_fn_trait_call(span, args);
            }

            let erl_call_target = self.tcx().erl_call_target_for_def(*callee_def_id, generic_args);
            (erl_call_target, afb.expr_atom(span, self.tcx().erl_fn_name(*callee_def_id)))
        } else if let ExprKind::ExternMod { def_id } = &self.resolve_expr(fun).kind {
            let erl_call_target =
                self.tcx().erl_call_target_for_def(*def_id, ty::GenericArgs::empty());
            (erl_call_target, afb.expr_atom(span, self.tcx().item_name(*def_id)))
        } else {
            (ErlCallTarget::Dynamic, self.translate_expr(fun))
        };

        let mut arg_exprs = self.call_witness_exprs(span, erl_call_target);
        arg_exprs.extend(args.iter().map(|&id| self.translate_expr(id)));

        let Some(call_kind) =
            self.tcx().erl_call_kind(self.owner.to_def_id(), erl_call_target, Some(forced_extern))
        else {
            return self.translate_call_via_dispatch_through_type_parameter(
                span,
                erl_call_target,
                arg_exprs,
            );
        };

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

        let mod_name = self.tcx().erl_module_name(module_id);

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

    fn translate_call_via_dispatch_through_type_parameter(
        &mut self,
        span: Span,
        erl_call_target: ErlCallTarget<'tcx>,
        arg_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let ErlCallTarget::Item { def_id, .. } = erl_call_target else {
            span_bug!(span, "dispatch through a type parameter must have an item target");
        };

        let self_witness = arg_exprs
            .first()
            .unwrap_or_else(|| span_bug!(span, "dispatch must have a receiver witness"));

        let element_call = afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::erlang),
            afb.expr_atom(span, sym::element),
            [afb.expr_int(span, 1), *self_witness],
        );

        afb.expr_remote_call(
            span,
            element_call,
            afb.expr_atom(span, self.tcx().erl_fn_name(def_id)),
            arg_exprs,
        )
    }

    /// The witness arguments a call site must pass to `target`, in order.
    fn call_witness_exprs(
        &self,
        span: Span,
        target: ErlCallTarget<'tcx>,
    ) -> SmallVec<[eaf::Expr<'arena>; 8]> {
        let ErlCallTarget::Item { def_id, args } = target else {
            return SmallVec::new();
        };
        self.tcx()
            .erl_call_witness_tys(def_id, args)
            .into_iter()
            .map(|ty| {
                let witness = self.translator.build_witness(self.owner.to_def_id(), span, ty);
                self.materialize_witness(span, witness)
            })
            .collect()
    }

    fn materialize_witness(&self, span: Span, witness: Witness) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        match witness {
            Witness::Param(param) => match self.witness_vars.get(&param) {
                Some(&var) => afb.expr_var(span, var),
                None => afb.expr_atom(span, sym::dollar_patina_unbounded_type),
            },
            Witness::ModuleWithSubtypes { module, subtypes } => {
                let subtype_exprs =
                    subtypes.into_iter().map(|subtype| self.materialize_witness(span, subtype));
                let subtypes_expr_tuple = afb.expr_tuple(span, subtype_exprs);
                afb.expr_tuple(span, [afb.expr_atom(span, module), subtypes_expr_tuple])
            }
        }
    }

    fn translate_fn_trait_call(&mut self, span: Span, args: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let &[callee_id, tuple_id] = args else {
            span_bug!(span, "expected 2 arguments for fn trait call");
        };

        let ExprKind::Tuple { fields } = &self.resolve_expr(tuple_id).kind else {
            span_bug!(span, "expected tuple for fn trait call")
        };

        let callee_expr = self.translate_expr(callee_id);
        let call_arg_exprs = fields.iter().map(|&field_id| self.translate_expr(field_id));

        afb.expr_local_call(span, callee_expr, call_arg_exprs)
    }

    fn translate_format_render(
        &mut self,
        span: Span,
        fn_name: Symbol,
        arg: ExprId,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let (_, Some((_, hir_id))) = self.resolve_expr_with_scope(arg) else {
            span_bug!(span, "format argument must be a HIR-derived expression");
        };
        let ty = self.tcx().typeck(self.owner).node_type(hir_id);

        let translated = self.translate_expr(arg);
        if fn_name == sym::display && ty.is_str() {
            return translated;
        }

        let format_str = self.translator.afb.expr_string(span, sym::debug_format_str);
        let args_list = self.translator.afb.expr_list(span, [translated], None);

        afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::io_lib),
            afb.expr_atom(span, sym::format),
            [format_str, args_list],
        )
    }

    fn translate_closure(
        &mut self,
        expr: &Expr<'tcx>,
        closure: &thir::ClosureExpr<'tcx>,
    ) -> eaf::Expr<'arena> {
        let translated = self
            .with_scope(|cx| {
                cx.translator.translate_body(
                    closure.closure_id,
                    cx.parts,
                    cx.scopes,
                    cx.witness_vars,
                )
            })
            .0;

        let afb = &self.translator.afb;
        let clause = afb.clause(expr.span, translated.params, None, translated.value);
        afb.expr_fun(expr.span, [clause])
    }

    fn translate_zst_literal(&mut self, expr: &Expr<'tcx>) -> eaf::Expr<'arena> {
        let tcx = self.tcx();
        let afb = &self.translator.afb;

        if let ty::TyKind::FnDef(def_id, args) = expr.ty.kind()
            && matches!(tcx.def_kind(*def_id), DefKind::Fn | DefKind::AssocFn)
        {
            let target = tcx.erl_call_target_for_def(*def_id, args);

            let Some(call_kind) = tcx.erl_call_kind(self.owner.to_def_id(), target, None) else {
                let receiver = tcx.erl_call_target_self_ty(target).unwrap_or_else(|| {
                    span_bug!(expr.span, "dispatch through a type parameter must have a receiver")
                });
                not_implemented!(tcx, expr.span, "dispatch through a type parameter", receiver);
            };

            let fn_name = tcx.erl_fn_name(*def_id);
            let arity = tcx.fn_arg_idents(*def_id).len() as u32;

            match call_kind {
                ErlCallKind::Local | ErlCallKind::LocalOrExtern => {
                    afb.expr_local_fun(expr.span, fn_name, arity)
                }
                ErlCallKind::Extern => {
                    let module_resolution =
                        tcx.erl_module_for_target(self.owner.to_def_id(), target).unwrap_or_else(
                            || span_bug!(expr.span, "external function item must have a module"),
                        );
                    let ErlModuleResolution::Static(module_id) = module_resolution else {
                        span_bug!(expr.span, "external function item must have a static module");
                    };
                    let mod_name = tcx.erl_module_name(module_id);
                    afb.expr_remote_fun(expr.span, mod_name, fn_name, arity)
                }
                ErlCallKind::Dynamic => {
                    not_implemented!(tcx, expr.span, "dynamic function-item value", expr.ty)
                }
            }
        } else {
            not_implemented!(tcx, expr.span, "ZST literal", expr.ty)
        }
    }

    fn translate_assign(
        &mut self,
        expr: &Expr<'tcx>,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let rhs = self.translate_expr(rhs);
        self.do_translate_assign(expr, lhs, rhs)
    }

    fn translate_assign_op(
        &mut self,
        expr: &Expr<'tcx>,
        op: AssignOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let rhs = self.translate_binary(expr, op.into(), lhs, rhs);
        self.do_translate_assign(expr, lhs, rhs)
    }

    fn do_translate_assign(
        &mut self,
        expr: &Expr<'tcx>,
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
        let var_info = self.scopes.bind_var(var_id, ident.name, true);

        let lhs = afb.pat_var(expr.span, var_info.erl_name);
        afb.expr_match(expr.span, lhs, rhs)
    }

    fn translate_if(
        &mut self,
        expr: &Expr<'tcx>,
        cond: ExprId,
        then: ExprId,
        else_opt: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        if let ExprKind::Let { expr: scrutinee, pat } = &self.resolve_expr(cond).kind {
            let scrutinee = self.translate_expr(*scrutinee);
            let then_span = self.resolve_expr(then).span;
            let ((then_patterns, then_body, then_always_diverges), then_scope) =
                self.with_scope(|cx| {
                    let patterns = cx.translate_pattern(pat);
                    let always_diverges = cx.is_always_diverging_expr(then);
                    let body = cx.translate_expr(then);
                    (patterns, body, always_diverges)
                });

            let ((else_body, else_span, else_always_diverges), else_scope) =
                self.with_scope(|cx| match else_opt {
                    Some(else_expr) => (
                        cx.translate_expr(else_expr),
                        cx.resolve_expr(else_expr).span,
                        cx.is_always_diverging_expr(else_expr),
                    ),
                    None => (cx.translator.afb.expr_unit(expr.span), expr.span, false),
                });

            let mut translated = SmallVec::new();
            translated.push(TranslatedArm {
                span: then_span,
                patterns: then_patterns,
                guard: None,
                body: then_body,
                scope: then_scope,
                always_diverges: then_always_diverges,
            });
            let else_pattern = self.translator.afb.pat_var(else_span, kw::Underscore);
            translated.push(TranslatedArm {
                span: else_span,
                patterns: else_pattern.into(),
                guard: None,
                body: else_body,
                scope: else_scope,
                always_diverges: else_always_diverges,
            });
            return self.translate_to_case(expr.span, scrutinee, translated);
        }

        let cond = self.translate_expr(cond);

        let (then_body, then_scope) = self.with_scope(|cx| cx.translate_expr(then));
        let then_span = self.resolve_expr(then).span;
        let then_always_diverges = self.is_always_diverging_expr(then);

        let ((else_body, else_span, else_always_diverges), else_scope) =
            self.with_scope(|cx| match else_opt {
                Some(else_expr) => (
                    cx.translate_expr(else_expr),
                    cx.resolve_expr(else_expr).span,
                    cx.is_always_diverging_expr(else_expr),
                ),
                None => (cx.translator.afb.expr_unit(expr.span), expr.span, false),
            });

        let changed_vars = self.scopes.modified_outer_vars([
            (&then_scope, then_always_diverges),
            (&else_scope, else_always_diverges),
        ]);
        let then_body =
            self.bind_branch_outer_variables(then_span, then_body, &then_scope, &changed_vars);
        let else_body =
            self.bind_branch_outer_variables(else_span, else_body, &else_scope, &changed_vars);

        self.scopes.join_branches(&changed_vars);

        let afb = &self.translator.afb;
        let then_pat = afb.pat_atom(expr.span, kw::True);
        let then_clause = afb.clause(expr.span, [then_pat], None, then_body);
        let else_pat = afb.pat_atom(expr.span, kw::False);
        let else_clause = afb.clause(expr.span, [else_pat], None, else_body);

        afb.expr_case(expr.span, cond, [then_clause, else_clause])
    }

    fn translate_match(
        &mut self,
        expr: &Expr<'tcx>,
        scrutinee: ExprId,
        arms: &[thir::ArmId],
    ) -> eaf::Expr<'arena> {
        let scrutinee = self.translate_expr(scrutinee);

        let mut translated: SmallVec<[TranslatedArm<'arena>; 4]> = SmallVec::new();
        for &arm_id in arms {
            self.translate_arm(arm_id, &mut translated);
        }

        self.translate_to_case(expr.span, scrutinee, translated)
    }

    fn translate_to_case(
        &mut self,
        span: Span,
        scrutinee: eaf::Expr<'arena>,
        mut translated: SmallVec<[TranslatedArm<'arena>; 4]>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let changed_vars = self
            .scopes
            .modified_outer_vars(translated.iter().map(|arm| (&arm.scope, arm.always_diverges)));

        for arm in &mut translated {
            arm.body =
                self.bind_branch_outer_variables(arm.span, arm.body, &arm.scope, &changed_vars);
        }

        self.scopes.join_branches(&changed_vars);

        let clauses = translated.into_iter().flat_map(|arm| {
            let TranslatedArm { span, patterns, guard, body, .. } = arm;
            patterns.into_iter().map(move |GuardedPattern(pattern, pattern_guards)| {
                afb.clause(span, [pattern], guard.into_iter().chain(pattern_guards), body)
            })
        });
        afb.expr_case(span, scrutinee, clauses)
    }

    fn translate_arm(
        &mut self,
        arm_id: thir::ArmId,
        translated: &mut SmallVec<[TranslatedArm<'arena>; 4]>,
    ) {
        let thir = self.thir;
        let Arm { span, pattern, guard, body, hir_id: _, scope: _ } = &thir[arm_id];
        let ((patterns, guard, body, always_diverges), scope) = self.with_scope(|cx| {
            let patterns = cx.translate_pattern(pattern);
            let guard = guard.map(|guard| cx.translate_expr(guard));
            let always_diverges = cx.is_always_diverging_expr(*body);
            let body = cx.translate_expr(*body);
            (patterns, guard, body, always_diverges)
        });
        translated.push(TranslatedArm {
            span: *span,
            patterns,
            guard,
            body,
            scope,
            always_diverges,
        });
    }

    /// Makes sure the latest version of every modified outer variable is bound by the branch.
    fn bind_branch_outer_variables(
        &mut self,
        span: Span,
        body: eaf::Expr<'arena>,
        branch_scope: &Scope,
        changed_vars: &FxIndexMap<LocalVarId, PatinaVar>,
    ) -> eaf::Expr<'arena> {
        // All vars are already bound by this branch at the latest version.
        if changed_vars.iter().all(|(var_id, scope_var)| matches!(branch_scope.get_var(*var_id), Some(branch_var) if branch_var.vsn == scope_var.vsn)) {
            return body;
        }

        let afb = &self.translator.afb;

        let result_var = self.scopes.fresh_var(sym::branch_value, true);

        let result_var_pat = afb.pat_var(span, result_var.erl_name);
        let result_var_expr = afb.expr_var(span, result_var.erl_name);
        let result_var_match = afb.expr_match(span, result_var_pat, body);

        let exports = changed_vars.iter().flat_map(|(id, export_var)| {
            let branch_var = branch_scope
                .get_var(*id)
                .or_else(|| self.scopes.get_var(*id))
                .unwrap_or_else(|| span_bug!(span, "variable {id:?} not in scope"));

            if branch_var.vsn < export_var.vsn {
                let branch_var_expr = if branch_var.initialized() {
                    afb.expr_var(span, branch_var.erl_name)
                } else {
                    afb.expr_unit(span)
                };
                let export_var_pat = afb.pat_var(span, export_var.erl_name);
                Some(afb.expr_match(span, export_var_pat, branch_var_expr))
            } else {
                None
            }
        });

        afb.expr_block(
            span,
            std::iter::once(result_var_match)
                .chain(exports)
                .chain(std::iter::once(result_var_expr)),
        )
    }

    pub(super) fn translate_single_pattern(&mut self, pat: &Pat<'tcx>) -> eaf::Pattern<'arena> {
        let patterns = self.translate_pattern(pat);
        if let [GuardedPattern(pattern, guards)] = patterns.as_slice()
            && guards.is_empty()
        {
            *pattern
        } else {
            span_bug!(
                pat.span,
                "pattern expanded to multiple patterns or guards in a singleton context"
            )
        }
    }

    pub(super) fn translate_pattern(&mut self, pat: &Pat<'tcx>) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        match &pat.kind {
            PatKind::Or { pats } => {
                pats.iter().flat_map(|pattern| self.translate_pattern(pattern)).collect()
            }

            PatKind::Range(range) => self.translate_range_pattern(pat.span, range),

            PatKind::Wild => afb.pat_var(pat.span, kw::Underscore).into(),

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

            PatKind::Pin { var } => self.translate_pin_var_ref(pat.span, *var).into(),

            PatKind::Deref { subpattern, .. } => self.translate_pattern(subpattern),

            PatKind::Leaf { subpatterns } => self.translate_leaf_pattern(pat, subpatterns),

            PatKind::Variant { adt_def, variant_index, subpatterns, .. } => {
                let variant = adt_def.variant(*variant_index);
                self.translate_adt_pattern(pat.span, *adt_def, variant, subpatterns)
            }

            PatKind::Constant { value } => self.translate_constant_pattern(pat.span, *value).into(),

            PatKind::Map { fields } => self.translate_map_pattern(pat.span, fields),

            PatKind::Slice { prefix, slice, suffix } => {
                self.translate_slice_pattern(pat.span, prefix, slice.as_deref(), suffix)
            }

            _ => not_implemented!(self.tcx(), pat.span, "THIR pattern kind", pat.kind),
        }
    }

    fn translate_map_pattern(
        &mut self,
        span: Span,
        fields: &[MapFieldPat<'tcx>],
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let keys: SmallVec<[_; 4]> =
            fields.iter().map(|field| self.translate_single_pattern(&field.key)).collect();
        let groups: SmallVec<[_; 4]> =
            fields.iter().map(|field| self.translate_pattern(&field.pattern)).collect();

        self.pattern_cartesian_product(span, &groups, |values| {
            afb.pat_map(span, keys.iter().copied().zip(values.iter().copied()))
        })
    }

    fn translate_slice_pattern(
        &mut self,
        span: Span,
        prefix: &[Pat<'tcx>],
        slice: Option<&Pat<'tcx>>,
        suffix: &[Pat<'tcx>],
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        if !suffix.is_empty() {
            span_bug!(span, "list rest pattern must be trailing")
        }

        let tail = slice.map(|pattern| self.translate_pattern(pattern));
        let groups: SmallVec<[_; 4]> =
            prefix.iter().map(|pattern| self.translate_pattern(pattern)).chain(tail).collect();

        self.pattern_cartesian_product(span, &groups, |patterns| {
            let (items, tail) = if slice.is_some() {
                let (tail, items) = patterns.split_last().unwrap();
                (items, Some(*tail))
            } else {
                (patterns, None)
            };
            afb.pat_list(span, items.iter().copied(), tail)
        })
    }

    fn declare_vars_from_pattern(&mut self, pat: &Pat<'tcx>) {
        match &pat.kind {
            PatKind::Binding { var, subpattern, is_primary, .. } => {
                if *is_primary {
                    let ident = self.tcx().hir_ident(var.0);
                    self.scopes.bind_var(*var, ident.name, false);
                }
                if let Some(subpattern) = subpattern {
                    self.declare_vars_from_pattern(subpattern);
                }
            }
            PatKind::Deref { subpattern, .. }
            | PatKind::DerefPattern { subpattern, .. }
            | PatKind::Guard { subpattern, .. } => self.declare_vars_from_pattern(subpattern),
            PatKind::Leaf { subpatterns } | PatKind::Variant { subpatterns, .. } => {
                for field in subpatterns {
                    self.declare_vars_from_pattern(&field.pattern);
                }
            }
            PatKind::Slice { prefix, slice, suffix } | PatKind::Array { prefix, slice, suffix } => {
                for pattern in prefix.iter().chain(slice.as_deref()).chain(suffix.iter()) {
                    self.declare_vars_from_pattern(pattern);
                }
            }
            PatKind::Or { pats } => {
                for pattern in pats {
                    self.declare_vars_from_pattern(pattern);
                }
            }
            PatKind::Map { fields, .. } => {
                for field in fields {
                    self.declare_vars_from_pattern(&field.pattern);
                }
            }
            PatKind::Missing
            | PatKind::Wild
            | PatKind::Pin { .. }
            | PatKind::Constant { .. }
            | PatKind::Range(_)
            | PatKind::Never
            | PatKind::Error(_) => {}
        }
    }

    fn translate_constant_pattern(
        &self,
        span: Span,
        value: ty::Value<'tcx>,
    ) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;
        match *value.ty.kind() {
            ty::TyKind::Bool => afb.pat_atom(
                span,
                if value
                    .try_to_bool()
                    .unwrap_or_else(|| span_bug!(span, "invalid boolean pattern constant"))
                {
                    kw::True
                } else {
                    kw::False
                },
            ),
            ty::TyKind::Char => {
                let character = char::try_from(value.to_leaf())
                    .unwrap_or_else(|_| span_bug!(span, "invalid character pattern constant"));
                afb.pat_char(span, character)
            }
            ty::TyKind::Int => {
                let int = value.to_leaf().to_i128();
                if int < 0 {
                    let magnitude = int.unsigned_abs();
                    let pat = afb.pat_int(span, magnitude);
                    afb.pat_unary_op(span, eaf::UnaryOp::Neg, pat)
                } else {
                    afb.pat_int(span, int as u128)
                }
            }
            ty::TyKind::Float => afb.pat_float(span, f64::from_bits(value.to_leaf().to_u64())),
            ty::TyKind::Str => {
                let bytes = value.try_to_raw_bytes(self.tcx()).unwrap_or_else(|| {
                    span_bug!(span, "string pattern did not lower to raw bytes")
                });
                afb.pat_binary_bytes(span, bytes)
            }
            ty::TyKind::Atom => {
                let bytes = value
                    .try_to_raw_bytes(self.tcx())
                    .unwrap_or_else(|| span_bug!(span, "atom pattern did not lower to raw bytes"));
                let sym = Symbol::intern(
                    std::str::from_utf8(bytes)
                        .unwrap_or_else(|_| span_bug!(span, "atom pattern is not valid UTF-8")),
                );
                afb.pat_atom(span, sym)
            }
            _ => not_implemented!(self.tcx(), span, "constant pattern type", value.ty),
        }
    }

    fn translate_range_pattern(
        &mut self,
        span: Span,
        range: &thir::PatRange<'tcx>,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let range_var = self.scopes.fresh_var(sym::range, true);

        let lower = range.lo.as_finite().map(|value| {
            let lhs = afb.expr_var(span, range_var.erl_name);
            let rhs = self.translate_range_boundary(span, range.ty, value);
            afb.expr_binary_op(span, eaf::BinaryOp::Ge, lhs, rhs)
        });
        let upper = range.hi.as_finite().map(|value| {
            let lhs = afb.expr_var(span, range_var.erl_name);
            let rhs = self.translate_range_boundary(span, range.ty, value);
            let op = match range.end {
                rustc_hir::RangeEnd::Included => eaf::BinaryOp::Le,
                rustc_hir::RangeEnd::Excluded => eaf::BinaryOp::Lt,
            };
            afb.expr_binary_op(span, op, lhs, rhs)
        });

        GuardedPattern(
            afb.pat_var(span, range_var.erl_name),
            lower.into_iter().chain(upper).collect(),
        )
        .into()
    }

    fn translate_range_boundary(
        &self,
        span: Span,
        ty: Ty<'tcx>,
        value: ty::ValTree<'tcx>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        match ty.kind() {
            ty::TyKind::Int => {
                let value = value.to_leaf().to_i128();
                afb.expr_sint(span, value)
            }
            ty::TyKind::Char => {
                let value = char::try_from(value.to_leaf())
                    .unwrap_or_else(|_| span_bug!(span, "invalid range character"));
                afb.expr_char(span, value)
            }
            ty::TyKind::Float => afb.expr_float(span, f64::from_bits(value.to_leaf().to_u64())),
            _ => not_implemented!(self.tcx(), span, "range pattern type", ty),
        }
    }

    fn translate_binding_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        name: Symbol,
        _mode: BindingMode,
        var: LocalVarId,
        subpattern: Option<&Pat<'tcx>>,
        is_primary: bool,
    ) -> Patterns<'arena> {
        let var_info = if is_primary {
            self.scopes.bind_var(var, name, true)
        } else {
            self.scopes.fetch_var(var)
        };

        let var_pat = self.translator.afb.pat_var(pat.span, var_info.erl_name);

        if let Some(sub) = subpattern {
            self.translate_pattern(sub)
                .into_iter()
                .map(|GuardedPattern(pattern, guards)| {
                    GuardedPattern(
                        self.translator.afb.pat_match(pat.span, var_pat, pattern),
                        guards,
                    )
                })
                .collect()
        } else {
            var_pat.into()
        }
    }

    fn translate_leaf_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        subpatterns: &[rustc_middle::thir::FieldPat<'tcx>],
    ) -> Patterns<'arena> {
        match pat.ty.kind() {
            ty::TyKind::Tuple(_) => {
                let field_count = pat.ty.tuple_fields().len();
                let afb = &self.translator.afb;

                let groups: SmallVec<[_; 4]> = (0..field_count)
                    .map(|idx| match subpatterns.iter().find(|field| field.field.index() == idx) {
                        Some(field) => self.translate_pattern(&field.pattern),
                        None => afb.pat_var(pat.span, kw::Underscore).into(),
                    })
                    .collect();

                self.pattern_cartesian_product(pat.span, &groups, |elements| {
                    afb.pat_tuple(pat.span, elements.iter().copied())
                })
            }

            ty::TyKind::Adt(adt_def, _) => {
                let variant = adt_def.non_enum_variant();
                self.translate_adt_pattern(pat.span, *adt_def, variant, subpatterns)
            }

            _ => not_implemented!(self.tcx(), pat.span, "leaf pattern type", pat.ty),
        }
    }

    fn resolve_expr(&self, expr: ExprId) -> &'a Expr<'tcx> {
        self.resolve_expr_with_scope(expr).0
    }

    fn resolve_expr_with_scope(
        &self,
        expr: ExprId,
    ) -> (&'a Expr<'tcx>, Option<(region::Scope, HirId)>) {
        let mut expr = &self.thir[expr];
        let mut region_scope = None;
        loop {
            match &expr.kind {
                ExprKind::Scope { region_scope: scope, hir_id, value, .. } => {
                    region_scope = Some((*scope, *hir_id));
                    expr = &self.thir[*value];
                }
                ExprKind::Use { source }
                | ExprKind::ByUse { expr: source, .. }
                | ExprKind::NeverToAny { source }
                | ExprKind::PointerCoercion { source, .. }
                | ExprKind::PlaceTypeAscription { source, .. }
                | ExprKind::ValueTypeAscription { source, .. } => expr = &self.thir[*source],

                _ => return (expr, region_scope),
            }
        }
    }

    fn is_always_diverging_expr(&self, expr_id: ExprId) -> bool {
        let (_expr, Some((_, hir_id))) = self.resolve_expr_with_scope(expr_id) else {
            span_bug!(
                self.thir[expr_id].span,
                "expected HIR-derived expression to have an outer Scope"
            );
        };

        self.tcx().typeck(self.owner).is_always_diverging_expr(hir_id)
    }

    pub(super) fn pattern_cartesian_product(
        &self,
        span: Span,
        groups: &[Patterns<'arena>],
        mut build: impl FnMut(&[eaf::Pattern<'arena>]) -> eaf::Pattern<'arena>,
    ) -> Patterns<'arena> {
        let combinations = groups.iter().fold(1usize, |acc, group| acc.saturating_mul(group.len()));
        if combinations > 1000 {
            span_fatal!(self.tcx(), span, "or-pattern expands to too many combinations");
        }

        // Build a cartesian product via DFS.
        // Somewhat prematurely optimized to avoid a Vec of Vecs
        fn visit<'arena>(
            groups: &[Patterns<'arena>],
            index: usize,
            combination: &mut SmallVec<[GuardedPattern<'arena>; 4]>,
            translated: &mut Patterns<'arena>,
            build: &mut impl FnMut(&[eaf::Pattern<'arena>]) -> eaf::Pattern<'arena>,
        ) {
            if index == groups.len() {
                let guards = combination
                    .iter()
                    .flat_map(|GuardedPattern(_, guards)| guards.into_iter())
                    .copied()
                    .collect();

                let patterns: SmallVec<[eaf::Pattern<'arena>; 4]> =
                    combination.iter().map(|GuardedPattern(pattern, _)| pattern).copied().collect();

                translated.push(GuardedPattern(build(&patterns), guards));
                return;
            }

            for alternative in &groups[index] {
                combination.push(alternative.clone());
                visit(groups, index + 1, combination, translated, build);
                combination.pop();
            }
        }

        let mut translated = SmallVec::new();
        visit(groups, 0, &mut SmallVec::new(), &mut translated, &mut build);
        translated
    }
}
