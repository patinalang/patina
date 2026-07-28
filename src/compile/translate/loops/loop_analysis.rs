use rustc_data_structures::fx::FxIndexSet;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{self, Expr, ExprId, ExprKind, LocalVarId};

use crate::compile::translate::scopes::Scopes;

pub(super) struct LoopVariables {
    pub params: FxIndexSet<LocalVarId>,
    pub rebound: FxIndexSet<LocalVarId>,
}

pub(super) fn analyze<'thir, 'tcx: 'thir>(
    thir: &'thir thir::Thir<'tcx>,
    body: ExprId,
    scopes: &Scopes,
) -> LoopVariables {
    let mut analysis = LoopVarAnalysis {
        thir,
        scopes,
        referenced: FxIndexSet::default(),
        rebound: FxIndexSet::default(),
    };
    analysis.visit_expr(&thir[body]);

    LoopVariables { params: analysis.referenced, rebound: analysis.rebound }
}

struct LoopVarAnalysis<'thir, 'tcx, 'scopes> {
    thir: &'thir thir::Thir<'tcx>,
    scopes: &'scopes Scopes,
    referenced: FxIndexSet<LocalVarId>,
    rebound: FxIndexSet<LocalVarId>,
}

impl LoopVarAnalysis<'_, '_, '_> {
    fn record_lhs(&mut self, mut expr_id: ExprId) {
        loop {
            match &self.thir[expr_id].kind {
                ExprKind::VarRef { id } => {
                    if self.scopes.get_var(*id).is_some() {
                        self.referenced.insert(*id);
                        self.rebound.insert(*id);
                    }
                    return;
                }
                ExprKind::Scope { value, .. } => expr_id = *value,
                ExprKind::Use { source }
                | ExprKind::ByUse { expr: source, .. }
                | ExprKind::NeverToAny { source }
                | ExprKind::Cast { source }
                | ExprKind::PointerCoercion { source, .. }
                | ExprKind::PlaceTypeAscription { source, .. }
                | ExprKind::ValueTypeAscription { source, .. } => expr_id = *source,
                _ => return,
            }
        }
    }
}

impl<'thir, 'tcx: 'thir> Visitor<'thir, 'tcx> for LoopVarAnalysis<'thir, 'tcx, '_> {
    fn thir(&self) -> &'thir thir::Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'thir Expr<'tcx>) {
        match &expr.kind {
            ExprKind::VarRef { id } => {
                if self.scopes.get_var(*id).is_some() {
                    self.referenced.insert(*id);
                }
            }
            ExprKind::UpvarRef { var_hir_id, .. } => {
                if self.scopes.get_var(*var_hir_id).is_some() {
                    self.referenced.insert(*var_hir_id);
                }
            }
            ExprKind::Assign { lhs, .. } | ExprKind::AssignOp { lhs, .. } => {
                self.record_lhs(*lhs);
                visit::walk_expr(self, expr);
            }
            ExprKind::Closure(closure) => {
                // The closure body has its own THIR, but its capture expressions are
                // evaluated in (and therefore must be parameters of) this loop helper.
                for upvar in closure.upvars.iter() {
                    self.visit_expr(&self.thir[*upvar]);
                }
            }
            _ => visit::walk_expr(self, expr),
        }
    }
}
