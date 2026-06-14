use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::{Impl, ItemKind};
use rustc_middle::ty::TyCtxt;

pub(crate) struct ImplCollector<'tcx> {
    impls: FxHashMap<DefId, Vec<&'tcx Impl<'tcx>>>,
}

impl<'tcx> ImplCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> ImplCollector<'tcx> {
        let mut impls: FxHashMap<DefId, Vec<&'tcx Impl<'tcx>>> = FxHashMap::default();

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

            let self_ty = tcx.type_of(def_id).instantiate_identity().skip_norm_wip();
            let Some(adt_def_id) = self_ty.ty_adt_def().map(|adt| adt.did()) else {
                continue;
            };

            impls.entry(adt_def_id).or_default().push(impl_item);
        }

        ImplCollector { impls }
    }

    pub fn impls_for(&self, def_id: DefId) -> Option<&Vec<&'tcx Impl<'tcx>>> {
        self.impls.get(&def_id)
    }
}
