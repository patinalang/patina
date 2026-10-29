use rustc_middle::bug;
use rustc_middle::erl::etir::visit::{MutVisitor, walk_body_mut};
use rustc_middle::erl::etir::{Body, ExprId, JoinId, LoopId, PatId, VarId};
use rustc_middle::ty::{self, GenericArgsRef, Ty, TyCtxt};

use crate::BodyBuildCx;

impl<'tcx> BodyBuildCx<'tcx> {
    /// Appends a copy of `source` the current body, and returns the copy of its value.
    pub(crate) fn graft(&mut self, mut source: Body<'tcx>, args: GenericArgsRef<'tcx>) -> ExprId {
        Rebaser::rebase(&mut source, self, args);

        let Body { params, value, exprs, pats, vars, loops, joins } = source;
        if !params.is_empty() {
            bug!("grafted body has parameters");
        }

        self.body.exprs.extend(exprs);
        self.body.pats.extend(pats);
        self.body.vars.extend(vars);
        self.body.loops.extend(loops);
        self.body.joins.extend(joins);

        value
    }
}

/// Offsets the ids of copied nodes, past the max offset already in the target body.
/// Instantiates source's generic parameters in the target's environment.
struct Rebaser<'a, 'tcx> {
    cx: &'a BodyBuildCx<'tcx>,
    args: GenericArgsRef<'tcx>,
}

impl<'a, 'tcx> Rebaser<'a, 'tcx> {
    fn rebase(source: &mut Body<'tcx>, cx: &'a BodyBuildCx<'tcx>, args: GenericArgsRef<'tcx>) {
        walk_body_mut(&mut Self { cx, args }, source);
    }
}

impl<'a, 'tcx> Rebaser<'a, 'tcx> {
    fn instantiate<T: ty::TypeFoldable<TyCtxt<'tcx>> + Copy>(&self, value: T) -> T {
        if self.args.is_empty() {
            return value;
        }
        let BodyBuildCx { tcx, typing_env, .. } = *self.cx;
        let value = ty::EarlyBinder::bind(value).instantiate(tcx, self.args);
        tcx.normalize_erasing_regions_or_unchanged(typing_env, value)
    }
}

impl<'a, 'etir, 'tcx: 'etir> MutVisitor<'etir, 'tcx> for Rebaser<'a, 'tcx> {
    fn visit_expr_id(&mut self, id: &mut ExprId) {
        *id = ExprId::from_usize(id.index() + self.cx.body.exprs.len());
    }

    fn visit_pat_id(&mut self, id: &mut PatId) {
        *id = PatId::from_usize(id.index() + self.cx.body.pats.len());
    }

    fn visit_var_id(&mut self, id: &mut VarId) {
        *id = VarId::from_usize(id.index() + self.cx.body.vars.len());
    }

    fn visit_loop_id(&mut self, id: &mut LoopId) {
        *id = LoopId::from_usize(id.index() + self.cx.body.loops.len());
    }

    fn visit_join_id(&mut self, id: &mut JoinId) {
        *id = JoinId::from_usize(id.index() + self.cx.body.joins.len());
    }

    fn visit_ty(&mut self, ty: &mut Ty<'tcx>) {
        *ty = self.instantiate(*ty);
    }

    fn visit_args(&mut self, args: &mut GenericArgsRef<'tcx>) {
        *args = self.instantiate(*args);
    }
}
