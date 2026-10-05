mod loop_analysis;

use rustc_data_structures::fx::FxIndexSet;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::HirId;
use rustc_middle::erl::erl_fit_atom;
use rustc_middle::middle::region;
use rustc_middle::thir::{Expr, ExprId, LocalVarId};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::translate::BodyCx;
use crate::compile::translate::loops::loop_analysis::LoopVariables;
use crate::compile::translate::scopes::PatinaVar;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

pub(super) struct Loops {
    loops: SmallVec<[LoopContext; 4]>,
}

impl Loops {
    pub(super) fn new() -> Self {
        Self { loops: SmallVec::new() }
    }

    pub(super) fn current(&self) -> &LoopContext {
        self.loops.last().unwrap_or_else(|| bug!("loop stack is empty"))
    }

    pub(super) fn in_loop(&self) -> bool {
        !self.loops.is_empty()
    }

    pub(super) fn depth(&self) -> usize {
        self.loops.len()
    }

    pub(super) fn record_break(&mut self, label: region::Scope) -> Option<usize> {
        let target = self.loops.iter().position(|loop_cx| loop_cx.label == label)?;
        if let [handler, through @ ..] = &mut self.loops[target..] {
            handler.control.breaks_here = true;
            for loop_cx in through {
                loop_cx.control.breaks_through = true;
            }
        }
        Some(target)
    }

    pub(super) fn record_continue(&mut self, label: region::Scope) -> Option<usize> {
        let target = self.loops.iter().position(|loop_cx| loop_cx.label == label)?;
        if let [_target, handler, through @ ..] = &mut self.loops[target..] {
            handler.control.continues_outer = true;
            for loop_cx in through {
                loop_cx.control.continues_through = true;
            }
        }
        Some(target)
    }

    pub(super) fn record_return(&mut self) {
        for loop_cx in &mut self.loops {
            loop_cx.control.returns = true;
        }
    }

    fn push(&mut self, loop_cx: LoopContext) {
        self.loops.push(loop_cx);
    }

    fn pop(&mut self) -> LoopContext {
        self.loops.pop().unwrap_or_else(|| bug!("loop stack is empty"))
    }
}

#[derive(Default)]
pub(super) struct LoopControl {
    pub breaks_here: bool,
    pub breaks_through: bool,
    pub continues_outer: bool,
    pub continues_through: bool,
    pub returns: bool,
}

pub(super) struct LoopContext {
    pub helper_erl_name: Symbol,
    pub depth: usize,
    label: region::Scope,
    /// Variables that are passed as parameters to the loop helper.
    pub params: FxIndexSet<LocalVarId>,
    /// Variables that are rebound in the loop body; a subset of `params`.
    pub rebound: FxIndexSet<LocalVarId>,
    pub control: LoopControl,
}

impl LoopContext {
    fn new(
        helper_erl_name: Symbol,
        depth: usize,
        label: region::Scope,
        params: FxIndexSet<LocalVarId>,
        rebound: FxIndexSet<LocalVarId>,
    ) -> Self {
        Self { helper_erl_name, depth, label, params, rebound, control: LoopControl::default() }
    }
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn with_loop<R>(
        &mut self,
        expr_id: ExprId,
        label: region::Scope,
        f: impl FnOnce(&mut Self) -> R,
    ) -> (R, LoopContext) {
        let LoopVariables { params, rebound } =
            loop_analysis::analyze(self.thir, expr_id, self.scopes);

        let helper_erl_name = self.loop_helper_erl_name();
        let loop_cx = LoopContext::new(helper_erl_name, self.loops.depth(), label, params, rebound);

        self.loops.push(loop_cx);
        let result = f(self);
        let loop_cx = self.loops.pop();
        (result, loop_cx)
    }

    fn loop_helper_erl_name(&mut self) -> Symbol {
        let tcx = self.tcx();
        let root = tcx.typeck_root_def_id_local(self.owner);
        let num = self.scopes.next_loop_num();
        let name = if let Some(impl_def_id) = tcx.trait_impl_of_assoc(root.to_def_id()) {
            let tag = tcx.erl_impl_tag(impl_def_id.expect_local());
            format!("<{tag}>::{}$patina_loop_{num}", tcx.item_name(root.to_def_id()))
        } else {
            format!("{}$patina_loop_{num}", tcx.erl_fn_name(root))
        };
        Symbol::intern(&erl_fit_atom(name))
    }

    pub(super) fn translate_loop(
        &mut self,
        expr_id: ExprId,
        expr: &Expr<'tcx>,
        body: ExprId,
        region_scope: Option<(region::Scope, HirId)>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let Some((label, _)) = region_scope else {
            span_bug!(expr.span, "expected HIR-derived loop expression to have an outer Scope")
        };

        let ((loop_body, _body_scope), loop_cx) = self.with_loop(expr_id, label, |cx| {
            cx.with_scope(|cx| {
                let translated_body = cx.translate_expr(body);

                let loop_cx = cx.loops.current();
                let recurse_args = cx.loop_helper_param_exprs(expr.span, loop_cx);
                let recurse = cx.marker_call(expr.span, kw::Continue, recurse_args);

                // The body does not land as a tail expression, so Erlang warns
                // the result is unused, because it doesn't know we rewrite assembly later.
                // Assign to `_` to silence the warnings.
                let discard_body = afb.expr_match(
                    expr.span,
                    afb.pat_var(expr.span, kw::Underscore),
                    translated_body,
                );
                afb.expr_block(expr.span, [discard_body, recurse])
            })
        });

        let helper_function = afb.simple_function(
            expr.span,
            loop_cx.helper_erl_name,
            self.loop_helper_param_pats(expr.span, &loop_cx),
            loop_body,
        );

        self.translator.push_function(self.parts, expr.span, helper_function);
        let arity = self.witness_env.vars.len() + loop_cx.params.len();
        self.parts.exports.push((loop_cx.helper_erl_name, arity as u32));

        let loop_helper_call = afb.expr_local_call(
            expr.span,
            afb.expr_atom(expr.span, loop_cx.helper_erl_name),
            self.loop_helper_param_exprs(expr.span, &loop_cx),
        );

        for id in &loop_cx.rebound {
            self.scopes.rebind_var(*id);
        }

        let break_clause = loop_cx.control.breaks_here.then(|| {
            let result_var = self.scopes.fresh_var(sym::loop_value);
            self.break_clause(expr.span, &result_var, &loop_cx)
        });

        let propagate_break_clause = loop_cx.control.breaks_through.then(|| {
            let outer_loop = self.loops.current();
            let target_depth_var = self.scopes.fresh_var(sym::target_loop_depth);
            let result_var = self.scopes.fresh_var(sym::loop_value);
            self.propagate_break_clause(
                expr.span,
                outer_loop,
                &target_depth_var,
                &result_var,
                &loop_cx,
            )
        });

        let continue_clause = loop_cx
            .control
            .continues_outer
            .then(|| self.outer_continue_clause(expr.span, self.loops.current(), &loop_cx));

        let propagate_continue_clause = loop_cx.control.continues_through.then(|| {
            let outer_loop = self.loops.current();
            let target_depth_var = self.scopes.fresh_var(sym::target_loop_depth);
            self.propagate_continue_clause(expr.span, outer_loop, &target_depth_var, &loop_cx)
        });

        let return_clause = loop_cx.control.returns.then(|| {
            let result_var = self.scopes.fresh_var(sym::loop_value);
            self.propagate_return_clause(
                expr.span,
                self.loops.in_loop().then(|| self.loops.current()),
                &result_var,
                &loop_cx,
            )
        });

        let clauses = [
            break_clause,
            propagate_break_clause,
            continue_clause,
            propagate_continue_clause,
            return_clause,
        ];

        if clauses.iter().all(Option::is_none) {
            loop_helper_call
        } else {
            afb.expr_case(expr.span, loop_helper_call, clauses.into_iter().flatten())
        }
    }

    pub(super) fn translate_break(
        &mut self,
        span: Span,
        label: region::Scope,
        value: Option<ExprId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let value =
            value.map(|value| self.translate_expr(value)).unwrap_or_else(|| afb.expr_unit(span));

        let target_depth = self
            .loops
            .record_break(label)
            .unwrap_or_else(|| not_implemented!(self.tcx(), span, "break target outside of loop"));

        let target_depth = afb.expr_int(span, target_depth as u128);
        let loop_cx = self.loops.current();

        self.loop_break_expr(span, target_depth, value, loop_cx)
    }

    pub(super) fn translate_continue(
        &mut self,
        span: Span,
        label: region::Scope,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let target_depth = self.loops.record_continue(label).unwrap_or_else(|| {
            not_implemented!(self.tcx(), span, "continue target outside of loop")
        });

        let loop_cx = self.loops.current();

        if target_depth == loop_cx.depth {
            let args = self.loop_helper_param_exprs(span, loop_cx);
            self.marker_call(span, kw::Continue, args)
        } else {
            let target_depth = afb.expr_int(span, target_depth as u128);
            self.loop_continue_expr(span, target_depth, loop_cx)
        }
    }

    pub(super) fn loop_return_expr(
        &self,
        span: Span,
        value: eaf::Expr<'arena>,
        loop_cx: &LoopContext,
    ) -> eaf::Expr<'arena> {
        let marker = kw::Return;
        let tuple = self.loop_flow_control_tuple(span, marker, [value], loop_cx);
        self.marker_call(span, marker, [tuple])
    }

    fn loop_break_expr(
        &self,
        span: Span,
        target_depth: eaf::Expr<'arena>,
        value: eaf::Expr<'arena>,
        loop_cx: &LoopContext,
    ) -> eaf::Expr<'arena> {
        let marker = kw::Break;
        let tuple = self.loop_flow_control_tuple(span, marker, [target_depth, value], loop_cx);
        self.marker_call(span, marker, [tuple])
    }

    fn loop_continue_expr(
        &self,
        span: Span,
        target_depth: eaf::Expr<'arena>,
        loop_cx: &LoopContext,
    ) -> eaf::Expr<'arena> {
        let tuple = self.loop_flow_control_tuple(span, kw::Continue, [target_depth], loop_cx);
        self.marker_call(span, kw::Break, [tuple])
    }

    fn loop_flow_control_tuple(
        &self,
        span: Span,
        marker: Symbol,
        additional_args: impl IntoIterator<Item = eaf::Expr<'arena>>,
        loop_cx: &LoopContext,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let marker = afb.expr_atom(span, marker);
        let vars = self.var_exprs(span, loop_cx.rebound.iter().copied());
        afb.expr_tuple(span, std::iter::once(marker).chain(additional_args).chain(vars))
    }

    fn outer_continue_clause(
        &self,
        span: Span,
        outer_loop: &LoopContext,
        loop_cx: &LoopContext,
    ) -> eaf::Clause<'arena> {
        let afb = &self.translator.afb;
        let continue_pattern = self.loop_flow_control_pattern(
            span,
            kw::Continue,
            [afb.pat_int(span, outer_loop.depth as u128)],
            loop_cx,
        );
        let args = self.loop_helper_param_exprs(span, outer_loop);
        let body = self.marker_call(span, kw::Continue, args);
        afb.clause(span, [continue_pattern], None, body)
    }

    fn break_clause(
        &self,
        span: Span,
        result_var: &PatinaVar,
        loop_cx: &LoopContext,
    ) -> eaf::Clause<'arena> {
        let afb = &self.translator.afb;
        let break_pattern = self.loop_flow_control_pattern(
            span,
            kw::Break,
            [afb.pat_int(span, loop_cx.depth as u128), afb.pat_var(span, result_var.erl_name)],
            loop_cx,
        );
        afb.clause(span, [break_pattern], None, afb.expr_var(span, result_var.erl_name))
    }

    fn propagate_break_clause(
        &self,
        span: Span,
        outer_loop: &LoopContext,
        target_depth_var: &PatinaVar,
        result_var: &PatinaVar,
        loop_cx: &LoopContext,
    ) -> eaf::Clause<'arena> {
        let afb = &self.translator.afb;

        let break_pattern = self.loop_flow_control_pattern(
            span,
            kw::Break,
            [afb.pat_var(span, target_depth_var.erl_name), afb.pat_var(span, result_var.erl_name)],
            loop_cx,
        );
        let body = self.loop_break_expr(
            span,
            afb.expr_var(span, target_depth_var.erl_name),
            afb.expr_var(span, result_var.erl_name),
            outer_loop,
        );
        afb.clause(span, [break_pattern], None, body)
    }

    fn propagate_continue_clause(
        &self,
        span: Span,
        outer_loop: &LoopContext,
        target_depth_var: &PatinaVar,
        loop_cx: &LoopContext,
    ) -> eaf::Clause<'arena> {
        let afb = &self.translator.afb;
        let propagate_pattern = self.loop_flow_control_pattern(
            span,
            kw::Continue,
            [afb.pat_var(span, target_depth_var.erl_name)],
            loop_cx,
        );
        let body = self.loop_continue_expr(
            span,
            afb.expr_var(span, target_depth_var.erl_name),
            outer_loop,
        );
        afb.clause(span, [propagate_pattern], None, body)
    }

    fn propagate_return_clause(
        &self,
        span: Span,
        outer_loop: Option<&LoopContext>,
        result_var: &PatinaVar,
        loop_cx: &LoopContext,
    ) -> eaf::Clause<'arena> {
        let afb = &self.translator.afb;
        let propagate_pattern = self.loop_flow_control_pattern(
            span,
            kw::Return,
            [afb.pat_var(span, result_var.erl_name)],
            loop_cx,
        );
        let body = if let Some(outer_loop) = outer_loop {
            self.loop_return_expr(span, afb.expr_var(span, result_var.erl_name), outer_loop)
        } else {
            self.marker_call(span, kw::Return, [afb.expr_var(span, result_var.erl_name)])
        };
        afb.clause(span, [propagate_pattern], None, body)
    }

    fn loop_flow_control_pattern(
        &self,
        span: Span,
        marker: Symbol,
        additional_pats: impl IntoIterator<Item = eaf::Pattern<'arena>>,
        loop_cx: &LoopContext,
    ) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;
        let marker = afb.pat_atom(span, marker);
        let vars = self.var_pats(span, loop_cx.rebound.iter().copied());
        afb.pat_tuple(span, std::iter::once(marker).chain(additional_pats).chain(vars))
    }

    /// var_pats with witness variables first
    fn loop_helper_param_pats(
        &self,
        span: Span,
        loop_cx: &LoopContext,
    ) -> impl Iterator<Item = eaf::Pattern<'arena>> {
        let afb = &self.translator.afb;
        let witnesses = self.witness_env.vars.iter().map(move |&var| afb.pat_var(span, var));
        witnesses.chain(self.var_pats(span, loop_cx.params.iter().copied()))
    }

    /// var_exprs with witness variables first
    fn loop_helper_param_exprs(
        &self,
        span: Span,
        loop_cx: &LoopContext,
    ) -> impl Iterator<Item = eaf::Expr<'arena>> {
        let afb = &self.translator.afb;
        let witnesses = self.witness_env.vars.iter().map(move |&var| afb.expr_var(span, var));
        witnesses.chain(self.var_exprs(span, loop_cx.params.iter().copied()))
    }

    fn var_pats(
        &self,
        span: Span,
        ids: impl IntoIterator<Item = LocalVarId>,
    ) -> impl Iterator<Item = eaf::Pattern<'arena>> {
        let afb = &self.translator.afb;
        ids.into_iter().map(move |id| {
            let var = self.scopes.fetch_var(id);
            afb.pat_var(span, var.erl_name)
        })
    }

    fn var_exprs(
        &self,
        span: Span,
        ids: impl IntoIterator<Item = LocalVarId>,
    ) -> impl Iterator<Item = eaf::Expr<'arena>> {
        let afb = &self.translator.afb;
        ids.into_iter().map(move |id| {
            let var = self.scopes.fetch_var(id);
            afb.expr_var(span, var.erl_name)
        })
    }
}
