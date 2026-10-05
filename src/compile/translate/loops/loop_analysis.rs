use rustc_data_structures::fx::FxIndexSet;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{self, Expr, ExprId, ExprKind, LocalVarId, Pat, PatKind};

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

    fn visit_pat(&mut self, pat: &'thir Pat<'tcx>) {
        match pat.kind {
            PatKind::Pin { var } if self.scopes.get_var(var).is_some() => {
                self.referenced.insert(var);
            }
            PatKind::Assignee { var } if self.scopes.get_var(var).is_some() => {
                self.referenced.insert(var);
                self.rebound.insert(var);
            }
            _ => {}
        }
        visit::walk_pat(self, pat);
    }
}
