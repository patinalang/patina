use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_hir::{Impl, ItemKind};
use rustc_middle::erl::ErlModuleId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

use crate::package::CORE_PACKAGE_NAME;
use crate::{not_implemented, span_fatal};

pub(crate) struct ImplCollector<'tcx> {
    impls: FxHashMap<ErlModuleId, Vec<(DefId, &'tcx Impl<'tcx>)>>,
}

impl<'tcx> ImplCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> ImplCollector<'tcx> {
        let mut impls: FxHashMap<ErlModuleId, Vec<(DefId, &'tcx Impl<'tcx>)>> =
            FxHashMap::default();

        let bootstrap = tcx.crate_name(LOCAL_CRATE) == Symbol::intern(CORE_PACKAGE_NAME);

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

            match tcx.erl_definition_module(def_id) {
                Some(ErlModuleId::Prim(prim_ty)) => {
                    if !bootstrap {
                        not_implemented!(tcx, impl_item.self_ty.span, "impl for a primitive type");
                    }
                    impls
                        .entry(ErlModuleId::Prim(prim_ty))
                        .or_default()
                        .push((def_id.to_def_id(), impl_item));
                }
                Some(module_id) => {
                    impls.entry(module_id).or_default().push((def_id.to_def_id(), impl_item));
                }
                None => {
                    not_implemented!(tcx, impl_item.self_ty.span, "impl with a non-ADT self type");
                }
            }
        }

        ImplCollector { impls }
    }

    pub fn impls_for(&self, module: ErlModuleId) -> Option<&Vec<(DefId, &'tcx Impl<'tcx>)>> {
        self.impls.get(&module)
    }

    pub fn prim_modules(&self) -> impl Iterator<Item = &ErlModuleId> + '_ {
        self.impls.keys().filter(|&module| matches!(module, ErlModuleId::Prim(_)))
    }
}
