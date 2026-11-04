//! # Minimal Specialization
//!
//! This module contains the checks for specialization used when the
//! `min_specialization` feature is enabled.

use rustc_hir::def_id::LocalDefId;
use rustc_infer::traits::specialization_graph::Node;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::trait_def::TraitSpecializationKind;
use rustc_span::ErrorGuaranteed;

use crate::errors;

pub(super) fn check_min_specialization(
    tcx: TyCtxt<'_>,
    impl_def_id: LocalDefId,
) -> Result<(), ErrorGuaranteed> {
    if let Some(node) = parent_specialization_node(tcx, impl_def_id) {
        check_has_items(tcx, impl_def_id, node)?;
    }
    Ok(())
}

fn parent_specialization_node(tcx: TyCtxt<'_>, impl1_def_id: LocalDefId) -> Option<Node> {
    let trait_ref = tcx.impl_trait_ref(impl1_def_id);
    let trait_def = tcx.trait_def(trait_ref.skip_binder().def_id);

    let impl2_node = trait_def.ancestors(tcx, impl1_def_id.to_def_id()).ok()?.nth(1)?;

    let always_applicable_trait =
        matches!(trait_def.specialization_kind, TraitSpecializationKind::AlwaysApplicable);
    if impl2_node.is_from_trait() && !always_applicable_trait {
        // Implementing a normal trait isn't a specialization.
        return None;
    }
    if trait_def.is_marker {
        // Overlapping marker implementations are not really specializations.
        return None;
    }
    Some(impl2_node)
}

fn check_has_items(
    tcx: TyCtxt<'_>,
    impl1_def_id: LocalDefId,
    impl2_node: Node,
) -> Result<(), ErrorGuaranteed> {
    if let Node::Impl(impl2_id) = impl2_node
        && tcx.associated_item_def_ids(impl1_def_id).is_empty()
    {
        let span = tcx.def_span(impl1_def_id);
        let base_impl_span = tcx.def_span(impl2_id);
        return Err(tcx.dcx().emit_err(errors::EmptySpecialization { span, base_impl_span }));
    }
    Ok(())
}
