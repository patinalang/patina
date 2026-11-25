use std::ops::Range;

use rustc_middle::bug;
use rustc_middle::erl::etir::visit::{MutVisitor, walk_body_mut};
use rustc_middle::erl::etir::{Body, ExprId, JoinId, LoopId, PatId, VarId};
use rustc_middle::ty::{self, GenericArgsRef, Ty, TyCtxt};
use rustc_span::Span;

use crate::BodyBuildCx;

pub(crate) struct Graft {
    pub(crate) value: ExprId,
    /// All exprs added by this graft on top of the original body.
    pub(crate) exprs: Range<ExprId>,
    /// All pats added by this graft on top of the original body.
    pub(crate) pats: Range<PatId>,
}

impl<'tcx> BodyBuildCx<'tcx> {
    /// Appends a copy of `source` applied to `call_args` (with `param = call_arg` exprs).
    /// Instantiates source's generic parameters with `args`, and normalizes the resulting types
    /// in the owner's environment. Replaces all spans of the copy with `caller_span`.
    pub(crate) fn graft(
        &mut self,
        mut source: Body<'tcx>,
        args: GenericArgsRef<'tcx>,
        call_args: &[ExprId],
        caller_span: Span,
    ) -> Graft {
        Rebaser::rebase(&mut source, self, args, caller_span);

        let Body { params, value, exprs, pats, vars, loops, joins } = source;
        if params.len() != call_args.len() {
            bug!("grafted body takes {} args, called with {}", params.len(), call_args.len());
        }

        let exprs_start = self.body.exprs.next_index();
        let pats_start = self.body.pats.next_index();

        self.body.exprs.extend(exprs);
        self.body.pats.extend(pats);
        self.body.vars.extend(vars);
        self.body.loops.extend(loops);
        self.body.joins.extend(joins);

        let value = with_expr_block!(self, |block| {
            for (param, &arg) in std::iter::zip(params, call_args) {
                block.assign(self, param, arg);
            }
            value
        });

        Graft {
            value,
            exprs: exprs_start..self.body.exprs.next_index(),
            pats: pats_start..self.body.pats.next_index(),
        }
    }
}

/// Offsets the ids of copied nodes, past the max offset already in the target body.
/// Instantiates source's generic parameters in the target's environment.
/// Replaces all spans with `span`.
struct Rebaser<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    typing_env: ty::TypingEnv<'tcx>,
    target: &'a Body<'tcx>,
    args: GenericArgsRef<'tcx>,
    span: Span,
}

impl<'a, 'tcx> Rebaser<'a, 'tcx> {
    fn rebase(
        source: &mut Body<'tcx>,
        cx: &'a BodyBuildCx<'tcx>,
        args: GenericArgsRef<'tcx>,
        span: Span,
    ) {
        let tcx = cx.tcx;
        let typing_env = ty::TypingEnv::post_analysis(tcx, tcx.typeck_root_def_id(cx.owner));
        walk_body_mut(&mut Self { tcx, typing_env, target: &cx.body, args, span }, source);
    }
}

impl<'a, 'tcx> Rebaser<'a, 'tcx> {
    fn instantiate<T: ty::TypeFoldable<TyCtxt<'tcx>> + Copy>(&self, value: T) -> T {
        if self.args.is_empty() {
            return value;
        }
        let value = ty::EarlyBinder::bind(value).instantiate(self.tcx, self.args);
        self.tcx.normalize_erasing_regions_or_unchanged(self.typing_env, value)
    }
}

impl<'a, 'etir, 'tcx: 'etir> MutVisitor<'etir, 'tcx> for Rebaser<'a, 'tcx> {
    fn visit_expr_id(&mut self, id: &mut ExprId) {
        *id = ExprId::from_usize(id.index() + self.target.exprs.len());
    }

    fn visit_pat_id(&mut self, id: &mut PatId) {
        *id = PatId::from_usize(id.index() + self.target.pats.len());
    }

    fn visit_var_id(&mut self, id: &mut VarId) {
        *id = VarId::from_usize(id.index() + self.target.vars.len());
    }

    fn visit_loop_id(&mut self, id: &mut LoopId) {
        *id = LoopId::from_usize(id.index() + self.target.loops.len());
    }

    fn visit_join_id(&mut self, id: &mut JoinId) {
        *id = JoinId::from_usize(id.index() + self.target.joins.len());
    }

    fn visit_ty(&mut self, ty: &mut Ty<'tcx>) {
        *ty = self.instantiate(*ty);
    }

    fn visit_args(&mut self, args: &mut GenericArgsRef<'tcx>) {
        *args = self.instantiate(*args);
    }

    fn visit_span(&mut self, span: &mut Span) {
        *span = self.span;
    }
}
