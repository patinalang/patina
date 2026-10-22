use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_middle::erl::etir::visit::{self, Visitor};
use rustc_middle::erl::etir::{
    Body, CatchArm, Expr, ExprId, ExprKind, JoinId, LoopId, LoopOuterVar, Pat, PatId, PatKind,
    VarId,
};

/// Analyzes which parameters have to be passed in to loop helpers / joins, and which vars have
/// to be returned out of loop helpers.
/// Fills in `etir::Loop.outer_vars` and `etir::Join.params`.
pub(super) fn analyze(body: &mut Body<'_>) {
    let mut captures = Captures {
        body,
        used: FxIndexMap::default(),
        bound: FxHashSet::default(),
        outer_vars: Vec::new(),
        join_params: Vec::new(),
    };
    visit::visit_expr_id(&mut captures, body.value);
    let Captures { outer_vars, join_params, .. } = captures;
    for (id, vars) in outer_vars {
        body.loops[id].outer_vars = vars;
    }
    for (id, vars) in join_params {
        body.joins[id].params = vars;
    }
}

struct Captures<'etir, 'tcx> {
    body: &'etir Body<'tcx>,
    /// Variables from outer scopes that are used inside the current body.
    used: FxIndexMap<VarId, LoopOuterVar>,
    /// Variables that are bound inside the current scope.
    bound: FxHashSet<VarId>,
    outer_vars: Vec<(LoopId, Box<[LoopOuterVar]>)>,
    join_params: Vec<(JoinId, Box<[VarId]>)>,
}

impl<'etir, 'tcx> Captures<'etir, 'tcx> {
    fn use_var(&mut self, var: LoopOuterVar) {
        if self.bound.contains(&var.var()) {
            return;
        }

        let used = self.used.entry(var.var()).or_insert(var);
        if let LoopOuterVar::Reassigned(_) = var {
            *used = var;
        }
    }

    fn with_scope(
        &mut self,
        f: impl FnOnce(&mut Self),
    ) -> (FxIndexMap<VarId, LoopOuterVar>, FxHashSet<VarId>) {
        let used = std::mem::take(&mut self.used);
        let bound = std::mem::take(&mut self.bound);
        f(self);
        let inner_bound = std::mem::replace(&mut self.bound, bound);
        let captured = std::mem::replace(&mut self.used, used);
        for &var in captured.values() {
            self.use_var(var);
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
            ExprKind::Var(var) => self.use_var(LoopOuterVar::Referenced(*var)),
            ExprKind::Declare { vars } => {
                self.bound.extend(vars.iter().copied());
            }
            ExprKind::Loop { id, body: _ } => {
                let captured = self.with_scope(|s| visit::walk_expr(s, expr)).0;
                self.outer_vars.push((*id, captured.into_values().collect()));
            }
            ExprKind::Join { joins, body } => {
                let (used_in_body, bound_in_body) =
                    self.with_scope(|s| visit::visit_expr_id(s, *body));

                let reassigned_in_body: Vec<VarId> = used_in_body
                    .into_values()
                    .filter_map(|var| match var {
                        LoopOuterVar::Reassigned(var) => Some(var),
                        LoopOuterVar::Referenced(_) => None,
                    })
                    .collect();

                for &join in joins {
                    let used_in_join =
                        self.with_scope(|s| visit::visit_expr_id(s, s.body[join].body)).0;
                    let read_body_bindings =
                        used_in_join.into_keys().filter(|var| bound_in_body.contains(var));
                    let params =
                        reassigned_in_body.iter().copied().chain(read_body_bindings).collect();

                    self.join_params.push((join, params));
                }
            }
            _ => visit::walk_expr(self, expr),
        }
    }

    fn visit_pat(&mut self, _id: PatId, pat: &'etir Pat<'tcx>) {
        match pat.kind {
            PatKind::Pin { var } => self.use_var(LoopOuterVar::Referenced(var)),
            PatKind::Assignee { var } => self.use_var(LoopOuterVar::Reassigned(var)),
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
