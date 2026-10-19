use rustc_data_structures::fx::FxIndexSet;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_index::{IndexSlice, newtype_index};
use rustc_span::Span;

use crate::ty::{self, TyCtxt, TypeVisitableExt};

newtype_index! {
    /// A direct supertrait of a trait, in `erl_trait_supertraits` order.
    #[encodable]
    #[debug_format = "supertrait{}"]
    pub struct SupertraitIdx {}
}

newtype_index! {
    /// A bound of an item or of an impl, in `erl_bounds` order.
    #[encodable]
    #[debug_format = "bound{}"]
    pub struct BoundIdx {}
}

newtype_index! {
    /// A declared bound of an associated type, in `erl_assoc_type_bounds` order.
    #[encodable]
    #[debug_format = "assoc_bound{}"]
    pub struct AssocTypeBoundIdx {}
}

impl<'tcx> TyCtxt<'tcx> {
    /// The bounds in `clauses` that need a runtime witness, in order and without duplicates.
    fn erl_witnessed_trait_refs(
        self,
        clauses: impl IntoIterator<Item = &'tcx (ty::Clause<'tcx>, Span)>,
    ) -> FxIndexSet<ty::TraitRef<'tcx>> {
        let mut trait_refs = FxIndexSet::default();
        for &(clause, _) in clauses {
            let Some(trait_pred) = clause.as_trait_clause() else { continue };
            let trait_pred = self.instantiate_bound_regions_with_erased(trait_pred);
            // A bound without type parameters, like `where S: Derived`, needs no runtime
            // witness: code that uses it selects the impl itself.
            if trait_pred.polarity == ty::PredicatePolarity::Positive
                && trait_pred.trait_ref.has_non_region_param()
            {
                self.insert_witnessed_trait_ref(trait_pred.trait_ref, &mut trait_refs);
            }
        }
        trait_refs
    }

    /// A bound on a trait without a witness stands for the bounds on its supertraits.
    fn insert_witnessed_trait_ref(
        self,
        trait_ref: ty::TraitRef<'tcx>,
        trait_refs: &mut FxIndexSet<ty::TraitRef<'tcx>>,
    ) {
        if self.erl_trait_needs_witness(trait_ref.def_id) {
            trait_refs.insert(trait_ref);
            return;
        }

        for &(clause, _) in self.explicit_implied_predicates_of(trait_ref.def_id).skip_binder() {
            if let Some(supertrait) = clause.as_trait_clause() {
                let supertrait = self.instantiate_bound_regions_with_erased(supertrait);
                let supertrait = ty::EarlyBinder::bind(supertrait.trait_ref)
                    .instantiate(self, trait_ref.args)
                    .skip_normalization();
                self.insert_witnessed_trait_ref(supertrait, trait_refs);
            }
        }
    }

    pub fn erl_needs_witnesses(self, def_id: DefId) -> bool {
        self.erl_entry_trait(def_id).is_some() || !self.erl_bounds(def_id).skip_binder().is_empty()
    }
}

/// Provider for [`TyCtxt::erl_trait_supertraits`].
pub(super) fn erl_trait_supertraits<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
) -> ty::EarlyBinder<'tcx, &'tcx IndexSlice<SupertraitIdx, ty::TraitRef<'tcx>>> {
    // The implied predicates also hold the bounds on the associated types of `Self`, like
    // `<Self as HasItem>::Item: Label` in `trait Labelled: HasItem<Item: Label>`.
    let clauses = tcx.explicit_implied_predicates_of(trait_def_id).skip_binder();
    let supertraits = tcx.erl_witnessed_trait_refs(clauses).into_iter().filter(|trait_ref| {
        let mut ty = trait_ref.self_ty();
        while let ty::Alias(alias) = ty.kind()
            && let ty::AliasTyKind::Projection { .. } = alias.kind
        {
            ty = alias.self_ty();
        }
        ty == tcx.types.self_param
    });
    ty::EarlyBinder::bind(IndexSlice::from_raw(tcx.arena.alloc_from_iter(supertraits)))
}

/// Provider for [`TyCtxt::erl_assoc_type_bounds`].
pub(super) fn erl_assoc_type_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    assoc_def_id: DefId,
) -> ty::EarlyBinder<'tcx, &'tcx IndexSlice<AssocTypeBoundIdx, ty::TraitRef<'tcx>>> {
    let clauses = tcx.explicit_item_bounds(assoc_def_id).skip_binder();
    let bounds = tcx.erl_witnessed_trait_refs(clauses).into_iter().filter(|trait_ref| {
        let mut ty = trait_ref.self_ty();
        while let ty::Alias(alias) = ty.kind()
            && let ty::AliasTyKind::Projection { def_id } = alias.kind
        {
            if def_id == assoc_def_id {
                return true;
            }
            ty = alias.self_ty();
        }
        false
    });
    ty::EarlyBinder::bind(IndexSlice::from_raw(tcx.arena.alloc_from_iter(bounds)))
}

/// Provider for [`TyCtxt::erl_bounds`].
pub(super) fn erl_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
) -> ty::EarlyBinder<'tcx, &'tcx IndexSlice<BoundIdx, ty::TraitRef<'tcx>>> {
    if let Some(trait_def_id) = tcx.trait_of_assoc(def_id) {
        let self_trait_ref = ty::TraitRef::identity(tcx, trait_def_id);
        // Both are in the trait's generics.
        let supertraits = tcx.erl_trait_supertraits(trait_def_id).skip_binder();
        let clauses = tcx
            .predicates_of(trait_def_id)
            .predicates
            .iter()
            .chain(tcx.predicates_of(def_id).predicates);
        let bounds = tcx.erl_witnessed_trait_refs(clauses).into_iter().filter(|trait_ref| {
            *trait_ref != self_trait_ref && !supertraits.raw.contains(trait_ref)
        });
        return ty::EarlyBinder::bind(IndexSlice::from_raw(tcx.arena.alloc_from_iter(bounds)));
    }

    if let Some(trait_item_def_id) = tcx.trait_item_of(def_id) {
        let args = tcx.erl_trait_item_args_for_impl_item(def_id);
        let bounds = tcx
            .erl_bounds(trait_item_def_id)
            .iter_instantiated_copied(tcx, args)
            .map(|bound| bound.skip_normalization());
        return ty::EarlyBinder::bind(IndexSlice::from_raw(tcx.arena.alloc_from_iter(bounds)));
    }

    let predicates = tcx.predicates_of(def_id);
    let parent_clauses = predicates
        .parent
        .filter(|&parent| matches!(tcx.def_kind(parent), DefKind::Impl { .. }))
        .into_iter()
        .flat_map(|parent| tcx.predicates_of(parent).predicates);
    let bounds = tcx.erl_witnessed_trait_refs(parent_clauses.chain(predicates.predicates));
    ty::EarlyBinder::bind(IndexSlice::from_raw(tcx.arena.alloc_from_iter(bounds)))
}

/// Provider for [`TyCtxt::erl_trait_needs_witness`].
pub(super) fn erl_trait_needs_witness(tcx: TyCtxt<'_>, trait_def_id: DefId) -> bool {
    // A closure is an Erlang fun, so a call through `Fn*` needs no witness.
    !tcx.is_fn_trait(trait_def_id)
        && tcx.associated_items(trait_def_id).in_definition_order().next().is_some()
}
