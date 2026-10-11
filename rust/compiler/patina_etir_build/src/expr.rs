use rustc_ast::LitKind;
use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_hir::def::DefKind;
use rustc_hir::{BinarySegmentTy, HirId};
use rustc_middle::erl::binary_pattern::{BinaryEndianness, BinarySegmentLayout, BinarySegmentSize};
use rustc_middle::erl::etir::{self, ExprId, PatId, PatternInput};
use rustc_middle::erl::witness::WitnessArgs;
use rustc_middle::erl::{ErlCallTarget, ErlModuleId};
use rustc_middle::middle::region;
use rustc_middle::mir::{BinOp, UnOp};
use rustc_middle::span_bug;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{self, AdtExprBase, LocalVarId, PatKind, Thir};
use rustc_middle::ty::{self, GenericArgsRef, Ty};
use rustc_span::{Span, Symbol, sym};

use super::{ThirBodyCx, not_implemented, not_implemented_kind};

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
        self.push_expr(etir::ExprKind::Block(exprs.into_boxed_slice()), ty, span)
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
        let tail = tail.map(|tail| self.lower_expr(tail));
        let items = self.lower_exprs(items);
        self.push_expr(etir::ExprKind::List { items, tail }, expr.ty, expr.span)
    }

    fn lower_expr_map(
        &mut self,
        expr: &thir::Expr<'tcx>,
        fields: &[thir::MapFieldExpr],
        base: Option<thir::ExprId>,
    ) -> ExprId {
        let base = base.map(|base| self.lower_expr(base));
        let fields = fields
            .iter()
            .map(|field| (self.lower_expr(field.key), self.lower_expr(field.value)))
            .collect();
        self.push_expr(etir::ExprKind::Map { fields, base }, expr.ty, expr.span)
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
        let fields = self.lower_exprs(fields);
        self.push_expr(etir::ExprKind::Tuple { fields }, expr.ty, expr.span)
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
            return self.wrap_side_effects(&[lhs], tag);
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
        let op = match op {
            thir::LogicalOp::And => etir::LogicalOp::And,
            thir::LogicalOp::Or => etir::LogicalOp::Or,
        };
        let kind =
            etir::ExprKind::LogicalOp { op, lhs: self.lower_expr(lhs), rhs: self.lower_expr(rhs) };
        self.push_expr(kind, expr.ty, expr.span)
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

        let callee = etir::Callee::Item { item: self.item_ref(span, target), forced_extern };
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
        let kind = etir::ExprKind::FnRef {
            item: self.item_ref(expr.span, target),
            arity: tcx.fn_sig(def_id).skip_binder().inputs().skip_binder().len(),
            erl_arity: tcx.erl_fn_arity(def_id) as u32,
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

        let mut side_effects = Vec::new();
        for field in &adt_expr.fields {
            if tcx.erl_runtime_field_position(variant, field.name).is_none() {
                side_effects.push(self.lower_expr(field.expr));
            }
        }

        let repr = self.lower_adt_repr(span, adt_def, variant);

        // In declaration order.
        let fields: Box<[_]> = tcx
            .erl_runtime_fields(variant)
            .enumerate()
            .filter_map(|(position, (idx, field_def))| {
                let field = adt_expr.fields.iter().find(|field| field.name == idx)?;
                Some(etir::FieldExpr {
                    position: etir::FieldIdx::from_usize(position),
                    name: field_def.name,
                    value: self.lower_expr(field.expr),
                })
            })
            .collect();

        let base = match &adt_expr.base {
            AdtExprBase::None => None,
            AdtExprBase::Base(base) => Some(self.lower_expr(base.base)),
            AdtExprBase::DefaultFields(_) => {
                not_implemented_kind(tcx, span, "default fields", repr)
            }
        };

        let adt = etir::AdtExpr { repr, base, fields };
        let adt = self.push_expr(etir::ExprKind::Adt(Box::new(adt)), expr.ty, span);
        self.wrap_side_effects(&side_effects, adt)
    }

    fn lower_expr_assign(
        &mut self,
        expr: &thir::Expr<'tcx>,
        lhs: &thir::Pat<'tcx>,
        rhs: thir::ExprId,
    ) -> ExprId {
        let rhs = self.lower_expr(rhs);
        let lhs = self.lower_pat(lhs, PatternInput::Typed);
        self.push_expr(etir::ExprKind::Assign { lhs, rhs }, expr.ty, expr.span)
    }

    fn lower_expr_if(
        &mut self,
        expr: &thir::Expr<'tcx>,
        cond: thir::ExprId,
        then: thir::ExprId,
        else_opt: Option<thir::ExprId>,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let span = expr.span;

        // TODO: this detection of `ExprKind::Let` has an issue with let chains.
        // We'll want to rewrite this into `maybe`. Possibly, we'll want to rewrite all `case`
        // into `maybe`, if I confirm that it results in the same bytecode.
        // Note that a case without a wildcard sets up for case clause.
        let (scrutinee, then_pattern, is_let) =
            if let thir::ExprKind::Let { expr: scrutinee, pat } = &self.resolve(cond).kind {
                (self.lower_expr(*scrutinee), self.lower_pat(pat, PatternInput::Typed), true)
            } else {
                let bool_true = etir::PatKind::Literal(etir::Lit::Bool(true));
                (self.lower_expr(cond), self.push_pat(bool_true, tcx.types.bool, span), false)
            };

        let then_body = self.lower_expr(then);
        let else_body = match else_opt {
            Some(else_expr) => self.lower_expr(else_expr),
            None => self.unit_expr(span),
        };

        let then_span = self.cx.body[then_body].span;
        let else_span = self.cx.body[else_body].span;
        let else_pattern = if is_let {
            let wild = etir::PatKind::Wild { input: PatternInput::Typed };
            self.push_pat(wild, self.cx.body[scrutinee].ty, else_span)
        } else {
            let bool_false = etir::PatKind::Literal(etir::Lit::Bool(false));
            self.push_pat(bool_false, tcx.types.bool, span)
        };

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
        let arms = arms.iter().map(|&arm| self.lower_arm(arm)).collect();
        let kind = etir::ExprKind::Match { scrutinee, arms };
        self.push_expr(kind, expr.ty, expr.span)
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
        self.push_expr(etir::ExprKind::Return { value }, expr.ty, expr.span)
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

        let outer_vars = LoopVarAnalysis::analyze(self.thir, body)
            .into_iter()
            .map(|(var, reassigned)| {
                if reassigned {
                    etir::LoopOuterVar::Reassigned(self.var(var))
                } else {
                    etir::LoopOuterVar::Referenced(self.var(var))
                }
            })
            .collect();

        let loop_id = self.push_loop(label, outer_vars);
        let body = self.lower_expr(body);
        self.push_expr(etir::ExprKind::Loop { id: loop_id, body }, expr.ty, expr.span)
    }

    fn lower_expr_break(
        &mut self,
        expr: &thir::Expr<'tcx>,
        label: region::Scope,
        value: Option<thir::ExprId>,
    ) -> ExprId {
        let tcx = self.cx.tcx;
        let value = value.map(|value| self.lower_expr(value));
        let Some(&target) = self.cx.loop_ids.get(&label) else {
            not_implemented(tcx, expr.span, "break target outside of loop")
        };
        self.push_expr(etir::ExprKind::Break { target, value }, expr.ty, expr.span)
    }

    fn lower_expr_continue(&mut self, expr: &thir::Expr<'tcx>, label: region::Scope) -> ExprId {
        let Some(&target) = self.cx.loop_ids.get(&label) else {
            span_bug!(expr.span, "`continue` to something other than a loop")
        };
        self.push_expr(etir::ExprKind::Continue { target }, expr.ty, expr.span)
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

    fn wrap_side_effects(&mut self, side_effects: &[ExprId], value: ExprId) -> ExprId {
        if side_effects.is_empty() {
            return value;
        }

        let etir::Expr { ty, span, .. } = self.cx.body[value];
        let exprs = side_effects
            .iter()
            .map(|&side_effect| self.underscore_assign(side_effect))
            .chain([value])
            .collect();

        self.push_expr(etir::ExprKind::Block(exprs), ty, span)
    }

    fn underscore_assign(&mut self, value: ExprId) -> ExprId {
        let etir::Expr { ty, span, .. } = self.cx.body[value];
        let wild = self.push_pat(etir::PatKind::Wild { input: PatternInput::Typed }, ty, span);
        self.push_expr(etir::ExprKind::Assign { lhs: wild, rhs: value }, ty, span)
    }

    fn unit_expr(&mut self, span: Span) -> ExprId {
        let tcx = self.cx.tcx;
        let unit = etir::ExprKind::Tuple { fields: Box::new([]) };
        self.push_expr(unit, tcx.types.unit, span)
    }

    fn item_ref(&mut self, span: Span, target: ErlCallTarget<'tcx>) -> Box<etir::ItemRef<'tcx>> {
        let tcx = self.cx.tcx;
        let owner = self.owner.to_def_id();
        let module = tcx.erl_module_for_target(owner, target);
        let (name, witnesses) = match target {
            ErlCallTarget::Item { def_id, args } => {
                (tcx.erl_fn_name(def_id), self.cx.witness_resolver.witness_args(span, def_id, args))
            }
            ErlCallTarget::Extern { function, .. } => {
                (tcx.item_name(function), WitnessArgs::default())
            }
            ErlCallTarget::Constructor { .. } | ErlCallTarget::Dynamic => {
                span_bug!(span, "item reference to {target:?}")
            }
        };
        Box::new(etir::ItemRef { name, target, module, witnesses })
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

struct LoopVarAnalysis<'thir, 'tcx> {
    thir: &'thir Thir<'tcx>,
    outer_vars: FxIndexMap<LocalVarId, bool>,
    bound_in_loop: FxHashSet<LocalVarId>,
}

impl<'thir, 'tcx> LoopVarAnalysis<'thir, 'tcx> {
    fn analyze(thir: &'thir Thir<'tcx>, body: thir::ExprId) -> FxIndexMap<LocalVarId, bool> {
        let mut analysis = LoopVarAnalysis {
            thir,
            outer_vars: FxIndexMap::default(),
            bound_in_loop: FxHashSet::default(),
        };
        analysis.visit_expr(&thir[body]);
        let bound = &analysis.bound_in_loop;
        analysis.outer_vars.retain(|var, _| !bound.contains(var));
        analysis.outer_vars
    }
}

impl<'thir, 'tcx: 'thir> Visitor<'thir, 'tcx> for LoopVarAnalysis<'thir, 'tcx> {
    fn thir(&self) -> &'thir Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'thir thir::Expr<'tcx>) {
        match &expr.kind {
            thir::ExprKind::VarRef { .. } | thir::ExprKind::UpvarRef { .. }
                if expr.ty.is_phantom_data() => {}
            thir::ExprKind::VarRef { id } | thir::ExprKind::UpvarRef { var_hir_id: id, .. } => {
                self.outer_vars.entry(*id).or_insert(false);
            }
            thir::ExprKind::Closure(closure) => {
                // The closure body has its own THIR, but its capture expressions are
                // evaluated in (and therefore must be parameters of) this loop helper.
                for upvar in closure.upvars.iter() {
                    self.visit_expr(&self.thir[*upvar]);
                }
            }
            _ => visit::walk_expr(self, expr),
        }
    }

    fn visit_pat(&mut self, pat: &'thir thir::Pat<'tcx>) {
        match pat.kind {
            PatKind::Pin { .. } | PatKind::Assignee { .. } if pat.ty.is_phantom_data() => {}
            PatKind::Pin { var } => {
                self.outer_vars.entry(var).or_insert(false);
            }
            PatKind::Assignee { var } => {
                self.outer_vars.insert(var, true);
            }
            PatKind::Binding { var, .. } => {
                self.bound_in_loop.insert(var);
            }
            _ => {}
        }
        visit::walk_pat(self, pat);
    }
}
