//! Computes which associated functions are called from outside the Erlang module they are
//! translated into.
//! See [`TyCtxt::assoc_fns_called_externally`].

use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefIdSet};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, ExprKind};
use rustc_middle::erl::{ErlCallTarget, ErlModuleResolution};
use rustc_middle::query::Providers;
use rustc_middle::ty::{GenericArgsRef, TyCtxt, TypeckResults};

struct ExternalCallVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    typeck: &'tcx TypeckResults<'tcx>,
    owner: DefId,
    called_externally: &'a mut LocalDefIdSet,
}

impl<'tcx> ExternalCallVisitor<'_, 'tcx> {
    fn record_callee(&mut self, def_id: DefId, args: GenericArgsRef<'tcx>) {
        let target @ ErlCallTarget::Item { def_id, args: _ } =
            self.tcx.erl_call_target_for_def(def_id, args)
        else {
            return;
        };

        let Some(local) = def_id.as_local() else {
            return;
        };

        if self.tcx.def_kind(def_id) != DefKind::AssocFn {
            return;
        }

        let Some(caller_module) = self.tcx.erl_definition_module(self.owner) else {
            return;
        };
        let Some(ErlModuleResolution::Static(callee_module)) =
            self.tcx.erl_module_for_target(self.owner, target)
        else {
            return;
        };

        if caller_module != callee_module {
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
                    self.record_callee(def_id, self.typeck.node_args(expr.hir_id));
                }
            }
            ExprKind::MethodCall(..) => {
                if let Some(def_id) = self.typeck.type_dependent_def_id(expr.hir_id) {
                    self.record_callee(def_id, self.typeck.node_args(expr.hir_id));
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
            owner: owner.to_def_id(),
            called_externally: &mut called_externally,
        };
        visitor.visit_body(tcx.hir_body_owned_by(owner));
    }

    called_externally
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { assoc_fns_called_externally, ..*providers };
}
