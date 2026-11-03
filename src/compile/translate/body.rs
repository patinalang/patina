use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::{SmallVec, smallvec};
use rustc_hir::TermClass;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::etir::{
    self, Callee, ErlFun, ExprId, ExprKind, JoinId, Lit, PatId, PatKind, PatternInput, VarId,
};
use rustc_middle::erl::{ErlCallKind, ErlModuleResolution};
use rustc_middle::span_bug;
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::translate::Translator;
use crate::compile::translate::scopes::{PatinaVar, Scope, Scopes};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

/// Pattern with optional guards needed to match it.
#[derive(Debug, Clone)]
pub(super) struct GuardedPattern<'arena>(
    pub(super) eaf::Pattern<'arena>,
    pub(super) SmallVec<[eaf::Expr<'arena>; 1]>,
);
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
    pub owner: DefId,
    pub etir: &'tcx etir::Body<'tcx>,
    pub scopes: &'a mut Scopes,
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn new(
        translator: &'a Translator<'tcx, 'arena>,
        owner: DefId,
        etir: &'tcx etir::Body<'tcx>,
        scopes: &'a mut Scopes,
    ) -> Self {
        BodyCx { translator, owner, etir, scopes }
    }

    pub(super) fn tcx(&self) -> TyCtxt<'tcx> {
        self.translator.tcx
    }

    pub(super) fn translate_body_with_params(
        &mut self,
        params: &[PatId],
        value: ExprId,
    ) -> TranslatedBody<'arena> {
        let params = params.iter().map(|&pat| self.translate_single_pattern(pat)).collect();
        let value = self.translate_expr(value);
        TranslatedBody { params, value }
    }

    pub(super) fn translate_expr(&mut self, expr_id: ExprId) -> eaf::Expr<'arena> {
        let ir = self.etir;
        let expr = &ir[expr_id];
        let span = expr.span;

        match expr.kind {
            ExprKind::Block(ref exprs) => self.translate_block(span, exprs),

            ExprKind::Literal(lit, lit_span) => self.translate_literal(span, lit, lit_span),

            ExprKind::Var(id) => self.translate_var_ref(span, id),

            ExprKind::List { ref items, tail } => self.translate_list(span, items, tail),

            ExprKind::Map { ref fields, base } => self.translate_map(span, fields, base),

            ExprKind::BinaryPattern { ref segments } => {
                self.translate_expr_binary_pattern(span, segments)
            }

            ExprKind::Tuple { ref fields } => self.translate_tuple(span, fields),

            ExprKind::Field { lhs, repr, position, name } => {
                self.translate_field(span, lhs, repr, position, name)
            }

            ExprKind::TupleField { lhs, position } => {
                self.translate_tuple_field(span, lhs, position)
            }

            ExprKind::BinaryOp { op, lhs, rhs } => self.translate_binary(span, op, lhs, rhs),

            ExprKind::LogicalOp { op, lhs, rhs } => self.translate_logical(span, op, lhs, rhs),

            ExprKind::UnaryOp { op, arg } => self.translate_unary(span, op, arg),

            ExprKind::Cast { source, kind } => self.translate_cast(span, source, kind),

            ExprKind::Call { ref callee, ref args } => self.translate_call(span, callee, args),

            ExprKind::Adt(ref adt) => self.translate_adt(expr, adt),

            ExprKind::Closure { ref params, body } => self.translate_closure(span, params, body),

            ExprKind::FnItem { .. } => span_bug!(span, "FnItem in lowered ETIR"),
            ExprKind::Const { .. } => span_bug!(span, "Const in lowered ETIR"),

            ExprKind::Fun { ref fun, forced_extern } => {
                self.translate_fun(expr, fun, forced_extern)
            }

            ExprKind::Assign { lhs, rhs } => self.translate_assign(span, lhs, rhs),

            ExprKind::Declare { ref vars } => self.translate_declare(span, vars),

            ExprKind::Receive { ref arms, ref after } => {
                self.translate_receive(span, arms, after.as_ref())
            }

            ExprKind::TryCatch { body, ref arms, after } => {
                self.translate_try_catch(span, body, arms, after)
            }

            ExprKind::Match { scrutinee, ref arms } => self.translate_match(span, scrutinee, arms),

            ExprKind::Loop { id, body } => self.translate_loop(span, id, body, None),
            ExprKind::Join { join, body } => self.translate_join(span, join, body),
            ExprKind::Jump { target, value } => self.translate_jump(span, target, value),
        }
    }

    fn translate_list(
        &mut self,
        span: Span,
        items: &[ExprId],
        tail: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let tail = tail.map(|id| self.translate_expr(id));
        let items = items.iter().map(|&id| self.translate_expr(id));
        afb.expr_list(span, items, tail)
    }

    fn translate_map(
        &mut self,
        span: Span,
        fields: &[(ExprId, ExprId)],
        base: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let base_expr = base.map(|base_id| self.translate_expr(base_id));

        let assocs = fields.iter().map(|&(key, value)| {
            let key = self.translate_expr(key);
            let value = self.translate_expr(value);
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

    fn translate_tuple_field(
        &mut self,
        span: Span,
        lhs: ExprId,
        position: etir::FieldIdx,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let tuple = self.translate_expr(lhs);
        let index = afb.expr_int(span, (position.as_u32() + 1).into());
        afb.erlang_bif(span, sym::element, [index, tuple])
    }

    fn translate_cast(
        &mut self,
        span: Span,
        source: ExprId,
        kind: etir::CastKind,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let source = self.translate_expr(source);
        let function = match kind {
            etir::CastKind::Identity => return source,
            etir::CastKind::IntToFloat => sym::float,
            etir::CastKind::FloatToInt => sym::trunc,
        };
        afb.erlang_bif(span, function, [source])
    }

    fn translate_literal(&self, span: Span, lit: etir::Lit, lit_span: Span) -> eaf::Expr<'arena> {
        use etir::Lit;
        let afb = &self.translator.afb;
        match lit {
            Lit::Str(symbol) => afb.expr_binary_string(lit_span, symbol),
            Lit::Charlist(symbol) => afb.expr_string(lit_span, symbol),
            Lit::Atom(symbol) => afb.expr_atom(lit_span, symbol),
            Lit::ByteStr(symbol) => afb.expr_binary_bytes(lit_span, symbol),
            Lit::Char(char) => afb.expr_char(lit_span, char),
            Lit::Int { value, neg } => {
                let int = afb.expr_int(lit_span, value.get());
                if neg { afb.expr_unary_op(span, eaf::UnaryOp::Neg, int) } else { int }
            }
            Lit::Float(bits) => {
                let float = f64::from_bits(bits);
                let magnitude = afb.expr_float(lit_span, float.abs());
                if float.is_sign_negative() {
                    afb.expr_unary_op(span, eaf::UnaryOp::Neg, magnitude)
                } else {
                    magnitude
                }
            }
            Lit::Bool(true) => afb.expr_atom(lit_span, kw::True),
            Lit::Bool(false) => afb.expr_atom(lit_span, kw::False),
        }
    }

    fn translate_pin_var_ref(&self, span: Span, id: VarId) -> eaf::Pattern<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.pat_var(span, var.erl_name)
    }

    pub(super) fn translate_var_ref(&self, span: Span, id: VarId) -> eaf::Expr<'arena> {
        let var = self.var_for_read(span, id);
        self.translator.afb.expr_var(span, var.erl_name)
    }

    fn var_for_read(&self, span: Span, id: VarId) -> PatinaVar {
        *self.scopes.get_var(id).unwrap_or_else(|| span_bug!(span, "variable {id:?} not in scope"))
    }

    fn translate_block(&mut self, span: Span, exprs: &[ExprId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let exprs = exprs.iter().map(|&id| self.translate_expr(id));
        afb.expr_block(span, exprs)
    }

    fn translate_declare(&mut self, span: Span, vars: &[VarId]) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let declarations = vars.iter().map(|&var| {
            let var_span = self.etir[var].ident.span;
            let erl_name = self.scopes.bind_var(var, self.etir[var].ident.name).erl_name;
            afb.expr_match(var_span, afb.pat_var(var_span, erl_name), afb.expr_unit(var_span))
        });

        afb.expr_block(span, declarations)
    }

    fn translate_binary(
        &mut self,
        span: Span,
        op: etir::BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let left = self.translate_expr(lhs);
        let right = self.translate_expr(rhs);

        let operator = match op {
            etir::BinaryOp::Concat => eaf::BinaryOp::Concat,
            etir::BinaryOp::Subtract => eaf::BinaryOp::Subtract,
            etir::BinaryOp::Add => eaf::BinaryOp::Add,
            etir::BinaryOp::Sub => eaf::BinaryOp::Sub,
            etir::BinaryOp::Mul => eaf::BinaryOp::Mul,
            etir::BinaryOp::Div => eaf::BinaryOp::Div,
            etir::BinaryOp::DivInt => eaf::BinaryOp::DivInt,
            etir::BinaryOp::Rem => eaf::BinaryOp::Rem,
            etir::BinaryOp::Bxor => eaf::BinaryOp::Bxor,
            etir::BinaryOp::Band => eaf::BinaryOp::Band,
            etir::BinaryOp::Bor => eaf::BinaryOp::Bor,
            etir::BinaryOp::Bsl => eaf::BinaryOp::Bsl,
            etir::BinaryOp::Bsr => eaf::BinaryOp::Bsr,
            etir::BinaryOp::EqExact => eaf::BinaryOp::EqExact,
            etir::BinaryOp::NeExact => eaf::BinaryOp::NeExact,
            etir::BinaryOp::Lt => eaf::BinaryOp::Lt,
            etir::BinaryOp::Le => eaf::BinaryOp::Le,
            etir::BinaryOp::Ge => eaf::BinaryOp::Ge,
            etir::BinaryOp::Gt => eaf::BinaryOp::Gt,
        };

        self.translator.afb.expr_binary_op(span, operator, left, right)
    }

    fn translate_logical(
        &mut self,
        span: Span,
        op: etir::LogicalOp,
        lhs: ExprId,
        rhs: ExprId,
    ) -> eaf::Expr<'arena> {
        let left = self.translate_expr(lhs);
        let (right, right_scope) = self.with_scope(|cx| cx.translate_expr(rhs));

        if !self.scopes.modified_outer_vars([&right_scope]).is_empty() {
            span_bug!(span, "lowering didn't convert an rhs that assigns vars to a match");
        }

        let operator = match op {
            etir::LogicalOp::And => eaf::BinaryOp::Andalso,
            etir::LogicalOp::Or => eaf::BinaryOp::Orelse,
        };
        self.translator.afb.expr_binary_op(span, operator, left, right)
    }

    fn translate_unary(&mut self, span: Span, op: etir::UnaryOp, arg: ExprId) -> eaf::Expr<'arena> {
        let unary_op = match op {
            etir::UnaryOp::Not => eaf::UnaryOp::Not,
            etir::UnaryOp::Bnot => eaf::UnaryOp::Bnot,
            etir::UnaryOp::Neg => eaf::UnaryOp::Neg,
        };

        let operand = self.translate_expr(arg);
        self.translator.afb.expr_unary_op(span, unary_op, operand)
    }

    fn translate_join(&mut self, span: Span, join: JoinId, body: ExprId) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let body_span = self.etir[body].span;

        if let ExprKind::Loop { id, body: loop_body } = self.etir[body].kind {
            // Optimization: a join around a loop is necessarily its exit;
            // we can emit a better loop shape knowing that. See `translate_loop`.
            return self.translate_loop(body_span, id, loop_body, Some(join));
        }

        let (body, scope) = self.with_scope(|cx| cx.translate_expr(body));
        let body = TranslatedArm {
            patterns: afb.pat_var(body_span, kw::Underscore).into(),
            guard: None,
            branch: Branch { span: body_span, body, scope },
        };

        let mut translated = [self.translate_join_arm(join), body];
        self.merge_branches(translated.iter_mut().map(|arm| &mut arm.branch));
        afb.expr_case(span, afb.expr_nil(span), self.arm_clauses(translated))
    }

    /// `'$patina_marker':jump('$patina_label_N', Value, ..Params)`.
    fn translate_jump(
        &mut self,
        span: Span,
        target: JoinId,
        value: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let label = afb.expr_atom(span, label_name(target));
        let value = value.map(|value| self.translate_expr(value));
        let params = self.etir[target].params.iter().map(|&var| self.translate_var_ref(span, var));
        let args: SmallVec<[_; 4]> = std::iter::once(label).chain(value).chain(params).collect();

        let call = afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::dollar_patina_marker),
            afb.expr_atom(span, sym::jump),
            args,
        );
        // Wrap the marker call with `begin $call, [] end` so the call is never treated as a value
        // of an expression. We need that because otherwise, when we rewrite to jumps in core
        // Erlang, some jumps can end up in positions that are annoying to fix at that stage.
        afb.expr_block(span, [call, afb.expr_nil(span)])
    }

    fn translate_join_arm(&mut self, join: JoinId) -> TranslatedArm<'arena> {
        let body = self.etir[join].body;
        let span = self.etir[body].span;

        let ((pattern, body), scope) = self.with_scope(|cx| {
            let pattern = cx.bind_join_args(span, join);
            (pattern, cx.translate_expr(body))
        });

        TranslatedArm {
            patterns: smallvec![GuardedPattern(pattern, smallvec![])],
            guard: None,
            branch: Branch { span, body, scope },
        }
    }

    /// `{'$patina_label_N', Value, ..Params}` pattern.
    pub(super) fn bind_join_args(&mut self, span: Span, join: JoinId) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;
        let etir::Join { value, ref params, .. } = self.etir[join];
        let label = afb.pat_atom(span, label_name(join));
        let args = value.into_iter().chain(params.iter().copied()).map(|var| {
            let name = self.scopes.bind_var(var, self.etir[var].ident.name).erl_name;
            afb.pat_var(span, name)
        });
        afb.pat_tuple(span, std::iter::once(label).chain(args))
    }

    fn translate_call(
        &mut self,
        span: Span,
        callee: &Callee<'tcx>,
        args: &[ExprId],
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        match *callee {
            Callee::Value(fun) => {
                let callee_expr = self.translate_expr(fun);
                let arg_exprs = args.iter().map(|&id| self.translate_expr(id));
                afb.expr_local_call(span, callee_expr, arg_exprs)
            }

            Callee::Item { .. } => span_bug!(span, "item call in lowered ETIR"),

            Callee::Fun { ref fun, forced_extern } => {
                self.emit_fun_call(span, fun, args, forced_extern)
            }

            Callee::Direct(module, function) => {
                let arg_exprs = args.iter().map(|&id| self.translate_expr(id));
                afb.expr_remote_call(
                    span,
                    afb.expr_atom(span, module),
                    afb.expr_atom(span, function),
                    arg_exprs,
                )
            }
        }
    }

    fn emit_fun_call(
        &mut self,
        span: Span,
        fun: &ErlFun,
        args: &[ExprId],
        forced_extern: bool,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let callee_expr = afb.expr_atom(span, fun.name);

        match self.fun_call_kind(span, fun, forced_extern) {
            ErlCallKind::Local => {
                let arg_exprs = args.iter().map(|&id| self.translate_expr(id));
                afb.expr_local_call(span, callee_expr, arg_exprs)
            }
            ErlCallKind::Extern => {
                let mod_expr = afb.expr_atom(span, self.static_module(span, fun));
                let arg_exprs = args.iter().map(|&id| self.translate_expr(id));
                afb.expr_remote_call(span, mod_expr, callee_expr, arg_exprs)
            }
            ErlCallKind::Dynamic => {
                not_implemented!(
                    self.tcx(),
                    span,
                    "call of an item on a dynamic receiver",
                    fun.name
                )
            }
        }
    }

    fn fun_call_kind(&self, span: Span, fun: &ErlFun, forced_extern: bool) -> ErlCallKind {
        self.tcx()
            .erl_call_kind(self.owner, fun.module, forced_extern)
            .unwrap_or_else(|| span_bug!(span, "`{:?}` has no Erlang module", self.owner))
    }

    fn static_module(&self, span: Span, fun: &ErlFun) -> Symbol {
        let ErlModuleResolution::Static(module_id) = fun.module else {
            span_bug!(span, "dynamic target in static call path");
        };
        self.tcx().erl_module_name(module_id)
    }

    fn translate_closure(
        &mut self,
        span: Span,
        params: &[PatId],
        body: ExprId,
    ) -> eaf::Expr<'arena> {
        let translated = self.with_scope(|cx| cx.translate_body_with_params(params, body)).0;

        let afb = &self.translator.afb;
        let clause = afb.clause(span, translated.params, None, translated.value);
        afb.expr_fun(span, [clause])
    }

    fn translate_fun(
        &mut self,
        expr: &etir::Expr<'tcx>,
        fun: &ErlFun,
        forced_extern: bool,
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx();
        let span = expr.span;
        let afb = &self.translator.afb;

        match self.fun_call_kind(span, fun, forced_extern) {
            ErlCallKind::Local => afb.expr_local_fun(span, fun.name, fun.arity),
            ErlCallKind::Extern => {
                afb.expr_remote_fun(span, self.static_module(span, fun), fun.name, fun.arity)
            }
            ErlCallKind::Dynamic => {
                not_implemented!(tcx, span, "dynamic function-item value", expr.ty)
            }
        }
    }

    fn translate_assign(&mut self, span: Span, lhs: PatId, rhs: ExprId) -> eaf::Expr<'arena> {
        let rhs = self.translate_expr(rhs);
        let lhs = self.translate_single_pattern(lhs);
        self.translator.afb.expr_match(span, lhs, rhs)
    }

    pub(super) fn translate_assignee(&mut self, span: Span, var: VarId) -> eaf::Pattern<'arena> {
        let erl_name = self.scopes.bind_var(var, self.etir[var].ident.name).erl_name;
        self.translator.afb.pat_var(span, erl_name)
    }

    fn translate_match(
        &mut self,
        span: Span,
        scrutinee: ExprId,
        arms: &[etir::Arm],
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let scrutinee = self.translate_expr(scrutinee);

        let mut translated: Box<[TranslatedArm<'arena>]> =
            arms.iter().map(|arm| self.translate_arm(arm)).collect();

        self.merge_branches(translated.iter_mut().map(|arm| &mut arm.branch));
        afb.expr_case(span, scrutinee, self.arm_clauses(translated))
    }

    fn arm_clauses(
        &self,
        translated: impl IntoIterator<Item = TranslatedArm<'arena>>,
    ) -> impl Iterator<Item = eaf::Clause<'arena>> {
        let afb = &self.translator.afb;
        translated.into_iter().flat_map(move |arm| {
            let TranslatedArm { patterns, guard, branch: Branch { span, body, .. } } = arm;
            patterns.into_iter().map(move |GuardedPattern(pattern, pattern_guards)| {
                afb.clause(span, [pattern], pattern_guards.into_iter().chain(guard), body)
            })
        })
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
        span: Span,
        arms: &[etir::Arm],
        after: Option<&etir::ReceiveAfter>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let mut after = after.map(|after| {
            let span = self.etir[after.body].span;
            let (body, scope) = self.with_scope(|cx| cx.translate_expr(after.body));
            (self.translate_expr(after.timeout), Branch { span, body, scope })
        });

        let mut translated: SmallVec<[TranslatedArm<'arena>; 4]> =
            arms.iter().map(|arm| self.translate_arm(arm)).collect();
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
            span,
            clauses,
            after.map(|(timeout_expr, branch)| (timeout_expr, branch.body)),
        )
    }

    fn translate_try_catch(
        &mut self,
        span: Span,
        body: ExprId,
        arms: &[etir::CatchArm],
        after: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let (body, _) = self.with_scope(|cx| cx.translate_expr(body));
        let after = after.map(|after| self.with_scope(|cx| cx.translate_expr(after)).0);

        let catches = arms.iter().flat_map(|arm| {
            let etir::CatchArm { span, class, reason, stacktrace, guard, body } = *arm;

            let ((patterns, guard, body), _scope) = self.with_scope(|cx| {
                let patterns = cx.translate_exception_pattern(class, reason, stacktrace);
                let guard = guard.map(|guard| cx.translate_expr(guard));
                let body = cx.translate_expr(body);
                (patterns, guard, body)
            });

            patterns.into_iter().map(move |pattern| {
                afb.catch_clause(
                    span,
                    pattern.class,
                    pattern.reason,
                    pattern.stacktrace,
                    pattern.guards.into_iter().chain(guard),
                    body,
                )
            })
        });
        afb.expr_try(span, body, catches, after)
    }

    fn translate_exception_pattern(
        &mut self,
        class_pat: PatId,
        reason_pat: PatId,
        stacktrace: Option<VarId>,
    ) -> SmallVec<[CatchPattern<'arena>; 1]> {
        let stacktrace = match stacktrace {
            Some(var) => self.scopes.bind_var(var, self.etir[var].ident.name).erl_name,
            None => kw::Underscore,
        };

        let classes = self.translate_exception_kind_pattern(class_pat);
        let reasons = self.translate_pattern(reason_pat);
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
        id: PatId,
    ) -> SmallVec<[GuardedExceptionKind<'arena>; 1]> {
        let afb = &self.translator.afb;
        let pat = &self.etir[id];

        // The slot has no room for an alias, so keep the variable and demote the pattern it is
        // aliasing to a guard on it.
        if let PatKind::Binding { var, subpattern: Some(subpattern), is_primary, .. } = pat.kind {
            let class = self.bind_pattern_var(var, is_primary).erl_name;
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

        self.translate_pattern(id)
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

    fn translate_arm(&mut self, arm: &etir::Arm) -> TranslatedArm<'arena> {
        let etir::Arm { span, pattern, guard, body } = *arm;
        let ((patterns, guard, body), scope) = self.with_scope(|cx| {
            let patterns = cx.translate_pattern(pattern);
            let guard = guard.map(|guard| cx.translate_expr(guard));
            let body = cx.translate_expr(body);
            (patterns, guard, body)
        });
        TranslatedArm { patterns, guard, branch: Branch { span, body, scope } }
    }

    /// Makes sure the latest version of every modified outer variable is bound by the branch.
    fn bind_branch_outer_variables(
        &mut self,
        span: Span,
        body: eaf::Expr<'arena>,
        branch_scope: &Scope,
        changed_vars: &FxIndexMap<VarId, PatinaVar>,
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

    pub(super) fn translate_single_pattern(&mut self, pat: PatId) -> eaf::Pattern<'arena> {
        let patterns = self.translate_pattern(pat);
        if let [GuardedPattern(pattern, guards)] = patterns.as_slice()
            && guards.is_empty()
        {
            *pattern
        } else {
            span_bug!(
                self.etir[pat].span,
                "pattern expanded to multiple patterns or guards in a singleton context"
            )
        }
    }

    pub(super) fn translate_pattern(&mut self, id: PatId) -> Patterns<'arena> {
        let pat = &self.etir[id];

        match pat.kind {
            PatKind::Or { ref pats } => {
                pats.iter().flat_map(|&pattern| self.translate_pattern(pattern)).collect()
            }

            PatKind::Range(ref range) => self.translate_range_pattern(pat.span, range),

            PatKind::Wild { input } => self.translate_wild_pattern(pat.span, pat.ty, input),

            PatKind::Binding { var, is_primary, subpattern, input } => {
                self.translate_binding_pattern(pat, var, subpattern, is_primary, input)
            }

            PatKind::Pin { var } => self.translate_pin_var_ref(pat.span, var).into(),

            PatKind::Const { .. } => span_bug!(pat.span, "Const in lowered ETIR"),

            PatKind::Assignee { var } => self.translate_assignee(pat.span, var).into(),

            PatKind::TermClass { class, subpattern } => {
                self.translate_term_class_pattern(pat.span, class, subpattern)
            }

            PatKind::Tuple { ref fields } => self.translate_tuple_pattern(pat.span, fields),

            PatKind::Adt(ref adt) => self.translate_adt_pattern(pat, adt),

            PatKind::Literal(lit) => self.translate_literal_pattern(pat.span, lit).into(),

            PatKind::Map { ref fields } => self.translate_map_pattern(pat.span, fields),

            PatKind::BinaryPattern { ref segments } => {
                self.translate_pat_binary_pattern(pat.span, segments)
            }

            PatKind::List { ref items, tail } => self.translate_list_pattern(pat.span, items, tail),
        }
    }

    fn translate_tuple_pattern(&mut self, span: Span, fields: &[PatId]) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let groups: SmallVec<[_; 4]> =
            fields.iter().map(|&field| self.translate_pattern(field)).collect();

        self.pattern_cartesian_product(span, &groups, |elements| {
            afb.pat_tuple(span, elements.iter().copied())
        })
    }

    fn translate_map_pattern(
        &mut self,
        span: Span,
        fields: &[(ExprId, PatId)],
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let keys: SmallVec<[_; 4]> =
            fields.iter().map(|&(key, _)| self.translate_expr(key)).collect();
        let groups: SmallVec<[_; 4]> =
            fields.iter().map(|&(_, pattern)| self.translate_pattern(pattern)).collect();

        self.pattern_cartesian_product(span, &groups, |values| {
            afb.pat_map(span, keys.iter().copied().zip(values.iter().copied()))
        })
    }

    fn translate_list_pattern(
        &mut self,
        span: Span,
        items: &[PatId],
        tail: Option<PatId>,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let tail_patterns = tail.map(|pattern| self.translate_pattern(pattern));
        let groups: SmallVec<[_; 4]> = items
            .iter()
            .map(|&pattern| self.translate_pattern(pattern))
            .chain(tail_patterns)
            .collect();

        self.pattern_cartesian_product(span, &groups, |patterns| {
            let (items, tail) = if tail.is_some() {
                let (tail, items) = patterns.split_last().unwrap();
                (items, Some(*tail))
            } else {
                (patterns, None)
            };
            afb.pat_list(span, items.iter().copied(), tail)
        })
    }

    fn bind_pattern_var(&mut self, var: VarId, is_primary: bool) -> &PatinaVar {
        if is_primary {
            self.scopes.bind_var(var, self.etir[var].ident.name)
        } else {
            self.scopes.fetch_var(var)
        }
    }

    fn translate_literal_pattern(&self, span: Span, lit: Lit) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;
        match lit {
            Lit::Atom(atom) => afb.pat_atom(span, atom),
            Lit::Bool(true) => afb.pat_atom(span, kw::True),
            Lit::Bool(false) => afb.pat_atom(span, kw::False),
            Lit::Char(character) => afb.pat_char(span, character),
            Lit::Int { value, neg } => {
                let int = afb.pat_int(span, value.get());
                if neg { afb.pat_unary_op(span, eaf::UnaryOp::Neg, int) } else { int }
            }
            Lit::Float(bits) => afb.pat_float(span, f64::from_bits(bits)),
            Lit::Str(s) => afb.pat_binary_string(span, s),
            Lit::ByteStr(bytes) => afb.pat_binary_bytes(span, bytes.as_byte_str()),
            Lit::Charlist(symbol) => afb.pat_string(span, symbol.as_str()),
        }
    }

    fn translate_range_pattern(
        &mut self,
        span: Span,
        range: &etir::PatRange<'tcx>,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let range_var = self.scopes.fresh_var(sym::range);

        let lower = range.lo.map(|lo| {
            let lhs = afb.expr_var(span, range_var.erl_name);
            let rhs = self.translate_expr(lo);
            afb.expr_binary_op(span, eaf::BinaryOp::Ge, lhs, rhs)
        });
        let upper = range.hi.map(|hi| {
            let lhs = afb.expr_var(span, range_var.erl_name);
            let rhs = self.translate_expr(hi);
            let op = match range.end {
                rustc_hir::RangeEnd::Included => eaf::BinaryOp::Le,
                rustc_hir::RangeEnd::Excluded => eaf::BinaryOp::Lt,
            };
            afb.expr_binary_op(span, op, lhs, rhs)
        });
        let shape_guard = matches!(range.input, PatternInput::UntrustedRepresentation)
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

    fn translate_binding_pattern(
        &mut self,
        pat: &etir::Pat<'tcx>,
        var: VarId,
        subpattern: Option<PatId>,
        is_primary: bool,
        input: PatternInput,
    ) -> Patterns<'arena> {
        let erl_name = self.bind_pattern_var(var, is_primary).erl_name;
        let var_pat = self.translator.afb.pat_var(pat.span, erl_name);

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
        subpattern: PatId,
    ) -> Patterns<'arena> {
        self.translate_pattern(subpattern)
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
                let pattern = match self.etir[subpattern].kind {
                    PatKind::Wild { .. } => value_pat,
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
        afb.erlang_bif(span, name, [value])
    }

    fn translate_wild_pattern(
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

fn label_name(join: JoinId) -> Symbol {
    Symbol::intern(&format!("$patina_label_{}", join.as_usize()))
}
