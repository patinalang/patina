//! Computes which functions are called from outside the Erlang module they are translated into.
//! See [`TyCtxt::fns_called_externally`].

use rustc_data_structures::unord::UnordSet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, ExprKind};
use rustc_middle::bug;
use rustc_middle::erl::{ErlCallTarget, ErlModuleId, ErlModuleResolution};
use rustc_middle::query::Providers;
use rustc_middle::ty::{GenericArgsRef, TyCtxt, TypeckResults};

struct ExternalCallVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    typeck: &'tcx TypeckResults<'tcx>,
    owner: DefId,
    called_externally: &'a mut UnordSet<(ErlModuleId, DefId)>,
}

impl<'tcx> ExternalCallVisitor<'_, 'tcx> {
    fn record_callee(&mut self, def_id: DefId, args: GenericArgsRef<'tcx>) {
        let target @ ErlCallTarget::Item { def_id, args: _ } =
            self.tcx.erl_call_target_for_def(self.owner, def_id, args)
        else {
            return;
        };

        if !matches!(
            self.tcx.def_kind(def_id),
            DefKind::Fn | DefKind::AssocFn | DefKind::AssocConst { .. }
        ) {
            return;
        }

        let Some(caller_module) = self.tcx.erl_definition_module(self.owner) else {
            return;
        };

        let is_trait_item = self.tcx.def_kind(self.tcx.parent(def_id)) == DefKind::Trait;

        let Some(ErlModuleResolution::Static(callee_module)) =
            self.tcx.erl_module_for_target(self.owner, target)
        else {
            // Dispatch through a witness: the call site names a trait rather than an impl, so
            // this method must be exported from every local impl of that trait.
            if is_trait_item {
                self.record_trait_impls(def_id);
            }
            return;
        };

        // An inlined constant's calls land in whichever module uses its value.
        if caller_module != callee_module || self.tcx.is_in_erl_inlined_const(self.owner) {
            self.called_externally.insert((callee_module, self.tcx.erl_fn_item(def_id)));
        }
    }

    /// Marks each local impl's version of the trait method `def_id` as externally called.
    fn record_trait_impls(&mut self, def_id: DefId) {
        let Some(trait_def_id) = self.tcx.erl_entry_trait(def_id) else {
            bug!(
                "expected a trait method or trait impl item, got {}",
                self.tcx.def_path_str(def_id)
            );
        };

        // The call site names a trait item, but a blanket impl item can reach here too.
        let trait_item_def_id = self.tcx.erl_fn_item(def_id);

        for &impl_def_id in self.tcx.local_trait_impls(trait_def_id) {
            if let Some(module) = self.tcx.erl_definition_module(impl_def_id) {
                self.called_externally.insert((module, trait_item_def_id));
            }
        }
    }
}

impl<'tcx> Visitor<'tcx> for ExternalCallVisitor<'_, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Path(ref qpath) => {
                if let Res::Def(
                    DefKind::Fn | DefKind::AssocFn | DefKind::AssocConst { .. },
                    def_id,
                ) = self.typeck.qpath_res(qpath, expr.hir_id)
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

fn fns_called_externally(tcx: TyCtxt<'_>, (): ()) -> UnordSet<(ErlModuleId, DefId)> {
    let mut called_externally = UnordSet::default();

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
    *providers = Providers { fns_called_externally, ..*providers };
}
