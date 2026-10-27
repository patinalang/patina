use std::ops::Deref;

use rustc_data_structures::smallvec::SmallVec;
use rustc_middle::erl::etir::{ExprId, JoinId, LoopId, VarId};
use rustc_span::{Span, kw, sym};

use crate::compile::translate::BodyCx;
use crate::erlang::abstract_format as eaf;

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_loop(
        &mut self,
        span: Span,
        id: LoopId,
        body: ExprId,
        exit: Option<JoinId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let reassigned = &self.etir[id].reassigned;

        let (loop_body, _) = self.with_scope(|cx| cx.translate_loop_body(span, id, body, exit));

        let loop_fun = afb.expr_named_fun(
            span,
            sym::_dollar_patina_loop,
            [afb.clause(span, self.var_pats(span, reassigned), None, loop_body)],
        );
        let call_loop = afb.expr_local_call(span, loop_fun, self.var_exprs(span, reassigned));

        match exit {
            Some(exit) => self.loop_exit_join(span, id, exit, call_loop),
            None => call_loop,
        }
    }

    fn translate_loop_body(
        &mut self,
        span: Span,
        id: LoopId,
        body: ExprId,
        exit: Option<JoinId>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let reassigned = &self.etir[id].reassigned;

        let body = self.translate_expr(body);

        // The body is not a tail expression, so Erlang would warn that its value is unused.
        let body = afb.expr_match(span, afb.pat_var(span, kw::Underscore), body);
        let self_fun = afb.expr_var(span, sym::_dollar_patina_loop);
        let recurse = afb.expr_local_call(span, self_fun, self.var_exprs(span, reassigned));
        let body = afb.expr_block(span, [body, recurse]);

        match exit {
            // Optimization: if we know of an exit from this loop (as is usual), we can hoist it inside
            // the loop, and avoid dispatch cost of a cross-frame jump in patina_compile_server.
            Some(exit) => self.loop_exit_jump(span, id, exit, body),
            None => body,
        }
    }

    fn var_pats(&self, span: Span, ids: &[VarId]) -> impl Iterator<Item = eaf::Pattern<'arena>> {
        let afb = &self.translator.afb;
        ids.iter().map(move |&id| {
            let var = self.scopes.fetch_var(id);
            afb.pat_var(span, var.erl_name)
        })
    }

    fn var_exprs(&self, span: Span, ids: &[VarId]) -> impl Iterator<Item = eaf::Expr<'arena>> {
        let afb = &self.translator.afb;
        ids.iter().map(move |&id| {
            let var = self.scopes.fetch_var(id);
            afb.expr_var(span, var.erl_name)
        })
    }

    fn loop_exit_join(
        &mut self,
        span: Span,
        id: LoopId,
        exit: JoinId,
        call_loop: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let reassigned = &self.etir[id].reassigned;

        let state: SmallVec<[eaf::Pattern; 1]> = self.etir[exit]
            .value
            .into_iter()
            .chain(reassigned.iter().copied())
            .map(|var| self.translate_assignee(span, var))
            .collect();

        let pattern = match state.deref() {
            &[item] => item,
            _ => afb.pat_tuple(span, state),
        };

        let match_loop_result = afb.expr_match(span, pattern, call_loop);
        let result = self.translate_expr(self.etir[exit].body);

        afb.expr_block(span, [match_loop_result, result])
    }

    fn loop_exit_jump(
        &mut self,
        span: Span,
        id: LoopId,
        exit: JoinId,
        loop_body: eaf::Expr<'arena>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let reassigned = &self.etir[id].reassigned;

        let (exit_clause, _) = self.with_scope(|cx| {
            let pattern = cx.bind_join_args(span, exit);
            let state: SmallVec<[eaf::Expr<'arena>; 1]> = cx.etir[exit]
                .value
                .into_iter()
                .chain(reassigned.iter().copied())
                .map(|var| cx.translate_var_ref(span, var))
                .collect();

            let body = match state.as_slice() {
                &[item] => item,
                _ => afb.expr_tuple(span, state),
            };

            afb.clause(span, [pattern], None, body)
        });

        let body_clause = afb.clause(span, [afb.pat_var(span, kw::Underscore)], None, loop_body);
        afb.expr_case(span, afb.expr_nil(span), [exit_clause, body_clause])
    }
}
