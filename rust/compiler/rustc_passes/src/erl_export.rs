//! Computes which associated functions are called from outside the Erlang module they are
//! translated into.
//! See [`TyCtxt::assoc_fns_called_externally`].

use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefIdSet};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, ExprKind};
use rustc_middle::query::Providers;
use rustc_middle::ty::{TyCtxt, TypeckResults};

struct ExternalCallVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    typeck: &'tcx TypeckResults<'tcx>,
    caller_home: DefId,
    called_externally: &'a mut LocalDefIdSet,
}

impl<'tcx> ExternalCallVisitor<'_, 'tcx> {
    fn record_callee(&mut self, callee: DefId) {
        let Some(local) = callee.as_local() else { return };
        if self.tcx.def_kind(callee) != DefKind::AssocFn {
            return;
        }

        // The call is external unless the caller lives in the very same generated module.
        if self.caller_home != self.tcx.erl_home_module(callee) {
            self.called_externally.insert(local);
        }
    }
}

impl<'tcx> Visitor<'tcx> for ExternalCallVisitor<'_, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Path(ref qpath) => {
                if let Res::Def(DefKind::AssocFn, def_id) =
                    self.typeck.qpath_res(qpath, expr.hir_id)
                {
                    self.record_callee(def_id);
                }
            }
            ExprKind::MethodCall(..) => {
                if let Some(def_id) = self.typeck.type_dependent_def_id(expr.hir_id) {
                    self.record_callee(def_id);
                }
            }
            _ => {}
        }

        intravisit::walk_expr(self, expr);
    }
}

fn assoc_fns_called_externally(tcx: TyCtxt<'_>, (): ()) -> LocalDefIdSet {
    let mut called_externally = LocalDefIdSet::default();

    for owner in tcx.hir_body_owners() {
        let mut visitor = ExternalCallVisitor {
            tcx,
            typeck: tcx.typeck(owner),
            caller_home: tcx.erl_home_module(owner),
            called_externally: &mut called_externally,
        };
        visitor.visit_body(tcx.hir_body_owned_by(owner));
    }

    called_externally
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { assoc_fns_called_externally, ..*providers };
}
