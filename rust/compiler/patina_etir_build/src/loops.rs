use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_middle::erl::etir::visit::{self, Visitor};
use rustc_middle::erl::etir::{
    Body, CatchArm, Expr, ExprId, ExprKind, LoopId, LoopOuterVar, Pat, PatId, PatKind, VarId,
};

/// Analyzes which parameters have to be passed in to loop helpers, and which vars have
/// to be returned out.
/// Fills in `etir::Loop.outer_vars`.
pub(super) fn analyze(body: &mut Body<'_>) {
    let mut captures = Captures {
        body,
        used: FxIndexMap::default(),
        bound: FxHashSet::default(),
        outer_vars: Vec::new(),
    };
    visit::visit_expr_id(&mut captures, body.value);
    for (id, vars) in captures.outer_vars {
        body.loops[id].outer_vars = vars;
    }
}

struct Captures<'etir, 'tcx> {
    body: &'etir Body<'tcx>,
    /// Variables from outer scopes that are used inside the current loop body.
    used: FxIndexMap<VarId, LoopOuterVar>,
    /// Variables that are bound inside the current scope.
    bound: FxHashSet<VarId>,
    outer_vars: Vec<(LoopId, Box<[LoopOuterVar]>)>,
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
                let used = std::mem::take(&mut self.used);
                let bound = std::mem::take(&mut self.bound);
                visit::walk_expr(self, expr);
                self.bound = bound;
                let captured = std::mem::replace(&mut self.used, used);

                for &var in captured.values() {
                    self.use_var(var);
                }
                self.outer_vars.push((*id, captured.into_values().collect()));
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
