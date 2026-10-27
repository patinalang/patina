use rustc_hir::def::DefKind;
use rustc_hir::{BinarySegmentTy, HirId};
use rustc_middle::erl::binary_pattern::{BinaryEndianness, BinarySegmentLayout, BinarySegmentSize};
use rustc_middle::erl::etir::visit::{self, Visitor, walk_pat};
use rustc_middle::erl::etir::{self, ExprId, JoinId, PatId, PatternInput};
use rustc_middle::erl::{ErlCallTarget, ErlModuleId, ErlModuleResolution};
use rustc_middle::middle::region;
use rustc_middle::mir::{BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::{self, AdtExprBase, LocalVarId};
use rustc_middle::ty::{self, GenericArgsRef, Ty};
use rustc_span::{Ident, Span, Symbol, sym};

use super::{ExprBlock, ThirBodyCx, not_implemented, not_implemented_kind};

macro_rules! with_expr_block {
    ($cx:ident, |$block:ident| $body:block) => {{
        let mut $block = ExprBlock::default();
        let result = (|| $body)();
        $block.emit(&mut $cx.cx.body, result)
    }};
}

impl<'a, 'tcx> ExprBlock {
    /// `lower_expr` then run it through `maybe_bind` to ensure we got a variable or literal.
    fn lower_and_maybe_bind(
        &mut self,
        cx: &mut ThirBodyCx<'a, 'tcx>,
        name: Symbol,
        expr_id: thir::ExprId,
    ) -> ExprId {
        let expr = cx.lower_expr(expr_id);
        self.maybe_bind(cx, name, expr)
    }

    /// Emits an assignment of `value` to a fresh var named `name`, and returns the var expr.
    /// If `value` is already a var or a literal, does nothing.
    fn maybe_bind(&mut self, cx: &mut ThirBodyCx<'a, 'tcx>, name: Symbol, value: ExprId) -> ExprId {
        let etir::Expr { ty, span, ref kind } = cx.cx.body[value];
        if matches!(kind, etir::ExprKind::Var(_) | etir::ExprKind::Literal(_, _)) {
            return value;
        }

        let var = cx.push_var(Ident { name, span });

        let binding = cx.push_pat(
            etir::PatKind::Binding {
                var,
                is_primary: true,
                subpattern: None,
                input: PatternInput::Typed,
            },
            ty,
            span,
        );

        let assign_expr =
            cx.push_expr(etir::ExprKind::Assign { lhs: binding, rhs: value }, ty, span);

        self.0.push(assign_expr);

        cx.push_expr(etir::ExprKind::Var(var), ty, span)
    }
}

impl<'a, 'tcx> ThirBodyCx<'a, 'tcx> {
    pub(super) fn lower_expr(&mut self, expr_id: thir::ExprId) -> ExprId {
        let tcx = self.cx.tcx;
        use thir::ExprKind as Thir;

        let (expr, region_scope) = self.resolve_expr_with_scope(expr_id);

        match &expr.kind {
            Thir::Block { block } => self.lower_block(*block, expr.ty, expr.span),

            Thir::Literal { lit, neg } => self.lower_expr_literal(expr, lit, *neg),

            Thir::VarRef { .. } | Thir::UpvarRef { .. } if expr.ty.is_phantom_data() => {
                self.lower_expr_phantom_data(expr)
            }

            Thir::VarRef { id } => self.lower_expr_var_ref(expr, *id),
            Thir::UpvarRef { var_hir_id, .. } => self.lower_expr_var_ref(expr, *var_hir_id),

            Thir::Array { fields, tail } => self.lower_expr_list(expr, fields, *tail),

            Thir::Map { fields, base } => self.lower_expr_map(expr, fields, *base),

            Thir::BinaryPattern { segments } => self.lower_expr_binary_pattern(expr, segments),

            Thir::Tuple { fields } => self.lower_expr_tuple(expr, fields),

            Thir::Field { lhs, variant_index, name } => {
                self.lower_expr_field(expr, *lhs, *variant_index, *name)
            }

            Thir::Binary { op, lhs, rhs } => self.lower_expr_binary_op(expr, *op, *lhs, *rhs),

            Thir::LogicalOp { op, lhs, rhs } => self.lower_expr_logical_op(expr, *op, *lhs, *rhs),

            Thir::Unary { op, arg } => self.lower_expr_unary_op(expr, *op, *arg),

            Thir::Cast { source } => self.lower_expr_cast(expr, *source),

            Thir::Call { ty, fun, args, forced_extern, .. } => {
                self.lower_expr_call(expr, *ty, *fun, args, *forced_extern)
            }

            Thir::Adt(adt_expr) => self.lower_expr_adt(expr, adt_expr),

            Thir::Closure(closure) => self.lower_expr_closure(expr, closure),

            Thir::ZstLiteral { .. } => self.lower_expr_zst_literal(expr),

            Thir::NamedConst { def_id, args, .. } => {
                self.lower_expr_named_const(expr, *def_id, args)
            }

            Thir::Assign { lhs, rhs } => self.lower_expr_assign(expr, lhs, *rhs),

            Thir::If { cond, then, else_opt, .. } => {
                self.lower_expr_if(expr, *cond, *then, *else_opt)
            }
            Thir::Receive { arms, timeout } => {
                self.lower_expr_receive(expr, arms, timeout.as_ref())
            }
            Thir::TryCatch { body, arms, after } => {
                self.lower_expr_try_catch(expr, *body, arms, *after)
            }
            Thir::Match { scrutinee, arms, .. } => self.lower_expr_match(expr, *scrutinee, arms),

            Thir::Return { value } => self.lower_expr_return(expr, *value),
            Thir::Loop { body } => self.lower_expr_loop(expr, region_scope, *body),
            Thir::Break { label, value } => self.lower_expr_break(expr, *label, *value),
            Thir::Continue { label } => self.lower_expr_continue(expr, *label),

            _ => not_implemented_kind(tcx, expr.span, "THIR expression kind", &expr.kind),
        }
    }

    fn lower_exprs(&mut self, exprs: &[thir::ExprId]) -> Box<[ExprId]> {
        exprs.iter().map(|&expr| self.lower_expr(expr)).collect()
    }

    fn lower_block(&mut self, id: thir::BlockId, ty: Ty<'tcx>, span: Span) -> ExprId {
        let block = &self.thir[id];
        let mut exprs: Vec<_> =
            block.stmts.iter().filter_map(|&stmt| self.lower_stmt(stmt)).collect();
        match block.expr {
            Some(expr) => exprs.push(self.lower_expr(expr)),
            // The last statement either provides the block's `()`, or never completes.
            None if exprs.last().is_some_and(|&expr| {
                let ty = self.cx.body[expr].ty;
                ty.is_unit() || ty.is_never()
            }) => {}
            None => exprs.push(self.unit_expr(block.span)),
        }

        let value = self.push_expr(etir::ExprKind::Block(exprs.into_boxed_slice()), ty, span);

        let join = self.cx.break_joins.remove(&block.region_scope);
        self.wrap_in_joins(join, value)
    }

    fn lower_stmt(&mut self, id: thir::StmtId) -> Option<ExprId> {
        match self.thir[id].kind {
            thir::StmtKind::Expr { expr, .. } => Some(self.lower_expr(expr)),

            thir::StmtKind::Let { ref pattern, initializer, else_block, span, .. } => {
                self.lower_stmt_let(span, pattern, initializer, else_block)
            }
        }
    }

    fn lower_stmt_let(
        &mut self,
        span: Span,
        pattern: &thir::Pat<'tcx>,
        initializer: Option<thir::ExprId>,
        else_block: Option<thir::BlockId>,
    ) -> Option<ExprId> {
        let tcx = self.cx.tcx;
        let unit = tcx.types.unit;
        let Some(initializer) = initializer else {
            let pattern = self.lower_pat(pattern, PatternInput::Typed);
            return self.declare_from_pattern(span, pattern);
        };

        let ty = self.thir[initializer].ty;
        let initializer = self.lower_expr(initializer);
        let pattern = self.lower_pat(pattern, PatternInput::Typed);

        let Some(else_block) = else_block else {
            let kind = etir::ExprKind::Assign { lhs: pattern, rhs: initializer };
            return Some(self.push_expr(kind, ty, span));
        };

        let else_span = self.thir[else_block].span;

        let success_arm = etir::Arm { pattern, guard: None, body: self.unit_expr(span), span };

        let else_arm = etir::Arm {
            pattern: self.push_pat(
                etir::PatKind::Wild { input: PatternInput::Typed },
                ty,
                else_span,
            ),
            guard: None,
            body: self.lower_block(else_block, tcx.types.never, else_span),
            span: else_span,
        };

        let declare = self.declare_from_pattern(span, pattern);
        let matched = self.push_expr(
            etir::ExprKind::Match {
                scrutinee: initializer,
                arms: Box::new([success_arm, else_arm]),
            },
            unit,
            span,
        );

        let Some(declare) = declare else { return Some(matched) };
        Some(self.push_expr(etir::ExprKind::Block(Box::new([declare, matched])), unit, span))
    }

    fn declare_from_pattern(&mut self, span: Span, pattern: PatId) -> Option<ExprId> {
        let vars = self.bound_vars(pattern);
        if vars.is_empty() {
            return None;
        }
        let kind = etir::ExprKind::Declare { vars: vars.into_iter().collect() };
        Some(self.push_expr(kind, self.cx.tcx.types.unit, span))
    }

    fn lower_expr_literal(
        &mut self,
        expr: &thir::Expr<'tcx>,
        lit: &rustc_hir::Lit,
        neg: bool,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        use rustc_ast::LitKind;
        let value = match lit.node {
            LitKind::Int(value) => etir::Lit::Int { value, neg },
            LitKind::Float(symbol, _style) => {
                let symbol_str = symbol.as_str();
                let float = symbol_str.parse::<f64>().unwrap_or_else(|e| {
                    tcx.dcx()
                        .span_fatal(lit.span, format!("failed to parse float `{symbol_str}`: {e}"))
                });
                etir::Lit::Float(if neg { -float } else { float }.to_bits())
            }
            LitKind::Err(guar) => guar.raise_fatal(),
            _ if neg => span_bug!(lit.span, "negated non-numeric literal {lit:?}"),
            LitKind::Str(symbol, _style) => etir::Lit::Str(symbol),
            LitKind::Atom(symbol, _style) => etir::Lit::Atom(symbol),
            LitKind::ByteStr(symbol, _style) => etir::Lit::ByteStr(symbol),
            LitKind::Char(char) => etir::Lit::Char(char),
            LitKind::Bool(bool) => etir::Lit::Bool(bool),
            // rustc leftovers
            LitKind::CStr(..) | LitKind::Byte(..) => tcx.dcx().span_fatal(
                lit.span,
                format!("literal kind not supported: {}", lit.node.to_string()),
            ),
        };
        let kind = etir::ExprKind::Literal(value, lit.span);
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_phantom_data(&mut self, expr: &thir::Expr<'tcx>) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;
        let Some(adt_def) = expr.ty.ty_adt_def() else {
            span_bug!(span, "`PhantomData` value has non-ADT type {}", expr.ty)
        };
        let tag = tcx.erl_variant_repr(adt_def.non_enum_variant().def_id).tag;
        let kind = etir::ExprKind::Literal(etir::Lit::Atom(tag), span);
        self.push_expr(kind, expr.ty, span)
    }

    fn lower_expr_var_ref(&mut self, expr: &thir::Expr<'tcx>, id: LocalVarId) -> ExprId {
        let var = self.var(id);
        self.push_expr(etir::ExprKind::Var(var), expr.ty, expr.span)
    }

    fn lower_expr_list(
        &mut self,
        expr: &thir::Expr<'tcx>,
        items: &[thir::ExprId],
        tail: Option<thir::ExprId>,
    ) -> ExprId {
        with_expr_block!(self, |expr_body| {
            let tail = tail.map(|tail| expr_body.lower_and_maybe_bind(self, sym::tail, tail));
            let items = items
                .iter()
                .map(|&item| expr_body.lower_and_maybe_bind(self, sym::element, item))
                .collect();

            self.push_expr(etir::ExprKind::List { items, tail }, expr.ty, expr.span)
        })
    }

    fn lower_expr_map(
        &mut self,
        expr: &thir::Expr<'tcx>,
        fields: &[thir::MapFieldExpr],
        base: Option<thir::ExprId>,
    ) -> ExprId {
        with_expr_block!(self, |expr_body| {
            let base = base.map(|base| expr_body.lower_and_maybe_bind(self, sym::field_base, base));
            let fields = fields
                .iter()
                .map(|field| {
                    let key = expr_body.lower_and_maybe_bind(self, sym::key, field.key);
                    let value = expr_body.lower_and_maybe_bind(self, sym::value, field.value);
                    (key, value)
                })
                .collect();

            self.push_expr(etir::ExprKind::Map { fields, base }, expr.ty, expr.span)
        })
    }

    fn lower_expr_binary_pattern(
        &mut self,
        expr: &thir::Expr<'tcx>,
        segments: &[thir::BinarySegmentExpr],
    ) -> ExprId {
        let segments =
            segments.iter().map(|segment| self.lower_expr_binary_segment(segment)).collect();
        self.push_expr(etir::ExprKind::BinaryPattern { segments }, expr.ty, expr.span)
    }

    fn lower_expr_binary_segment(
        &mut self,
        segment: &thir::BinarySegmentExpr,
    ) -> etir::BinarySegmentExpr {
        let value = self.lower_expr(segment.value);
        let layout = segment.layout.with_size_expr(|size| self.lower_expr(size));
        etir::BinarySegmentExpr { value, layout, span: segment.span }
    }

    fn lower_expr_tuple(&mut self, expr: &thir::Expr<'tcx>, fields: &[thir::ExprId]) -> ExprId {
        with_expr_block!(self, |expr_body| {
            let fields = fields
                .iter()
                .map(|&field| expr_body.lower_and_maybe_bind(self, sym::element, field))
                .collect();

            self.push_expr(etir::ExprKind::Tuple { fields }, expr.ty, expr.span)
        })
    }

    fn lower_expr_field(
        &mut self,
        expr: &thir::Expr<'tcx>,
        lhs: thir::ExprId,
        variant_index: rustc_abi::VariantIdx,
        name: rustc_abi::FieldIdx,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;
        let ty = self.thir[lhs].ty;

        if let ty::Tuple(_) = ty.kind() {
            let position = etir::FieldIdx::from_u32(name.as_u32());
            let kind = etir::ExprKind::TupleField { lhs: self.lower_expr(lhs), position };
            return self.push_expr(kind, expr.ty, span);
        }

        let ty::Adt(adt_def, _) = *ty.kind() else {
            not_implemented_kind(tcx, span, "field access on non-ADT type", ty);
        };

        let variant = adt_def.variant(variant_index);
        let lhs = self.lower_expr(lhs);

        let Some(position) = tcx.erl_runtime_field_position(variant, name) else {
            if !expr.ty.is_phantom_data() {
                span_bug!(span, "field without a runtime value has type {}", expr.ty)
            }
            let tag = self.lower_expr_phantom_data(expr);
            return with_expr_block!(self, |expr_body| {
                // The lhs may have side effects, such as in `fetch_struct().phantom`.
                expr_body.maybe_bind(self, sym::field_base, lhs);
                tag
            });
        };

        let kind = etir::ExprKind::Field {
            lhs,
            repr: self.lower_adt_repr(span, adt_def, variant),
            position: etir::FieldIdx::from_usize(position),
            name: variant.fields[name].name,
        };
        self.push_expr(kind, expr.ty, span)
    }

    fn lower_expr_binary_op(
        &mut self,
        expr: &thir::Expr<'tcx>,
        op: BinOp,
        lhs: thir::ExprId,
        rhs: thir::ExprId,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let lhs_ty = self.thir[lhs].ty;
        let lhs = self.lower_expr(lhs);
        let rhs = self.lower_expr(rhs);

        // String, binary and bitstring concatenation lowers to binary construction.
        if op == BinOp::Add && lhs_ty.kind().bitstring_rank().is_some() {
            let (ty, unit) = if lhs_ty.is_bits() {
                (BinarySegmentTy::Bits, 1)
            } else {
                (BinarySegmentTy::Bin, 8)
            };
            let layout = BinarySegmentLayout {
                ty,
                size: BinarySegmentSize::OfValue,
                unit,
                signed: false,
                endianness: BinaryEndianness::Big,
            };
            let segments = Box::new([
                etir::BinarySegmentExpr { value: lhs, layout, span: self.cx.body[lhs].span },
                etir::BinarySegmentExpr { value: rhs, layout, span: self.cx.body[rhs].span },
            ]);
            return self.push_expr(etir::ExprKind::BinaryPattern { segments }, expr.ty, expr.span);
        }

        use BinOp::*;
        use etir::BinaryOp;
        let op = match op {
            Add if lhs_ty.is_array() || lhs_ty.is_array_slice() => BinaryOp::Concat,
            Add => BinaryOp::Add,
            Sub if lhs_ty.is_array() || lhs_ty.is_array_slice() => BinaryOp::Subtract,
            Sub => BinaryOp::Sub,
            Mul => BinaryOp::Mul,
            Div if lhs_ty.is_floating_point() => BinaryOp::Div,
            Div => BinaryOp::DivInt,
            Rem => BinaryOp::Rem,
            BitXor => BinaryOp::Bxor,
            BitAnd => BinaryOp::Band,
            BitOr => BinaryOp::Bor,
            Shl => BinaryOp::Bsl,
            Shr => BinaryOp::Bsr,
            Eq => BinaryOp::EqExact,
            Ne => BinaryOp::NeExact,
            Lt => BinaryOp::Lt,
            Le => BinaryOp::Le,
            Ge => BinaryOp::Ge,
            Gt => BinaryOp::Gt,
            Cmp => not_implemented_kind(tcx, expr.span, "binary operator", op),
            // rustc leftovers
            AddUnchecked | SubUnchecked | MulUnchecked | AddWithOverflow | SubWithOverflow
            | MulWithOverflow | ShlUnchecked | ShrUnchecked | Offset => {
                tcx.dcx().span_fatal(expr.span, format!("binary operator not supported: {op:?}"))
            }
        };

        self.push_expr(etir::ExprKind::BinaryOp { op, lhs, rhs }, expr.ty, expr.span)
    }

    fn lower_expr_logical_op(
        &mut self,
        expr: &thir::Expr<'tcx>,
        op: thir::LogicalOp,
        lhs: thir::ExprId,
        rhs: thir::ExprId,
    ) -> ExprId {
        let span = expr.span;
        let lhs = self.lower_expr(lhs);

        let (rhs, reassigned) = self.track_reassigns(|cx| cx.lower_expr(rhs));

        if !reassigned {
            let op = match op {
                thir::LogicalOp::And => etir::LogicalOp::And,
                thir::LogicalOp::Or => etir::LogicalOp::Or,
            };
            return self.push_expr(etir::ExprKind::LogicalOp { op, lhs, rhs }, expr.ty, expr.span);
        }

        let then_pattern = self.bool_pat(span, true);

        let (then_body, else_body) = match op {
            thir::LogicalOp::And => (rhs, self.bool_expr(span, false)),
            thir::LogicalOp::Or => (self.bool_expr(span, true), rhs),
        };

        self.ifelse_match_expr(expr, lhs, then_pattern, then_body, else_body)
    }

    fn lower_expr_unary_op(
        &mut self,
        expr: &thir::Expr<'tcx>,
        op: UnOp,
        arg: thir::ExprId,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let op = match op {
            UnOp::Not if self.thir[arg].ty.is_bool() => etir::UnaryOp::Not,
            UnOp::Not => etir::UnaryOp::Bnot,
            UnOp::Neg => etir::UnaryOp::Neg,
            // rustc leftover
            UnOp::PtrMetadata => {
                tcx.dcx().span_fatal(expr.span, format!("unary operator not supported: {op:?}"))
            }
        };
        let kind = etir::ExprKind::UnaryOp { op, arg: self.lower_expr(arg) };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_cast(&mut self, expr: &thir::Expr<'tcx>, source: thir::ExprId) -> ExprId {
        let source_ty = self.thir[source].ty;
        let source = self.lower_expr(source);

        let kind = if source_ty.is_floating_point() && expr.ty.is_integral() {
            etir::CastKind::FloatToInt
        } else if source_ty.is_integral() && expr.ty.is_floating_point() {
            etir::CastKind::IntToFloat
        } else {
            etir::CastKind::Identity
        };
        self.push_expr(etir::ExprKind::Cast { source, kind }, expr.ty, expr.span)
    }

    fn lower_expr_call(
        &mut self,
        expr: &thir::Expr<'tcx>,
        callable_ty: Ty<'tcx>,
        fun: thir::ExprId,
        args: &[thir::ExprId],
        forced_extern: bool,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;

        let owner = self.owner.to_def_id();
        let target = if let ty::FnDef(callee_def_id, generic_args) = callable_ty.kind() {
            if tcx.is_fn_trait(tcx.parent(*callee_def_id)) {
                return self.lower_expr_fn_trait_call(expr, args);
            }
            if matches!(tcx.def_kind(*callee_def_id), DefKind::Ctor(..)) {
                not_implemented(tcx, span, "calling a constructor through a variable");
            }
            if let Some(intrinsic) = tcx.intrinsic(*callee_def_id) {
                return self.lower_expr_intrinsic_call(expr, intrinsic.name, args);
            }

            tcx.erl_call_target_for_def(owner, *callee_def_id, generic_args)
        } else if let thir::ExprKind::ExternMod { def_id } = &self.resolve(fun).kind {
            tcx.erl_call_target_for_def(owner, *def_id, ty::GenericArgs::empty())
        } else {
            let callee = etir::Callee::Value(self.lower_expr(fun));
            let kind = etir::ExprKind::Call { callee, args: self.lower_exprs(args) };
            return self.push_expr(kind, expr.ty, span);
        };

        // Temporary before we actually ship runtime modules (like `patina_fmt` could've been)
        if let ErlCallTarget::Extern { module, function } = target {
            let mod_name = tcx.erl_module_name(ErlModuleId::Extern(module));
            let fn_name = tcx.item_name(function);
            if let (sym::patina_fmt, sym::display | sym::debug) = (mod_name, fn_name) {
                return self.lower_expr_format_render(expr, fn_name, args[0]);
            }
        }

        let callee = match self.witnessless_erl_fun(span, target, args.len() as u32) {
            Some(fun) => etir::Callee::Fun { fun, forced_extern },
            None => etir::Callee::Item { item: self.item_ref(span, target), forced_extern },
        };
        let kind = etir::ExprKind::Call { callee, args: self.lower_exprs(args) };
        self.push_expr(kind, expr.ty, span)
    }

    fn lower_expr_intrinsic_call(
        &mut self,
        expr: &thir::Expr<'tcx>,
        name: Symbol,
        args: &[thir::ExprId],
    ) -> ExprId {
        let tcx = self.cx.tcx;
        match (name, args) {
            (sym::cast_unchecked, &[value]) => {
                let kind = etir::ExprKind::Cast {
                    source: self.lower_expr(value),
                    kind: etir::CastKind::Identity,
                };
                self.push_expr(kind, expr.ty, expr.span)
            }
            _ => not_implemented_kind(tcx, expr.span, "intrinsic", name),
        }
    }

    fn lower_expr_fn_trait_call(
        &mut self,
        expr: &thir::Expr<'tcx>,
        args: &[thir::ExprId],
    ) -> ExprId {
        let &[callee, tuple] = args else {
            span_bug!(expr.span, "expected 2 arguments for fn trait call");
        };

        let thir::ExprKind::Tuple { fields } = &self.resolve(tuple).kind else {
            span_bug!(expr.span, "expected tuple for fn trait call")
        };

        let callee = etir::Callee::Value(self.lower_expr(callee));
        let kind = etir::ExprKind::Call { callee, args: self.lower_exprs(fields) };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_format_render(
        &mut self,
        expr: &thir::Expr<'tcx>,
        fn_name: Symbol,
        arg: thir::ExprId,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;
        let (_, Some((_, hir_id))) = self.resolve_expr_with_scope(arg) else {
            span_bug!(span, "format argument must be a HIR-derived expression");
        };
        let ty = tcx.typeck(self.owner).node_type(hir_id);

        let arg = self.lower_expr(arg);
        if fn_name == sym::display && ty.is_str() {
            return arg;
        }

        let format = self.push_expr(
            etir::ExprKind::Literal(etir::Lit::Charlist(sym::debug_format_str), span),
            Ty::new_slice(tcx, tcx.types.char),
            span,
        );

        let args = self.push_expr(
            etir::ExprKind::List { items: Box::new([arg]), tail: None },
            Ty::new_slice(tcx, self.cx.body[arg].ty),
            span,
        );

        self.call_io_lib_format(expr, format, args)
    }

    fn lower_expr_closure(
        &mut self,
        expr: &thir::Expr<'tcx>,
        closure: &thir::ClosureExpr<'tcx>,
    ) -> ExprId {
        let (params, body) = ThirBodyCx::lower(self.cx, closure.closure_id);
        let kind = etir::ExprKind::Closure { def_id: closure.closure_id.to_def_id(), params, body };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_zst_literal(&mut self, expr: &thir::Expr<'tcx>) -> ExprId {
        let tcx = self.cx.tcx;

        let (def_id, args) = if let ty::FnDef(def_id, args) = *expr.ty.kind()
            && let DefKind::Fn | DefKind::AssocFn = tcx.def_kind(def_id)
        {
            (def_id, args)
        } else {
            not_implemented_kind(tcx, expr.span, "ZST literal", expr.ty)
        };
        if let Some(intrinsic) = tcx.intrinsic(def_id) {
            not_implemented_kind(tcx, expr.span, "intrinsic as a value", intrinsic.name);
        }

        let target = tcx.erl_call_target_for_def(self.owner.to_def_id(), def_id, args);
        let arity = tcx.fn_sig(def_id).skip_binder().inputs().skip_binder().len() as u32;

        // TODO: add an attribute to allow forcing `fnref` to be an extern call
        let kind = match self.witnessless_erl_fun(expr.span, target, arity) {
            Some(fun) => etir::ExprKind::Fun { fun, forced_extern: false },
            None => {
                let item = self.item_ref(expr.span, target);
                etir::ExprKind::FnItem { item, arity, forced_extern: false }
            }
        };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_named_const(
        &mut self,
        expr: &thir::Expr<'tcx>,
        def_id: rustc_hir::def_id::DefId,
        args: GenericArgsRef<'tcx>,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;

        // An associated const of a trait is a function of the witnesses of its impl.
        if matches!(tcx.def_kind(def_id), DefKind::AssocConst { .. })
            && tcx.erl_entry_trait(def_id).is_some()
        {
            let target = tcx.erl_call_target_for_def(self.owner.to_def_id(), def_id, args);
            let item = self.item_ref(span, target);
            let callee = etir::Callee::Item { item, forced_extern: false };
            let kind = etir::ExprKind::Call { callee, args: Box::new([]) };
            return self.push_expr(kind, expr.ty, span);
        }

        if !args.is_empty() {
            not_implemented_kind(tcx, span, "generic named constants", args);
        }

        if !def_id.is_local() {
            not_implemented_kind(tcx, span, "external named constants", def_id);
        }

        self.push_expr(etir::ExprKind::Const { def_id }, expr.ty, span)
    }

    fn lower_expr_adt(
        &mut self,
        expr: &thir::Expr<'tcx>,
        adt_expr: &thir::AdtExpr<'tcx>,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;
        let adt_def = adt_expr.adt_def;
        let variant = adt_def.variant(adt_expr.variant_index);

        with_expr_block!(self, |expr_body| {
            let repr = self.lower_adt_repr(span, adt_def, variant);

            let base = match &adt_expr.base {
                AdtExprBase::None => None,
                AdtExprBase::Base(base) => {
                    Some(expr_body.lower_and_maybe_bind(self, sym::field_base, base.base))
                }
                AdtExprBase::DefaultFields(_) => {
                    not_implemented_kind(tcx, span, "default fields", repr)
                }
            };

            // In declaration order.
            let fields: Box<[_]> = variant
                .fields
                .iter_enumerated()
                .filter_map(|(idx, field_def)| {
                    let field = adt_expr.fields.iter().find(|field| field.name == idx)?;
                    let value = expr_body.lower_and_maybe_bind(self, field_def.name, field.expr);
                    let Some(position) = tcx.erl_runtime_field_position(variant, idx) else {
                        // The binding for `field.expr` was still emitted to `expr_body`
                        // to account for any side effects, even though we don't emit the field.
                        return None;
                    };
                    Some(etir::FieldExpr {
                        position: etir::FieldIdx::from_usize(position),
                        name: field_def.name,
                        value,
                    })
                })
                .collect();

            let adt = etir::AdtExpr { repr, base, fields };
            self.push_expr(etir::ExprKind::Adt(Box::new(adt)), expr.ty, span)
        })
    }

    fn lower_expr_assign(
        &mut self,
        expr: &thir::Expr<'tcx>,
        lhs: &thir::Pat<'tcx>,
        rhs: thir::ExprId,
    ) -> ExprId {
        let rhs = self.lower_expr(rhs);
        let lhs = self.lower_pat(lhs, PatternInput::Typed);
        self.cx.reassign_count += 1;
        self.push_expr(etir::ExprKind::Assign { lhs, rhs }, expr.ty, expr.span)
    }

    fn lower_expr_if(
        &mut self,
        expr: &thir::Expr<'tcx>,
        cond: thir::ExprId,
        then: thir::ExprId,
        else_opt: Option<thir::ExprId>,
    ) -> ExprId {
        let cond = self.lower_cond(cond);

        let then_body = self.lower_expr(then);
        let else_body = match else_opt {
            Some(else_expr) => self.lower_expr(else_expr),
            None => self.unit_expr(expr.span),
        };

        self.logical_match_expr(expr, cond, then_body, else_body)
    }

    fn lower_cond(&mut self, cond: thir::ExprId) -> Cond {
        let expr = self.resolve(cond);
        match expr.kind {
            thir::ExprKind::Let { expr: scrutinee, ref pat } => {
                let scrutinee = self.lower_expr(scrutinee);
                let pattern = self.lower_pat(pat, PatternInput::Typed);
                Cond::Test { scrutinee, pattern }
            }

            thir::ExprKind::LogicalOp { op, lhs, rhs } => {
                let lhs = self.lower_cond(lhs);
                let (rhs, rhs_reassigns) = self.track_reassigns(|cx| cx.lower_cond(rhs));
                match (lhs, rhs) {
                    // Two simple Cond::Test arms can be joined into a single Cond::Test.
                    (
                        Cond::Test { scrutinee: lhs, pattern: lhs_pattern },
                        Cond::Test { scrutinee: rhs, pattern: rhs_pattern },
                    ) if self.is_literal_bool_pat(lhs_pattern, true)
                        && self.is_literal_bool_pat(rhs_pattern, true)
                        && !rhs_reassigns =>
                    {
                        let op = match op {
                            thir::LogicalOp::And => etir::LogicalOp::And,
                            thir::LogicalOp::Or => etir::LogicalOp::Or,
                        };
                        let scrutinee = self.push_expr(
                            etir::ExprKind::LogicalOp { op, lhs, rhs },
                            expr.ty,
                            expr.span,
                        );
                        Cond::Test { scrutinee, pattern: lhs_pattern }
                    }

                    (lhs, rhs) => match op {
                        thir::LogicalOp::And => Cond::And(Box::new(lhs), Box::new(rhs)),
                        thir::LogicalOp::Or => Cond::Or(Box::new(lhs), Box::new(rhs)),
                    },
                }
            }

            thir::ExprKind::Unary { op: UnOp::Not, arg } => match self.lower_cond(arg) {
                Cond::Test { scrutinee, pattern } if self.is_literal_bool_pat(pattern, true) => {
                    let kind = etir::ExprKind::UnaryOp { op: etir::UnaryOp::Not, arg: scrutinee };
                    Cond::Test { scrutinee: self.push_expr(kind, expr.ty, expr.span), pattern }
                }
                arg => Cond::Not(Box::new(arg)),
            },

            _ => Cond::Test {
                scrutinee: self.lower_expr(cond),
                pattern: self.bool_pat(expr.span, true),
            },
        }
    }

    fn logical_match_expr(
        &mut self,
        expr: &thir::Expr<'tcx>,
        cond: Cond,
        mut then_body: ExprId,
        mut else_body: ExprId,
    ) -> ExprId {
        match cond {
            Cond::Test { scrutinee, pattern } => {
                self.ifelse_match_expr(expr, scrutinee, pattern, then_body, else_body)
            }

            Cond::And(lhs, rhs) => {
                let join = self.outline_to_join(&mut else_body);
                let then = self.logical_match_expr(expr, *rhs, then_body, else_body);
                let value = self.logical_match_expr(expr, *lhs, then, else_body);
                self.wrap_in_joins(join, value)
            }

            Cond::Or(lhs, rhs) => {
                let join = self.outline_to_join(&mut then_body);
                let otherwise = self.logical_match_expr(expr, *rhs, then_body, else_body);
                let value = self.logical_match_expr(expr, *lhs, then_body, otherwise);
                self.wrap_in_joins(join, value)
            }

            Cond::Not(cond) => self.logical_match_expr(expr, *cond, else_body, then_body),
        }
    }

    fn outline_to_join(&mut self, expr: &mut ExprId) -> Option<JoinId> {
        let etir::Expr { ty, span, ref kind } = self.cx.body[*expr];
        if matches!(kind, etir::ExprKind::Jump { .. }) {
            return None;
        }
        let join = self.push_join(*expr, None);
        *expr = self.push_expr(etir::ExprKind::Jump { target: join, value: None }, ty, span);
        Some(join)
    }

    fn ifelse_match_expr(
        &mut self,
        expr: &thir::Expr<'tcx>,
        scrutinee: ExprId,
        then_pattern: PatId,
        then_body: ExprId,
        else_body: ExprId,
    ) -> ExprId {
        let span = expr.span;
        let then_span = self.cx.body[then_body].span;
        let else_span = self.cx.body[else_body].span;

        let wild = etir::PatKind::Wild { input: PatternInput::Typed };
        let else_pattern = self.push_pat(wild, self.cx.body[scrutinee].ty, else_span);

        let arms = Box::new([
            etir::Arm { pattern: then_pattern, guard: None, body: then_body, span: then_span },
            etir::Arm { pattern: else_pattern, guard: None, body: else_body, span: else_span },
        ]);
        self.push_expr(etir::ExprKind::Match { scrutinee, arms }, expr.ty, span)
    }

    fn lower_expr_receive(
        &mut self,
        expr: &thir::Expr<'tcx>,
        arms: &[thir::ArmId],
        timeout: Option<&thir::ReceiveTimeout>,
    ) -> ExprId {
        let after = timeout.map(|timeout| etir::ReceiveAfter {
            timeout: self.lower_expr(timeout.expr),
            body: self.lower_expr(timeout.body),
        });
        let arms = arms.iter().map(|&arm| self.lower_arm(arm)).collect();
        let kind = etir::ExprKind::Receive { arms, after };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_expr_match(
        &mut self,
        expr: &thir::Expr<'tcx>,
        scrutinee: thir::ExprId,
        arms: &[thir::ArmId],
    ) -> ExprId {
        let scrutinee = self.lower_expr(scrutinee);

        let mut joins = Vec::new();
        let arms = arms
            .iter()
            .map(|&arm| {
                let mut arm = self.lower_arm(arm);
                if self.has_or_pattern(arm.pattern) {
                    joins.extend(self.outline_to_join(&mut arm.body));
                }
                arm
            })
            .collect();

        let value = self.push_expr(etir::ExprKind::Match { scrutinee, arms }, expr.ty, expr.span);
        self.wrap_in_joins(joins, value)
    }

    fn has_or_pattern(&self, pat: PatId) -> bool {
        struct FindOr<'a, 'tcx> {
            body: &'a etir::Body<'tcx>,
            found: bool,
        }

        impl<'a, 'tcx: 'a> Visitor<'a, 'tcx> for FindOr<'a, 'tcx> {
            fn etir(&self) -> &'a etir::Body<'tcx> {
                self.body
            }

            fn visit_pat(&mut self, _id: PatId, pat: &'a etir::Pat<'tcx>) {
                if let etir::PatKind::Or { .. } = pat.kind {
                    self.found = true;
                } else if !self.found {
                    walk_pat(self, pat);
                }
            }
        }

        let mut visitor = FindOr { body: &self.cx.body, found: false };
        visit::visit_pat_id(&mut visitor, pat);
        visitor.found
    }

    fn lower_arm(&mut self, arm: thir::ArmId) -> etir::Arm {
        let arm = &self.thir[arm];

        let pattern = self.lower_pat(&arm.pattern, PatternInput::Typed);
        let guard = arm.guard.map(|guard| self.lower_expr(guard));
        let body = self.lower_expr(arm.body);

        etir::Arm { pattern, guard, body, span: arm.span }
    }

    fn lower_expr_try_catch(
        &mut self,
        expr: &thir::Expr<'tcx>,
        body: thir::ExprId,
        arms: &[thir::ArmId],
        after: Option<thir::ExprId>,
    ) -> ExprId {
        let body = self.lower_expr(body);
        let after = after.map(|after| self.lower_expr(after));
        let arms = arms.iter().map(|&arm| self.lower_catch_arm(arm)).collect();
        let kind = etir::ExprKind::TryCatch { body, arms, after };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn lower_catch_arm(&mut self, arm: thir::ArmId) -> etir::CatchArm {
        let arm = &self.thir[arm];
        let thir::PatKind::Exception { class_pat, reason_pat, .. } = &arm.pattern.kind else {
            span_bug!(arm.pattern.span, "catch arm without exception pattern")
        };

        let class = self.lower_pat(class_pat, PatternInput::Typed);
        let reason = self.lower_pat(reason_pat, PatternInput::Typed);
        let guard = arm.guard.map(|guard| self.lower_expr(guard));

        let stacktrace = arm.stacktrace.as_deref().and_then(|pat| match pat.kind {
            thir::PatKind::Binding { var, is_primary: true, subpattern: None, .. } => {
                Some(self.var(var))
            }
            thir::PatKind::Wild => None,
            _ => span_bug!(pat.span, "stacktrace pattern isn't a variable or `_`"),
        });

        let body = self.lower_expr(arm.body);

        etir::CatchArm { class, reason, guard, stacktrace, body, span: arm.span }
    }

    fn lower_expr_return(
        &mut self,
        expr: &thir::Expr<'tcx>,
        value: Option<thir::ExprId>,
    ) -> ExprId {
        let value = value.map(|value| self.lower_expr(value));
        let join = self.return_join.unwrap_or_else(|| {
            let join = self.push_value_join(value, expr.span);
            self.return_join = Some(join);
            join
        });
        self.jump(expr, join, value)
    }

    fn lower_expr_loop(
        &mut self,
        expr: &thir::Expr<'tcx>,
        region_scope: Option<(region::Scope, HirId)>,
        body: thir::ExprId,
    ) -> ExprId {
        let Some((label, _)) = region_scope else {
            span_bug!(expr.span, "HIR-derived loop has no outer Scope")
        };

        let loop_id = self.push_loop();
        let body = self.lower_expr(body);
        // `continue` results in a Join inside the loop body...
        let continue_join = self.cx.continue_joins.remove(&label);
        let body = self.wrap_in_joins(continue_join, body);

        // ...while `break` builds a Join outside of it.
        let value = self.push_expr(etir::ExprKind::Loop { id: loop_id, body }, expr.ty, expr.span);
        let break_join = self.cx.break_joins.remove(&label);
        self.wrap_in_joins(break_join, value)
    }

    fn lower_expr_break(
        &mut self,
        expr: &thir::Expr<'tcx>,
        label: region::Scope,
        value: Option<thir::ExprId>,
    ) -> ExprId {
        let value = value.map(|value| self.lower_expr(value));
        let join = self.cx.break_joins.get(&label).copied().unwrap_or_else(|| {
            let join = self.push_value_join(value, expr.span);
            self.cx.break_joins.insert(label, join);
            join
        });
        self.jump(expr, join, value)
    }

    fn lower_expr_continue(&mut self, expr: &thir::Expr<'tcx>, label: region::Scope) -> ExprId {
        let join = self.cx.continue_joins.get(&label).copied().unwrap_or_else(|| {
            // The join returns nothing, breaking out of `case` and into the loop tail call.
            let unit = self.unit_expr(expr.span);
            let join = self.push_join(unit, None);
            self.cx.continue_joins.insert(label, join);
            join
        });
        self.jump(expr, join, None)
    }

    fn push_value_join(&mut self, value: Option<ExprId>, span: Span) -> JoinId {
        let ty = value.map_or(self.cx.tcx.types.unit, |value| self.cx.body[value].ty);

        // unit return value is synthesized at the jump.
        if ty.is_unit() || ty.is_never() {
            let body = self.unit_expr(span);
            return self.push_join(body, None);
        }

        let var = self.push_var(Ident { name: sym::jump_value, span });
        let body = self.push_expr(etir::ExprKind::Var(var), ty, span);
        self.push_join(body, Some(var))
    }

    fn jump(&mut self, expr: &thir::Expr<'tcx>, join: JoinId, value: Option<ExprId>) -> ExprId {
        with_expr_block!(self, |block| {
            let value = value.and_then(|value| {
                // Possibly only for a side effect, if the join takes no value.
                let var = block.maybe_bind(self, sym::jump_value, value);
                self.cx.body[join].value.map(|_| var)
            });

            self.push_expr(etir::ExprKind::Jump { target: join, value }, expr.ty, expr.span)
        })
    }

    fn call_io_lib_format(
        &mut self,
        expr: &thir::Expr<'tcx>,
        format: ExprId,
        args: ExprId,
    ) -> ExprId {
        let callee = etir::Callee::Direct(sym::io_lib, sym::format);
        let kind = etir::ExprKind::Call { callee, args: Box::new([format, args]) };
        self.push_expr(kind, expr.ty, expr.span)
    }

    fn unit_expr(&mut self, span: Span) -> ExprId {
        let tcx = self.cx.tcx;
        let unit = etir::ExprKind::Tuple { fields: Box::new([]) };
        self.push_expr(unit, tcx.types.unit, span)
    }

    fn item_ref(&self, span: Span, target: ErlCallTarget<'tcx>) -> Box<etir::ItemRef<'tcx>> {
        let (module, name) = self.target_module_and_name(span, target);
        Box::new(etir::ItemRef { name, target, module })
    }

    fn bool_expr(&mut self, span: Span, value: bool) -> ExprId {
        let kind = etir::ExprKind::Literal(etir::Lit::Bool(value), span);
        self.push_expr(kind, self.cx.tcx.types.bool, span)
    }

    fn witnessless_erl_fun(
        &self,
        span: Span,
        target: ErlCallTarget<'tcx>,
        arity: u32,
    ) -> Option<etir::ErlFun> {
        if let ErlCallTarget::Item { def_id, .. } = target
            && self.cx.tcx.erl_needs_witnesses(def_id)
        {
            return None;
        }

        let (module, name) = self.target_module_and_name(span, target);
        let module =
            module.unwrap_or_else(|| span_bug!(span, "`{name}` dispatches without a self witness"));

        Some(etir::ErlFun { module, name, arity })
    }

    fn target_module_and_name(
        &self,
        span: Span,
        target: ErlCallTarget<'tcx>,
    ) -> (Option<ErlModuleResolution>, Symbol) {
        let tcx = self.cx.tcx;
        let owner = self.owner.to_def_id();
        let module = tcx.erl_module_for_target(owner, target);
        let name = match target {
            ErlCallTarget::Item { def_id, .. } => tcx.erl_fn_name(def_id),
            ErlCallTarget::Extern { function, .. } => tcx.item_name(function),
            ErlCallTarget::Constructor { .. } | ErlCallTarget::Dynamic => {
                span_bug!(span, "item reference to {target:?}")
            }
        };
        (module, name)
    }

    fn resolve_expr_with_scope(
        &self,
        expr: thir::ExprId,
    ) -> (&'a thir::Expr<'tcx>, Option<(region::Scope, HirId)>) {
        let mut expr = &self.thir[expr];
        let mut region_scope = None;
        loop {
            match &expr.kind {
                thir::ExprKind::Scope { region_scope: scope, hir_id, value, .. } => {
                    region_scope = Some((*scope, *hir_id));
                    expr = &self.thir[*value];
                }
                thir::ExprKind::Use { source }
                | thir::ExprKind::ByUse { expr: source, .. }
                | thir::ExprKind::NeverToAny { source }
                | thir::ExprKind::PointerCoercion { source, .. }
                | thir::ExprKind::PlaceTypeAscription { source, .. }
                | thir::ExprKind::ValueTypeAscription { source, .. } => expr = &self.thir[*source],

                _ => return (expr, region_scope),
            }
        }
    }

    fn resolve(&self, expr: thir::ExprId) -> &'a thir::Expr<'tcx> {
        self.resolve_expr_with_scope(expr).0
    }
}

enum Cond {
    Test { scrutinee: ExprId, pattern: PatId },
    And(Box<Cond>, Box<Cond>),
    Or(Box<Cond>, Box<Cond>),
    Not(Box<Cond>),
}
