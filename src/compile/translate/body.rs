use rustc_abi::{FieldIdx, VariantIdx};
use rustc_ast::LitKind;
use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{BindingMode, HirId, Lit};
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution};
use rustc_middle::middle::region;
use rustc_middle::mir::{AssignOp, BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::{
    self, Arm, BlockId, Expr, ExprId, ExprKind, LocalVarId, Pat, PatKind, StmtId,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::translate::loops::Loops;
use crate::compile::translate::scopes::{PatinaVar, Scope, Scopes};
use crate::compile::translate::{ModuleParts, Translator, Witness};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

pub(crate) struct TranslatedBody<'arena> {
    pub params: SmallVec<[eaf::Pattern<'arena>; 8]>,
    pub value: eaf::Expr<'arena>,
}

struct TranslatedArm<'arena> {
    span: Span,
    pattern: eaf::Pattern<'arena>,
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

            ExprKind::Array { fields } => self.translate_list(expr.span, fields),

            ExprKind::Map { fields, base } => self.translate_map(expr.span, fields, *base),

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

    fn translate_list(&mut self, span: Span, fields: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let items = fields.iter().map(|&id| self.translate_expr(id));
        afb.expr_list(span, items)
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

        let var = self
            .scopes
            .get_var(id)
            .filter(|var| var.initialization_depth.is_some())
            .unwrap_or_else(|| {
                // TODO: this should be `report_use_of_uninitialized` from rustc
                span_fatal!(
                    self.tcx(),
                    span,
                    "variable {} is partially initialized",
                    ident.to_string()
                )
            });

        self.translator.afb.expr_var(span, var.erl_name)
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
        let afb = &self.translator.afb;
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
            exprs.push(afb.expr_unit(block.span));
        }

        afb.expr_block(block.span, exprs)
    }

    fn translate_stmt(&mut self, id: StmtId) -> Option<eaf::Expr<'arena>> {
        let stmt = &self.thir[id];

        match stmt.kind {
            thir::StmtKind::Expr { expr, .. } => Some(self.translate_expr(expr)),

            thir::StmtKind::Let { ref pattern, initializer, else_block, span, .. } => {
                if else_block.is_some() {
                    not_implemented!(self.tcx(), span, "let-else", else_block);
                }

                let Some(initializer) = initializer else {
                    self.declare_vars_from_pattern(pattern);
                    return None;
                };
                let rhs = self.translate_expr(initializer);
                let lhs = self.translate_pattern(pattern);
                Some(self.translator.afb.expr_match(span, lhs, rhs))
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

            let Some(module_resolution) = tcx.erl_module_for_target(self.owner.to_def_id(), target)
            else {
                let receiver = tcx.erl_call_target_self_ty(target).unwrap_or_else(|| {
                    span_bug!(expr.span, "dispatch through a type parameter must have a receiver")
                });
                not_implemented!(tcx, expr.span, "dispatch through a type parameter", receiver);
            };

            let ErlModuleResolution::Static(module_id) = module_resolution else {
                not_implemented!(tcx, expr.span, "dynamic function-item value", expr.ty);
            };

            let mod_name = self.translator.modules.module_name(module_id);
            let fn_name = tcx.item_name(*def_id);
            let arity = tcx.fn_arg_idents(*def_id).len();
            afb.expr_remote_fun(expr.span, mod_name, fn_name, arity as u32)
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
        let afb = &self.translator.afb;
        let scrutinee = self.translate_expr(scrutinee);

        let mut translated: SmallVec<[TranslatedArm<'arena>; 4]> =
            arms.iter().map(|&arm_id| self.translate_arm(arm_id)).collect();

        let changed_vars = self
            .scopes
            .modified_outer_vars(translated.iter().map(|arm| (&arm.scope, arm.always_diverges)));

        for arm in &mut translated {
            arm.body =
                self.bind_branch_outer_variables(arm.span, arm.body, &arm.scope, &changed_vars);
        }

        self.scopes.join_branches(&changed_vars);

        let clauses = translated
            .into_iter()
            .map(|arm| afb.clause(arm.span, [arm.pattern], arm.guard, arm.body));
        afb.expr_case(expr.span, scrutinee, clauses)
    }

    fn translate_arm(&mut self, arm_id: thir::ArmId) -> TranslatedArm<'arena> {
        let thir = self.thir;
        let Arm { span, pattern, guard, body, hir_id: _, scope: _ } = &thir[arm_id];
        let ((pattern, guard, body, always_diverges), scope) = self.with_scope(|cx| {
            let pattern = cx.translate_pattern(pattern);
            let guard = guard.map(|guard| cx.translate_expr(guard));
            let always_diverges = cx.is_always_diverging_expr(*body);
            let body = cx.translate_expr(*body);
            (pattern, guard, body, always_diverges)
        });
        TranslatedArm { span: *span, pattern, guard, body, scope, always_diverges }
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

    pub(super) fn translate_pattern(&mut self, pat: &Pat<'tcx>) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;

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

            PatKind::Constant { value } => self.translate_constant_pattern(pat.span, *value),

            PatKind::Map { fields } => {
                let assocs = fields.iter().map(|field| {
                    let key = self.translate_pattern(&field.key);
                    let value = self.translate_pattern(&field.pattern);
                    (key, value)
                });
                afb.pat_map(pat.span, assocs)
            }

            _ => not_implemented!(self.tcx(), pat.span, "THIR pattern kind", pat.kind),
        }
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
            _ => not_implemented!(self.tcx(), span, "constant pattern type", value.ty),
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
    ) -> eaf::Pattern<'arena> {
        let var_info = if is_primary {
            self.scopes.bind_var(var, name, true)
        } else {
            self.scopes.fetch_var(var)
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
        pat: &Pat<'tcx>,
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
                | ExprKind::Cast { source, .. }
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
}
