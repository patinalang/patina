use rustc_abi::{FIRST_VARIANT, FieldIdx, VariantIdx};
use rustc_ast::LitKind;
use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::{SmallVec, smallvec};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{HirId, Lit, TermClass};
use rustc_middle::erl::{
    ErlCallKind, ErlCallTarget, ErlModuleId, ErlModuleResolution, ErlReprKind,
};
use rustc_middle::middle::region;
use rustc_middle::mir::{BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::{
    self, Arm, BlockId, Expr, ExprId, ExprKind, LocalVarId, MapFieldPat, Pat, PatKind, StmtId,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::translate::loops::Loops;
use crate::compile::translate::scopes::{PatinaVar, Scope, Scopes};
use crate::compile::translate::witness::WitnessEnv;
use crate::compile::translate::{ModuleParts, Translator};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

/// Pattern with optional guards needed to match it.
#[derive(Debug, Clone)]
pub(super) struct GuardedPattern<'arena>(
    pub(super) eaf::Pattern<'arena>,
    pub(super) SmallVec<[eaf::Expr<'arena>; 1]>,
);
pub(super) type Patterns<'arena> = SmallVec<[GuardedPattern<'arena>; 1]>;

#[derive(Clone, Copy)]
pub(super) enum PatternInput {
    /// Static typing guarantees that the value has the pattern's type.
    Typed,
    /// The value came from `term` and each unconstrained leaf must be checked against its type.
    UntrustedRepresentation,
}

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

struct Branch<'arena> {
    span: Span,
    body: eaf::Expr<'arena>,
    scope: Scope,
}

struct TranslatedArm<'arena> {
    patterns: Patterns<'arena>,
    guard: Option<eaf::Expr<'arena>>,
    branch: Branch<'arena>,
}

struct CatchPattern<'arena> {
    class: eaf::ExceptionKind<'arena>,
    reason: eaf::Pattern<'arena>,
    stacktrace: Symbol,
    guards: SmallVec<[eaf::Expr<'arena>; 1]>,
}

struct GuardedExceptionKind<'arena> {
    kind: eaf::ExceptionKind<'arena>,
    guards: SmallVec<[eaf::Expr<'arena>; 1]>,
}

pub(super) struct BodyCx<'a, 'tcx, 'arena> {
    pub translator: &'a Translator<'tcx, 'arena>,
    pub owner: LocalDefId,
    pub thir: &'a thir::Thir<'tcx>,
    pub parts: &'a mut ModuleParts<'arena>,
    pub scopes: &'a mut Scopes,
    pub loops: Loops,
    pub witness_env: &'a WitnessEnv<'tcx, 'arena>,
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn new(
        translator: &'a Translator<'tcx, 'arena>,
        owner: LocalDefId,
        thir: &'a thir::Thir<'tcx>,
        parts: &'a mut ModuleParts<'arena>,
        scopes: &'a mut Scopes,
        witness_env: &'a WitnessEnv<'tcx, 'arena>,
    ) -> Self {
        BodyCx { translator, owner, thir, parts, scopes, witness_env, loops: Loops::new() }
    }

    pub(super) fn tcx(&self) -> TyCtxt<'tcx> {
        self.translator.tcx
    }

    pub(super) fn translate_expr(&mut self, expr_id: ExprId) -> eaf::Expr<'arena> {
        let (expr, region_scope) = self.resolve_expr_with_scope(expr_id);

        match &expr.kind {
            ExprKind::Block { block } => self.translate_block(*block),

            ExprKind::Literal { lit, neg } => self.translate_literal(expr.span, lit, *neg),

            ExprKind::VarRef { .. } | ExprKind::UpvarRef { .. } if expr.ty.is_phantom_data() => {
                self.translate_phantom_data(expr.span, expr.ty)
            }

            ExprKind::VarRef { id } => self.translate_var_ref(expr.span, *id),
            ExprKind::UpvarRef { var_hir_id, .. } => self.translate_var_ref(expr.span, *var_hir_id),

            ExprKind::Array { fields, tail } => self.translate_list(expr.span, fields, *tail),

            ExprKind::Map { fields, base } => self.translate_map(expr.span, fields, *base),

            ExprKind::BinaryPattern { segments } => {
                self.translate_expr_binary_pattern(expr.span, segments)
            }

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

            ExprKind::Assign { lhs, rhs } => self.translate_assign(expr, lhs, *rhs),

            ExprKind::If { if_then_scope: _, cond, then, else_opt } => {
                self.translate_if(expr, *cond, *then, *else_opt)
            }
            ExprKind::Receive { arms, timeout } => {
                self.translate_receive(expr, arms, timeout.as_ref())
            }
            ExprKind::TryCatch { body, arms, after } => {
                self.translate_try_catch(expr, *body, arms, after.as_ref())
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
        // An associated const of a trait is a function of the witnesses of its impl.
        if matches!(self.tcx().def_kind(def_id), DefKind::AssocConst { .. })
            && self.tcx().erl_entry_trait(def_id).is_some()
        {
            let target = self.tcx().erl_call_target_for_def(def_id, args);
            let arg_exprs = self.call_witness_exprs(span, target);
            let callee_expr = self.translator.afb.expr_atom(span, self.tcx().erl_fn_name(def_id));
            return self.emit_item_call(span, target, callee_expr, arg_exprs, false);
        }

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

    pub(super) fn translate_phantom_data(&self, span: Span, ty: Ty<'tcx>) -> eaf::Expr<'arena> {
        let Some(adt_def) = ty.ty_adt_def() else {
            span_bug!(span, "`PhantomData` value has non-ADT type {ty}")
        };
        let repr = self.tcx().erl_variant_repr(adt_def.non_enum_variant().def_id);
        let ErlReprKind::Atom = repr.kind else {
            span_bug!(span, "`PhantomData` is represented as {:?}, not as an atom", repr.kind)
        };
        self.translator.afb.expr_atom(span, repr.tag)
    }

    fn translate_pin_var_ref(&self, span: Span, id: LocalVarId) -> eaf::Pattern<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.pat_var(span, var.erl_name)
    }

    pub(super) fn translate_var_ref(&self, span: Span, id: LocalVarId) -> eaf::Expr<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.expr_var(span, var.erl_name)
    }

    fn var_for_read(&self, span: Span, id: LocalVarId) -> PatinaVar {
        *self.scopes.get_var(id).unwrap_or_else(|| span_bug!(span, "variable {id:?} not in scope"))
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
        let afb = &self.translator.afb;

        if let ty::Tuple(_) = ty.kind() {
            let subject = self.translate_expr(lhs);
            return afb.expr_remote_call(
                expr.span,
                afb.expr_atom(expr.span, sym::erlang),
                afb.expr_atom(expr.span, sym::element),
                [afb.expr_int(expr.span, name.index() as u128 + 1), subject],
            );
        }

        let adt_def = ty.ty_adt_def().unwrap_or_else(|| {
            not_implemented!(self.tcx(), expr.span, "field access on non-ADT type", ty);
        });

        let variant = adt_def.variant(variant_index);
        let repr = self.tcx().erl_variant_repr(variant.def_id);
        let subject = self.translate_expr(lhs);

        if self.tcx().is_erl_phantom_field(variant.fields[name].did) {
            let span = lhs_expr.span;
            let effect = afb.expr_match(span, afb.pat_var(span, kw::Underscore), subject);
            let value = self.translate_phantom_data(expr.span, expr.ty);
            return afb.expr_block(expr.span, [effect, value]);
        }

        match repr.kind {
            ErlReprKind::Elixir => {
                let field_name = afb.expr_atom(lhs_expr.span, variant.fields[name].name);
                afb.expr_remote_call(
                    expr.span,
                    afb.expr_atom(expr.span, sym::erlang),
                    afb.expr_atom(expr.span, sym::map_get),
                    [field_name, subject],
                )
            }
            ErlReprKind::Tagged => {
                let position =
                    self.tcx().erl_runtime_field_position(variant, name).unwrap_or_else(|| {
                        span_bug!(expr.span, "phantom field access in a tagged tuple")
                    });
                afb.expr_remote_call(
                    expr.span,
                    afb.expr_atom(expr.span, sym::erlang),
                    afb.expr_atom(expr.span, sym::element),
                    [afb.expr_int(expr.span, position as u128 + 2), subject],
                )
            }
            ErlReprKind::Int => not_implemented!(self.tcx(), expr.span, "int-represented ADT"),
            ErlReprKind::Transparent { .. } => subject,
            ErlReprKind::NativeRecord => afb.expr_record_field(
                expr.span,
                subject,
                self.record_module_for_access(repr.record_module),
                repr.tag,
                variant.fields[name].name,
            ),
            ErlReprKind::Atom => span_bug!(expr.span, "field access on atom-represented ADT"),
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
                    let declarations = self.declare_vars_from_pattern(pattern);
                    return (!declarations.is_empty())
                        .then(|| (afb.expr_block(span, declarations), true));
                };
                let rhs = self.translate_expr(initializer);

                let Some(else_block) = else_block else {
                    let lhs = self.translate_single_pattern(pattern);
                    return Some((afb.expr_match(span, lhs, rhs), false));
                };

                let declarations = self.declare_vars_from_pattern(pattern);

                let else_span = self.thir[else_block].span;
                let ((success_patterns, success_body), success_scope) = self.with_scope(|cx| {
                    let patterns = cx.translate_pattern(pattern, PatternInput::Typed);
                    let body = afb.expr_unit(span);
                    (patterns, body)
                });
                let (else_body, else_scope) = self.with_scope(|cx| cx.translate_block(else_block));

                let mut translated = SmallVec::new();
                translated.push(TranslatedArm {
                    patterns: success_patterns,
                    guard: None,
                    branch: Branch { span, body: success_body, scope: success_scope },
                });
                let else_pattern = afb.pat_var(else_span, kw::Underscore);
                translated.push(TranslatedArm {
                    patterns: else_pattern.into(),
                    guard: None,
                    branch: Branch { span: else_span, body: else_body, scope: else_scope },
                });

                let case = self.translate_to_case(span, rhs, translated);
                let expr = if declarations.is_empty() {
                    case
                } else {
                    afb.expr_block(span, declarations.into_iter().chain([case]))
                };
                Some((expr, true))
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

        // String, binary and bitstring concatenation lowers to binary construction.
        if op == BinOp::Add && lhs_ty.kind().bitstring_rank().is_some() {
            let tsl = if lhs_ty.is_bits() { eaf::Tsl::Bitstring } else { eaf::Tsl::Binary };
            let left_bin_elem =
                afb.expr_binary_element(self.thir[lhs].span, left, None, Some([tsl]));
            let right_bin_elem =
                afb.expr_binary_element(self.thir[rhs].span, right, None, Some([tsl]));

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
        let short_scope = Scope::new();

        let changed_vars = self.scopes.modified_outer_vars([&right_scope, &short_scope]);

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
            if matches!(self.tcx().def_kind(*callee_def_id), DefKind::Ctor(..)) {
                not_implemented!(self.tcx(), span, "calling a constructor through a variable");
            }
            if let Some(intrinsic) = self.tcx().intrinsic(*callee_def_id) {
                return self.translate_intrinsic_call(span, intrinsic.name, args);
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

        // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
        if let ErlCallTarget::Extern { module, function } = erl_call_target {
            let mod_name = self.tcx().erl_module_name(ErlModuleId::Extern(module));
            let fn_name = self.tcx().item_name(function);
            if let (sym::patina_fmt, sym::display | sym::debug) = (mod_name, fn_name) {
                return self.translate_format_render(span, fn_name, args[0]);
            }
        }

        let mut arg_exprs = self.call_witness_exprs(span, erl_call_target);
        arg_exprs.extend(args.iter().map(|&id| self.translate_expr(id)));

        self.emit_item_call(span, erl_call_target, callee_expr, arg_exprs, forced_extern)
    }

    fn translate_intrinsic_call(
        &mut self,
        span: Span,
        name: Symbol,
        args: &[ExprId],
    ) -> eaf::Expr<'arena> {
        match (name, args) {
            (sym::cast_unchecked, &[value]) => self.translate_expr(value),
            _ => not_implemented!(self.tcx(), span, "intrinsic", name),
        }
    }

    fn emit_item_call(
        &mut self,
        span: Span,
        target: ErlCallTarget<'tcx>,
        callee_expr: eaf::Expr<'arena>,
        arg_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
        forced_extern: bool,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let Some(call_kind) =
            self.tcx().erl_call_kind(self.owner.to_def_id(), target, Some(forced_extern))
        else {
            return self
                .translate_call_via_dispatch_through_type_parameter(span, target, arg_exprs);
        };

        if matches!(call_kind, ErlCallKind::Local | ErlCallKind::Dynamic) {
            return afb.expr_local_call(span, callee_expr, arg_exprs);
        }

        let module_resolution = self
            .tcx()
            .erl_module_for_target(self.owner.to_def_id(), target)
            .unwrap_or_else(|| span_bug!(span, "item call must have module resolution"));

        let ErlModuleResolution::Static(module_id) = module_resolution else {
            span_bug!(span, "dynamic target in static call path");
        };

        let mod_expr = afb.expr_atom(span, self.tcx().erl_module_name(module_id));

        afb.expr_remote_call(span, mod_expr, callee_expr, arg_exprs)
    }

    fn translate_call_via_dispatch_through_type_parameter(
        &mut self,
        span: Span,
        erl_call_target: ErlCallTarget<'tcx>,
        arg_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
    ) -> eaf::Expr<'arena> {
        let ErlCallTarget::Item { def_id, .. } = erl_call_target else {
            span_bug!(span, "dispatch through a type parameter must have an item target");
        };

        let mut arg_exprs = arg_exprs.into_iter();
        let self_witness = arg_exprs
            .next()
            .unwrap_or_else(|| span_bug!(span, "dispatch must have a receiver witness"));

        let fn_name = self.tcx().erl_fn_name(def_id);
        self.translator.dispatch_call(span, fn_name, self_witness, arg_exprs, self.scopes)
    }

    /// The witness arguments a call site must pass to `target`, in order.
    fn call_witness_exprs(
        &mut self,
        span: Span,
        target: ErlCallTarget<'tcx>,
    ) -> SmallVec<[eaf::Expr<'arena>; 8]> {
        let ErlCallTarget::Item { def_id, args } = target else {
            return SmallVec::new();
        };
        self.translator.call_witness_exprs(
            self.owner.to_def_id(),
            span,
            def_id,
            args,
            self.witness_env,
            self.scopes,
        )
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

    /// `fun(A1, ..., An) -> Name(Witnesses..., A1, ..., An) end`
    fn witnessed_fun(
        &mut self,
        span: Span,
        def_id: rustc_hir::def_id::DefId,
        target: ErlCallTarget<'tcx>,
        fn_name: Symbol,
        witness_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
    ) -> eaf::Expr<'arena> {
        let arity = self.tcx().fn_sig(def_id).skip_binder().inputs().skip_binder().len();
        let vars = (0..arity)
            .map(|_| self.scopes.fresh_var(sym::arg).erl_name)
            .collect::<SmallVec<[_; 4]>>();

        let afb = &self.translator.afb;
        let arg_exprs = witness_exprs
            .into_iter()
            .chain(vars.iter().map(|&var| afb.expr_var(span, var)))
            .collect();
        let callee_expr = afb.expr_atom(span, fn_name);
        let body = self.emit_item_call(span, target, callee_expr, arg_exprs, false);

        let afb = &self.translator.afb;
        let params = vars.iter().map(|&var| afb.pat_var(span, var));
        afb.expr_fun(span, [afb.clause(span, params, None, body)])
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
                    cx.witness_env,
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
            if let Some(intrinsic) = tcx.intrinsic(*def_id) {
                not_implemented!(tcx, expr.span, "intrinsic as a value", intrinsic.name);
            }

            let target = tcx.erl_call_target_for_def(*def_id, args);
            let fn_name = tcx.erl_fn_name(*def_id);

            // A function with witness parameters becomes a closure that passes this call site's
            // witnesses along.
            let witness_exprs = self.call_witness_exprs(expr.span, target);
            if !witness_exprs.is_empty() {
                return self.witnessed_fun(expr.span, *def_id, target, fn_name, witness_exprs);
            }

            let Some(call_kind) = tcx.erl_call_kind(self.owner.to_def_id(), target, None) else {
                span_bug!(expr.span, "`{fn_name}` dispatches through a witness it doesn't have")
            };

            let arity = tcx.erl_fn_arity(*def_id) as u32;

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
        lhs: &Pat<'tcx>,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let rhs = self.translate_expr(rhs);
        let lhs = self.translate_single_pattern(lhs);
        self.translator.afb.expr_match(expr.span, lhs, rhs)
    }

    fn translate_assignee(&mut self, span: Span, var: LocalVarId) -> eaf::Pattern<'arena> {
        let ident = self.tcx().hir_ident(var.0);
        let erl_name = self.scopes.bind_var(var, ident.name).erl_name;
        self.translator.afb.pat_var(span, erl_name)
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
            let ((then_patterns, then_body), then_scope) = self.with_scope(|cx| {
                let patterns = cx.translate_pattern(pat, PatternInput::Typed);
                let body = cx.translate_expr(then);
                (patterns, body)
            });

            let ((else_body, else_span), else_scope) = self.with_scope(|cx| match else_opt {
                Some(else_expr) => (cx.translate_expr(else_expr), cx.resolve_expr(else_expr).span),
                None => (cx.translator.afb.expr_unit(expr.span), expr.span),
            });

            let mut translated = SmallVec::new();
            translated.push(TranslatedArm {
                patterns: then_patterns,
                guard: None,
                branch: Branch { span: then_span, body: then_body, scope: then_scope },
            });
            let else_pattern = self.translator.afb.pat_var(else_span, kw::Underscore);
            translated.push(TranslatedArm {
                patterns: else_pattern.into(),
                guard: None,
                branch: Branch { span: else_span, body: else_body, scope: else_scope },
            });
            return self.translate_to_case(expr.span, scrutinee, translated);
        }

        let cond = self.translate_expr(cond);

        let (then_body, then_scope) = self.with_scope(|cx| cx.translate_expr(then));
        let then_span = self.resolve_expr(then).span;

        let ((else_body, else_span), else_scope) = self.with_scope(|cx| match else_opt {
            Some(else_expr) => (cx.translate_expr(else_expr), cx.resolve_expr(else_expr).span),
            None => (cx.translator.afb.expr_unit(expr.span), expr.span),
        });

        let changed_vars = self.scopes.modified_outer_vars([&then_scope, &else_scope]);
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

        let translated: SmallVec<[TranslatedArm<'arena>; 4]> =
            arms.iter().map(|&arm_id| self.translate_arm(arm_id)).collect();

        self.translate_to_case(expr.span, scrutinee, translated)
    }

    fn translate_to_case(
        &mut self,
        span: Span,
        scrutinee: eaf::Expr<'arena>,
        mut translated: SmallVec<[TranslatedArm<'arena>; 4]>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        self.merge_branches(translated.iter_mut().map(|arm| &mut arm.branch));

        let clauses = translated.into_iter().flat_map(|arm| {
            let TranslatedArm { patterns, guard, branch: Branch { span, body, .. } } = arm;
            patterns.into_iter().map(move |GuardedPattern(pattern, pattern_guards)| {
                afb.clause(span, [pattern], pattern_guards.into_iter().chain(guard), body)
            })
        });
        afb.expr_case(span, scrutinee, clauses)
    }

    fn merge_branches<'b>(&mut self, branches: impl IntoIterator<Item = &'b mut Branch<'arena>>)
    where
        'arena: 'b,
    {
        let mut branches: SmallVec<[&mut Branch<'arena>; 4]> = branches.into_iter().collect();
        let changed_vars =
            self.scopes.modified_outer_vars(branches.iter().map(|branch| &branch.scope));
        for branch in &mut branches {
            branch.body = self.bind_branch_outer_variables(
                branch.span,
                branch.body,
                &branch.scope,
                &changed_vars,
            );
        }
        self.scopes.join_branches(&changed_vars);
    }

    fn translate_receive(
        &mut self,
        expr: &Expr<'tcx>,
        arms: &[thir::ArmId],
        timeout: Option<&thir::ReceiveTimeout>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let mut after = timeout.map(|timeout| {
            let span = self.resolve_expr(timeout.body).span;
            let (body, scope) = self.with_scope(|cx| cx.translate_expr(timeout.body));
            (self.translate_expr(timeout.expr), Branch { span, body, scope })
        });

        let mut translated: SmallVec<[TranslatedArm<'arena>; 4]> =
            arms.iter().map(|&arm| self.translate_arm(arm)).collect();
        self.merge_branches(
            translated
                .iter_mut()
                .map(|arm| &mut arm.branch)
                .chain(after.iter_mut().map(|(_, branch)| branch)),
        );

        let clauses = translated.into_iter().flat_map(|arm| {
            let TranslatedArm { patterns, guard, branch: Branch { span, body, .. } } = arm;
            patterns.into_iter().map(move |GuardedPattern(pattern, pattern_guards)| {
                afb.clause(span, [pattern], pattern_guards.into_iter().chain(guard), body)
            })
        });
        afb.expr_receive(
            expr.span,
            clauses,
            after.map(|(timeout_expr, branch)| (timeout_expr, branch.body)),
        )
    }

    fn translate_try_catch(
        &mut self,
        expr: &Expr<'tcx>,
        body: ExprId,
        arms: &[thir::ArmId],
        after: Option<&ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let (body, _) = self.with_scope(|cx| cx.translate_expr(body));
        let after = after.map(|after| self.with_scope(|cx| cx.translate_expr(*after)).0);

        let catches = arms.iter().flat_map(|&arm| {
            let Arm { span, pattern, guard, stacktrace, body, hir_id: _, scope: _ } =
                &self.thir[arm];

            let ((patterns, guard, body), _scope) = self.with_scope(|cx| {
                let patterns = cx.translate_exception_pattern(pattern, stacktrace.as_deref());
                let guard = guard.map(|guard| cx.translate_expr(guard));
                let body = cx.translate_expr(*body);
                (patterns, guard, body)
            });

            patterns.into_iter().map(move |pattern| {
                afb.catch_clause(
                    *span,
                    pattern.class,
                    pattern.reason,
                    pattern.stacktrace,
                    pattern.guards.into_iter().chain(guard),
                    body,
                )
            })
        });
        afb.expr_try(expr.span, body, catches, after)
    }

    fn translate_exception_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        stacktrace: Option<&Pat<'tcx>>,
    ) -> SmallVec<[CatchPattern<'arena>; 1]> {
        let PatKind::Exception { class_pat, reason_pat, .. } = &pat.kind else {
            span_bug!(pat.span, "catch arm without exception pattern")
        };
        let stacktrace = match stacktrace {
            Some(stacktrace @ Pat { kind: PatKind::Binding { name, var, is_primary, .. }, .. }) => {
                if !is_primary {
                    span_bug!(stacktrace.span, "non-primary stacktrace binding")
                }
                self.scopes.bind_var(*var, *name).erl_name
            }
            Some(Pat { kind: PatKind::Wild, .. }) | None => kw::Underscore,
            Some(stacktrace) => {
                span_bug!(stacktrace.span, "non-variable stacktrace pattern")
            }
        };

        let classes = self.translate_exception_kind_pattern(class_pat);
        let reasons = self.translate_pattern(reason_pat, PatternInput::Typed);
        let mut patterns = SmallVec::new();

        for class in classes {
            for GuardedPattern(reason, reason_guards) in &reasons {
                patterns.push(CatchPattern {
                    class: class.kind,
                    reason: *reason,
                    stacktrace,
                    guards: class
                        .guards
                        .iter()
                        .copied()
                        .chain(reason_guards.iter().copied())
                        .collect(),
                });
            }
        }
        patterns
    }

    /// Translate a `catch` arm's class pattern into Erlang's class slot.
    fn translate_exception_kind_pattern(
        &mut self,
        pat: &Pat<'tcx>,
    ) -> SmallVec<[GuardedExceptionKind<'arena>; 1]> {
        let afb = &self.translator.afb;

        // The slot has no room for an alias, so keep the variable and demote the pattern it is
        // aliasing to a guard on it.
        if let PatKind::Binding { name, var, subpattern: Some(subpattern), is_primary, .. } =
            &pat.kind
        {
            let class = self.bind_pattern_var(*name, *var, *is_primary).erl_name;
            return self
                .translate_exception_kind_pattern(subpattern)
                .into_iter()
                .map(|alternative| {
                    let constraint = afb.expr_binary_op(
                        pat.span,
                        eaf::BinaryOp::EqExact,
                        afb.expr_var(pat.span, class),
                        afb.exception_kind_as_expr(alternative.kind),
                    );
                    GuardedExceptionKind {
                        kind: afb.exception_kind_var(pat.span, class),
                        guards: std::iter::once(constraint).chain(alternative.guards).collect(),
                    }
                })
                .collect();
        }

        self.translate_pattern(pat, PatternInput::Typed)
            .into_iter()
            .map(|GuardedPattern(pattern, guards)| GuardedExceptionKind {
                // The class pattern is typed `atom`, so nothing else can reach here.
                kind: afb.exception_kind_from_pattern(pattern).unwrap_or_else(|| {
                    span_bug!(pat.span, "exception class pattern is not an atom or variable")
                }),
                guards,
            })
            .collect()
    }

    fn translate_arm(&mut self, arm_id: thir::ArmId) -> TranslatedArm<'arena> {
        let thir = self.thir;
        let Arm { span, pattern, guard, stacktrace: _, body, hir_id: _, scope: _ } = &thir[arm_id];
        let ((patterns, guard, body), scope) = self.with_scope(|cx| {
            let patterns = cx.translate_pattern(pattern, PatternInput::Typed);
            let guard = guard.map(|guard| cx.translate_expr(guard));
            let body = cx.translate_expr(*body);
            (patterns, guard, body)
        });
        TranslatedArm { patterns, guard, branch: Branch { span: *span, body, scope } }
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

        let result_var = self.scopes.fresh_var(sym::branch_value);

        let result_var_pat = afb.pat_var(span, result_var.erl_name);
        let result_var_expr = afb.expr_var(span, result_var.erl_name);
        let result_var_match = afb.expr_match(span, result_var_pat, body);

        let exports = changed_vars.iter().flat_map(|(id, export_var)| {
            let branch_var = branch_scope
                .get_var(*id)
                .or_else(|| self.scopes.get_var(*id))
                .unwrap_or_else(|| span_bug!(span, "variable {id:?} not in scope"));

            if branch_var.vsn < export_var.vsn {
                let branch_var_expr = afb.expr_var(span, branch_var.erl_name);
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
        let patterns = self.translate_pattern(pat, PatternInput::Typed);
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

    pub(super) fn translate_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        input: PatternInput,
    ) -> Patterns<'arena> {
        match &pat.kind {
            PatKind::Or { pats } => {
                pats.iter().flat_map(|pattern| self.translate_pattern(pattern, input)).collect()
            }

            PatKind::Range(range) => self.translate_range_pattern(pat.span, range, input),

            PatKind::Wild => self.translate_wild_pattern(pat.span, pat.ty, input),

            PatKind::Binding {
                name,
                mode: _,
                var,
                subpattern,
                is_primary,
                ty: _,
                is_shorthand: _,
            } => self.translate_binding_pattern(
                pat,
                *name,
                *var,
                subpattern.as_deref(),
                *is_primary,
                input,
            ),

            PatKind::Pin { .. } | PatKind::Assignee { .. } if pat.ty.is_phantom_data() => {
                self.translate_wild_pattern(pat.span, pat.ty, input)
            }

            PatKind::Pin { var } => self.translate_pin_var_ref(pat.span, *var).into(),

            PatKind::Assignee { var } => self.translate_assignee(pat.span, *var).into(),

            PatKind::TermClass { class, subpattern } => {
                self.translate_term_class_pattern(pat.span, *class, subpattern)
            }

            PatKind::Exception { .. } => {
                span_bug!(pat.span, "exception pattern outside catch arm")
            }

            PatKind::TermPat { subpattern, adt_repr_match } => {
                let input = if *adt_repr_match {
                    PatternInput::UntrustedRepresentation
                } else {
                    PatternInput::Typed
                };
                self.translate_pattern(subpattern, input)
            }

            PatKind::Deref { subpattern, .. } => self.translate_pattern(subpattern, input),

            PatKind::Leaf { subpatterns } => self.translate_leaf_pattern(pat, subpatterns, input),

            PatKind::Variant { adt_def, variant_index, subpatterns, .. } => {
                self.translate_adt_pattern(pat, *adt_def, *variant_index, subpatterns, input)
            }

            PatKind::Constant { value } => self.translate_constant_pattern(pat.span, *value).into(),

            PatKind::Map { fields } => self.translate_map_pattern(pat.span, fields, input),

            PatKind::BinaryPattern { segments } => {
                self.translate_pat_binary_pattern(pat.span, segments)
            }

            PatKind::Slice { prefix, slice, suffix } => {
                self.translate_slice_pattern(pat.span, prefix, slice.as_deref(), suffix, input)
            }

            _ => not_implemented!(self.tcx(), pat.span, "THIR pattern kind", pat.kind),
        }
    }

    fn translate_map_pattern(
        &mut self,
        span: Span,
        fields: &[MapFieldPat<'tcx>],
        input: PatternInput,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let keys: SmallVec<[_; 4]> =
            fields.iter().map(|field| self.translate_expr(field.key)).collect();
        let groups: SmallVec<[_; 4]> =
            fields.iter().map(|field| self.translate_pattern(&field.pattern, input)).collect();

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
        input: PatternInput,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        if !suffix.is_empty() {
            span_bug!(span, "list rest pattern must be trailing")
        }

        let tail = slice.map(|pattern| self.translate_pattern(pattern, input));
        let groups: SmallVec<[_; 4]> = prefix
            .iter()
            .map(|pattern| self.translate_pattern(pattern, input))
            .chain(tail)
            .collect();

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

    fn declare_vars_from_pattern(&mut self, pat: &Pat<'tcx>) -> SmallVec<[eaf::Expr<'arena>; 4]> {
        let mut declarations = SmallVec::new();
        self.declare_vars_from_pattern_into(pat, &mut declarations);
        declarations
    }

    fn declare_vars_from_pattern_into(
        &mut self,
        pat: &Pat<'tcx>,
        declarations: &mut SmallVec<[eaf::Expr<'arena>; 4]>,
    ) {
        match &pat.kind {
            PatKind::Binding { var, subpattern, is_primary, .. } => {
                if *is_primary && !pat.ty.is_phantom_data() {
                    let ident = self.tcx().hir_ident(var.0);
                    let erl_name = self.scopes.bind_var(*var, ident.name).erl_name;
                    let afb = &self.translator.afb;
                    declarations.push(afb.expr_match(
                        pat.span,
                        afb.pat_var(pat.span, erl_name),
                        afb.expr_unit(pat.span),
                    ));
                }
                if let Some(subpattern) = subpattern {
                    self.declare_vars_from_pattern_into(subpattern, declarations);
                }
            }
            PatKind::Deref { subpattern, .. }
            | PatKind::DerefPattern { subpattern, .. }
            | PatKind::Guard { subpattern, .. }
            | PatKind::TermClass { subpattern, .. }
            | PatKind::TermPat { subpattern, .. } => {
                self.declare_vars_from_pattern_into(subpattern, declarations)
            }
            PatKind::Exception { class_pat, reason_pat, .. } => {
                self.declare_vars_from_pattern_into(class_pat, declarations);
                self.declare_vars_from_pattern_into(reason_pat, declarations);
            }
            PatKind::Leaf { subpatterns } | PatKind::Variant { subpatterns, .. } => {
                for field in subpatterns {
                    self.declare_vars_from_pattern_into(&field.pattern, declarations);
                }
            }
            PatKind::Slice { prefix, slice, suffix } | PatKind::Array { prefix, slice, suffix } => {
                for pattern in prefix.iter().chain(slice.as_deref()).chain(suffix.iter()) {
                    self.declare_vars_from_pattern_into(pattern, declarations);
                }
            }
            PatKind::Or { pats } => {
                for pattern in pats {
                    self.declare_vars_from_pattern_into(pattern, declarations);
                }
            }
            PatKind::Map { fields, .. } => {
                for field in fields {
                    self.declare_vars_from_pattern_into(&field.pattern, declarations);
                }
            }
            PatKind::BinaryPattern { segments } => {
                for segment in segments {
                    self.declare_vars_from_pattern_into(&segment.pattern, declarations);
                }
            }
            PatKind::Missing
            | PatKind::Wild
            | PatKind::Pin { .. }
            | PatKind::Assignee { .. }
            | PatKind::Constant { .. }
            | PatKind::Range(_)
            | PatKind::Never
            | PatKind::Error(_) => {}
        }
    }

    fn bind_pattern_var(&mut self, name: Symbol, var: LocalVarId, is_primary: bool) -> &PatinaVar {
        if is_primary { self.scopes.bind_var(var, name) } else { self.scopes.fetch_var(var) }
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
            ty::TyKind::Str | ty::TyKind::Bin | ty::TyKind::Bits => {
                let bytes = value.try_to_raw_bytes(self.tcx()).unwrap_or_else(|| {
                    span_bug!(span, "binary pattern did not lower to raw bytes")
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
        input: PatternInput,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let range_var = self.scopes.fresh_var(sym::range);

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
        let shape_guard = matches!(input, PatternInput::UntrustedRepresentation)
            .then(|| {
                let shape = self.tcx().erl_shape_of_ty(range.ty);
                self.guards_for_shape(span, afb.expr_var(span, range_var.erl_name), shape)
            })
            .flatten();

        GuardedPattern(
            afb.pat_var(span, range_var.erl_name),
            shape_guard.into_iter().chain(lower).chain(upper).collect(),
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
        var: LocalVarId,
        subpattern: Option<&Pat<'tcx>>,
        is_primary: bool,
        input: PatternInput,
    ) -> Patterns<'arena> {
        if pat.ty.is_phantom_data() {
            return match subpattern {
                Some(sub) => self.translate_pattern(sub, input),
                None => self.translate_wild_pattern(pat.span, pat.ty, input),
            };
        }

        let erl_name = self.bind_pattern_var(name, var, is_primary).erl_name;
        let var_pat = self.translator.afb.pat_var(pat.span, erl_name);

        if let Some(sub) = subpattern {
            self.translate_pattern(sub, input)
                .into_iter()
                .map(|GuardedPattern(pattern, guards)| {
                    GuardedPattern(
                        self.translator.afb.pat_match(pat.span, var_pat, pattern),
                        guards,
                    )
                })
                .collect()
        } else if matches!(input, PatternInput::UntrustedRepresentation) {
            self.guard_pattern_for_type(pat.span, pat.ty, erl_name)
        } else {
            var_pat.into()
        }
    }

    fn translate_term_class_pattern(
        &mut self,
        span: Span,
        class: TermClass,
        subpattern: &Pat<'tcx>,
    ) -> Patterns<'arena> {
        self.translate_pattern(subpattern, PatternInput::Typed)
            .into_iter()
            .map(|GuardedPattern(pattern, mut guards)| {
                if let eaf::Pattern::Var(_, name) = pattern
                    && name.0 != kw::Underscore
                    && guards.is_empty()
                {
                    let value = self.translator.afb.expr_var(span, name.0);
                    guards.insert(0, self.term_class_guard(span, value, class));
                    return GuardedPattern(pattern, guards);
                }

                let value = self.scopes.fresh_var(sym::value);
                let value_pat = self.translator.afb.pat_var(span, value.erl_name);
                let value_expr = self.translator.afb.expr_var(span, value.erl_name);
                let class_guard = self.term_class_guard(span, value_expr, class);
                guards.insert(0, class_guard);

                // Small codegen optimization - skip matching wildcard `_`.
                let pattern = match subpattern.kind {
                    PatKind::Wild => value_pat,
                    _ => self.translator.afb.pat_match(span, value_pat, pattern),
                };
                GuardedPattern(pattern, guards)
            })
            .collect()
    }

    fn term_class_guard(
        &self,
        span: Span,
        value: eaf::Expr<'arena>,
        class: TermClass,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let name = match class {
            TermClass::Int => sym::is_integer,
            TermClass::Float => sym::is_float,
            TermClass::Atom => sym::is_atom,
            TermClass::Bool => sym::is_boolean,
            TermClass::Bin => sym::is_binary,
            TermClass::List => sym::is_list,
            TermClass::Map => sym::is_map,
            TermClass::Tuple => sym::is_tuple,
            TermClass::Fn => sym::is_function,
            TermClass::Bits => sym::is_bitstring,
            TermClass::Pid => sym::is_pid,
            TermClass::Port => sym::is_port,
            TermClass::Ref => sym::is_reference,
        };
        afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::erlang),
            afb.expr_atom(span, name),
            [value],
        )
    }

    pub(super) fn translate_wild_pattern(
        &mut self,
        span: Span,
        ty: Ty<'tcx>,
        input: PatternInput,
    ) -> Patterns<'arena> {
        if !matches!(input, PatternInput::UntrustedRepresentation) {
            return self.translator.afb.pat_var(span, kw::Underscore).into();
        }
        let var = self.scopes.fresh_var(sym::value);
        self.guard_pattern_for_type(span, ty, var.erl_name)
    }

    fn guard_pattern_for_type(&self, span: Span, ty: Ty<'tcx>, var: Symbol) -> Patterns<'arena> {
        let pattern = self.translator.afb.pat_var(span, var);
        let shape = self.tcx().erl_shape_of_ty(ty);
        match self.guards_for_shape(span, self.translator.afb.expr_var(span, var), shape) {
            Some(guard) => GuardedPattern(pattern, smallvec![guard]).into(),
            None => pattern.into(),
        }
    }

    fn translate_leaf_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        subpatterns: &[rustc_middle::thir::FieldPat<'tcx>],
        input: PatternInput,
    ) -> Patterns<'arena> {
        match pat.ty.kind() {
            ty::TyKind::Tuple(_) => {
                let field_count = pat.ty.tuple_fields().len();
                let afb = &self.translator.afb;

                let groups: SmallVec<[_; 4]> = (0..field_count)
                    .map(|idx| match subpatterns.iter().find(|field| field.field.index() == idx) {
                        Some(field) => self.translate_pattern(&field.pattern, input),
                        None => {
                            self.translate_wild_pattern(pat.span, pat.ty.tuple_fields()[idx], input)
                        }
                    })
                    .collect();

                self.pattern_cartesian_product(pat.span, &groups, |elements| {
                    afb.pat_tuple(pat.span, elements.iter().copied())
                })
            }

            ty::TyKind::Adt(adt_def, _) => {
                self.translate_adt_pattern(pat, *adt_def, FIRST_VARIANT, subpatterns, input)
            }

            _ => not_implemented!(self.tcx(), pat.span, "leaf pattern type", pat.ty),
        }
    }

    pub(super) fn resolve_expr(&self, expr: ExprId) -> &'a Expr<'tcx> {
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
