use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::{Impl, ItemKind};
use rustc_middle::erl::ErlModuleId;
use rustc_middle::ty::TyCtxt;

use crate::{not_implemented, span_fatal};

pub(crate) struct ImplCollector<'tcx> {
    impls: FxHashMap<ErlModuleId, Vec<(DefId, &'tcx Impl<'tcx>)>>,
}

impl<'tcx> ImplCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> ImplCollector<'tcx> {
        let mut impls: FxHashMap<ErlModuleId, Vec<(DefId, &'tcx Impl<'tcx>)>> =
            FxHashMap::default();

        for item_id in tcx.hir_free_items() {
            if !matches!(tcx.def_kind(item_id.owner_id), DefKind::Impl { .. }) {
                continue;
            }

            let def_id = item_id.owner_id.def_id;
            let item = tcx.hir_item(item_id);

            let impl_item = match &item.kind {
                ItemKind::Impl(impl_item) => impl_item,
                _ => unreachable!(),
            };

            // An impl of a trait that needs no witness has no functions.
            // Skip it to avoid creating a module for it later down the line.
            if tcx
                .impl_opt_trait_id(def_id)
                .is_some_and(|trait_def_id| !tcx.erl_trait_needs_witness(trait_def_id))
            {
                continue;
            }

            // Likewise an inherent impl whose only methods are intrinsics.
            if tcx.impl_opt_trait_id(def_id).is_none()
                && tcx
                    .associated_items(def_id)
                    .in_definition_order()
                    .all(|item| !item.is_fn() || tcx.intrinsic(item.def_id).is_some())
            {
                continue;
            }

            match tcx.erl_definition_module(def_id) {
                Some(module_id) => {
                    impls.entry(module_id).or_default().push((def_id.to_def_id(), impl_item));
                }
                None => {
                    not_implemented!(
                        tcx,
                        impl_item.self_ty.span,
                        "inherent impl on a type without a module"
                    );
                }
            }
        }

        ImplCollector { impls }
    }

    pub fn impls_for(&self, module: ErlModuleId) -> Option<&Vec<(DefId, &'tcx Impl<'tcx>)>> {
        self.impls.get(&module)
    }

    /// Modules with no local item behind them.
    pub fn impl_only_modules(&self) -> impl Iterator<Item = &ErlModuleId> + '_ {
        self.impls.keys().filter(|&module| {
            matches!(
                module,
                ErlModuleId::Prim(_)
                    | ErlModuleId::AdtImplHost(..)
                    | ErlModuleId::PrimImplHost(..)
                    | ErlModuleId::BlanketImplHost(_)
            )
        })
    }
}
