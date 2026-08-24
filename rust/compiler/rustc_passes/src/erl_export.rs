//! Computes which associated functions are called from outside the Erlang module they are
//! translated into.
//! See [`TyCtxt::assoc_fns_called_externally`].

use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefIdSet};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, ExprKind};
use rustc_middle::bug;
use rustc_middle::erl::{ErlCallTarget, ErlModuleResolution};
use rustc_middle::query::Providers;
use rustc_middle::ty::{self, GenericArgsRef, TyCtxt, TypeckResults};

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

        if self.tcx.def_kind(def_id) != DefKind::AssocFn {
            return;
        }

        let Some(caller_module) = self.tcx.erl_definition_module(self.owner) else {
            return;
        };

        let is_trait_method = self.tcx.def_kind(self.tcx.parent(def_id)) == DefKind::Trait;

        let Some(ErlModuleResolution::Static(callee_module)) =
            self.tcx.erl_module_for_target(self.owner, target)
        else {
            // Dispatch through a type parameter: the call site names a trait rather than an impl,
            // so this method must be exported from every local impl of that trait.
            if is_trait_method {
                self.record_trait_impls(def_id);
            }
            return;
        };

        if caller_module == callee_module {
            return;
        }

        let callee = if is_trait_method {
            // The call site names the trait item, but the body that runs is an impl item.
            // Resolve to that impl item and record it, rather than the whole trait.
            let typing_env = ty::TypingEnv::non_body_analysis(self.tcx, self.owner);
            match ty::Instance::try_resolve(self.tcx, typing_env, def_id, args) {
                Ok(Some(instance)) => instance.def_id(),
                _ => return,
            }
        } else {
            def_id
        };

        if let Some(local) = callee.as_local() {
            self.called_externally.insert(local);
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
        let trait_item_def_id = match self.tcx.def_kind(self.tcx.parent(def_id)) {
            DefKind::Trait => def_id,
            _ => match self.tcx.associated_item(def_id).trait_item_def_id() {
                Some(trait_item_def_id) => trait_item_def_id,
                None => return,
            },
        };

        for impl_def_id in self.tcx.local_trait_impls(trait_def_id) {
            let Some(&impl_item_def_id) =
                self.tcx.impl_item_implementor_ids(impl_def_id.to_def_id()).get(&trait_item_def_id)
            else {
                bug!(
                    "impl {} does not implement {}",
                    self.tcx.def_path_str(impl_def_id.to_def_id()),
                    self.tcx.def_path_str(trait_item_def_id)
                );
            };

            if let Some(local) = impl_item_def_id.as_local() {
                self.called_externally.insert(local);
            }
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
