use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_index::IndexVec;
use rustc_middle::erl::etir::visit::{self, Visitor};
use rustc_middle::erl::etir::{
    Body, CatchArm, Expr, ExprId, ExprKind, JoinId, LoopId, Pat, PatId, PatKind, VarId,
};

/// Analyzes which parameters have to be passed in to loop helpers / joins, and which vars have
/// to be returned out of loop helpers.
/// Fills in `etir::Loop.reassigned` and `etir::Join.params`.
pub(super) fn analyze(body: &mut Body<'_>) {
    let mut captures = Captures {
        body,
        used: FxIndexMap::default(),
        bound: FxHashSet::default(),
        loop_reassigned: IndexVec::from_elem_n(Box::default(), body.loops.len()),
        join_params: IndexVec::from_elem_n(Box::default(), body.joins.len()),
    };
    visit::visit_expr_id(&mut captures, body.value);
    let Captures { loop_reassigned, join_params, .. } = captures;
    for (loop_, reassigned) in body.loops.iter_mut().zip(loop_reassigned) {
        loop_.reassigned = reassigned;
    }
    for (join, params) in body.joins.iter_mut().zip(join_params) {
        join.params = params;
    }
}

struct Captures<'etir, 'tcx> {
    body: &'etir Body<'tcx>,
    /// Variables from outer scopes that are used inside the current body.
    /// `true` if they're reassigned.
    used: FxIndexMap<VarId, bool>,
    /// Variables that are bound inside the current scope.
    bound: FxHashSet<VarId>,
    loop_reassigned: IndexVec<LoopId, Box<[VarId]>>,
    join_params: IndexVec<JoinId, Box<[VarId]>>,
}

impl<'etir, 'tcx> Captures<'etir, 'tcx> {
    fn use_var(&mut self, var: VarId, reassigned: bool) {
        if self.bound.contains(&var) {
            return;
        }

        *self.used.entry(var).or_default() |= reassigned;
    }

    fn with_scope(
        &mut self,
        f: impl FnOnce(&mut Self),
    ) -> (FxIndexMap<VarId, bool>, FxHashSet<VarId>) {
        let used = std::mem::take(&mut self.used);
        let bound = std::mem::take(&mut self.bound);
        f(self);
        let inner_bound = std::mem::replace(&mut self.bound, bound);
        let captured = std::mem::replace(&mut self.used, used);
        for (&var, &reassigned) in &captured {
            self.use_var(var, reassigned);
        }
        self.bound.extend(inner_bound.iter().copied());
        (captured, inner_bound)
    }
}

impl<'etir, 'tcx: 'etir> Visitor<'etir, 'tcx> for Captures<'etir, 'tcx> {
    fn etir(&self) -> &'etir Body<'tcx> {
        self.body
    }

    fn visit_expr(&mut self, _id: ExprId, expr: &'etir Expr<'tcx>) {
        match &expr.kind {
            ExprKind::Var(var) => self.use_var(*var, false),
            ExprKind::Declare { vars } => {
                self.bound.extend(vars.iter().copied());
            }
            ExprKind::Loop { id, body: _ } => {
                let captured = self.with_scope(|s| visit::walk_expr(s, expr)).0;
                self.loop_reassigned[*id] = reassigned_vars(captured).collect();
            }
            ExprKind::Join { joins, body } => {
                let (used_in_body, bound_in_body) =
                    self.with_scope(|s| visit::visit_expr_id(s, *body));

                let reassigned_in_body: Vec<VarId> = reassigned_vars(used_in_body).collect();

                for &join in joins {
                    let used_in_join =
                        self.with_scope(|s| visit::visit_expr_id(s, s.body[join].body)).0;
                    let read_body_bindings =
                        used_in_join.into_keys().filter(|var| bound_in_body.contains(var));
                    let params =
                        reassigned_in_body.iter().copied().chain(read_body_bindings).collect();

                    self.join_params[join] = params;
                }
            }
            _ => visit::walk_expr(self, expr),
        }
    }

    fn visit_pat(&mut self, _id: PatId, pat: &'etir Pat<'tcx>) {
        match pat.kind {
            PatKind::Pin { var } => self.use_var(var, false),
            PatKind::Assignee { var } => self.use_var(var, true),
            PatKind::Binding { var, .. } => {
                self.bound.insert(var);
            }
            _ => {}
        }
        visit::walk_pat(self, pat);
    }

    fn visit_catch_arm(&mut self, arm: &'etir CatchArm) {
        if let Some(var) = arm.stacktrace {
            self.bound.insert(var);
        }
        visit::walk_catch_arm(self, arm);
    }
}

fn reassigned_vars(used: FxIndexMap<VarId, bool>) -> impl Iterator<Item = VarId> {
    used.into_iter().filter_map(|(var, reassigned)| reassigned.then_some(var))
}
