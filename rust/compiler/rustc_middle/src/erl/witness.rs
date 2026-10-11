use std::collections::hash_map::Entry;

use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hir::def_id::DefId;
use rustc_index::{IndexVec, newtype_index};
use rustc_macros::{TyDecodable, TyEncodable};
use rustc_span::Span;
use smallvec::SmallVec;

use crate::ty::{self, GenericArgsRef, TyCtxt, Unnormalized};

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

/// For a given bound `T: Trait`, describe where its witness (how we know the bound holds)
/// comes from in a given item, in a way that can be translated to runtime code.
#[derive(Clone, Debug, TyEncodable, TyDecodable)]
pub enum WitnessSource<'tcx> {
    /// The witness for the bound is passed in by this item's user.
    Param(ParamBound),
    /// The bound `T: Trait` is proven by its subtrait's bound `trait U: Trait`,
    /// so the witness can be found nested within `U`'s witness.
    Supertrait(Box<WitnessSource<'tcx>>, SupertraitIdx),
    /// At this point in code, we know exactly which impl proves the bound,
    /// so we can build our own witness.
    Impl {
        impl_def_id: DefId,
        args: GenericArgsRef<'tcx>,
        supertraits: IndexVec<SupertraitIdx, WitnessSource<'tcx>>,
        where_clauses: IndexVec<BoundIdx, WitnessSource<'tcx>>,
    },
    /// The bound is declared on an associated type of a trait,
    /// such as `trait U { type T: Trait }`, so the witness can be found
    /// nested within `U`'s witness.
    AssocTypeBound {
        assoc_def_id: DefId,
        args: GenericArgsRef<'tcx>,
        witness_args: WitnessArgs<'tcx>,
        index: AssocTypeBoundIdx,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum ParamBound {
    /// This item's `Self: Trait` is `T: Trait`.
    InSelf,
    /// This item belongs to a trait impl, and the impl's `i`-th where-clause is `T: Trait`.
    ImplWhereClause(BoundIdx),
    /// This item's `i`-th bound (see `erl_bounds`) is `T: Trait`.
    ItemBound(BoundIdx),
}

/// Witnesses passed in by a user of an item (e.g. function caller).
/// The user-side counterpart to the item-side `WitnessParams`.
#[derive(Clone, Debug, Default, TyEncodable, TyDecodable)]
pub struct WitnessArgs<'tcx> {
    pub self_witness: Option<Box<WitnessSource<'tcx>>>,
    pub bounds: IndexVec<BoundIdx, WitnessSource<'tcx>>,
}

impl WitnessArgs<'_> {
    pub fn is_empty(&self) -> bool {
        self.self_witness.is_none() && self.bounds.is_empty()
    }
}

/// Witnesses received by an item when used (e.g. the function definition).
/// The item-side counterpart to the user-side `WitnessArgs`.
#[derive(Clone, Debug)]
pub struct WitnessParams<'tcx> {
    pub self_bound: Option<SelfBound<'tcx>>,
    pub bounds: IndexVec<BoundIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
}

/// The `Self: Trait` bound of a trait item or a trait impl item.
#[derive(Clone, Debug)]
pub enum SelfBound<'tcx> {
    Trait(Unnormalized<'tcx, ty::TraitRef<'tcx>>),
    TraitImpl(TraitImplSelfBound<'tcx>),
}

#[derive(Clone, Debug)]
pub struct TraitImplSelfBound<'tcx> {
    pub impl_def_id: DefId,
    pub bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>,
    pub supertraits: IndexVec<SupertraitIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
    pub where_clauses: IndexVec<BoundIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
}

impl<'tcx> TyCtxt<'tcx> {
    pub fn erl_witness_params(
        self,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> WitnessParams<'tcx> {
        let self_bound = self.erl_entry_trait(def_id).map(|trait_def_id| {
            match self.trait_impl_of_assoc(def_id) {
                Some(impl_def_id) => {
                    SelfBound::TraitImpl(self.erl_trait_impl_self_bound(impl_def_id, args))
                }

                None => {
                    let bound = ty::EarlyBinder::bind(ty::TraitRef::identity(self, trait_def_id));
                    SelfBound::Trait(bound.instantiate(self, args))
                }
            }
        });

        WitnessParams {
            self_bound,
            bounds: self.erl_bounds(def_id).iter_instantiated_copied(self, args).collect(),
        }
    }

    pub fn erl_trait_impl_self_bound(
        self,
        impl_def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> TraitImplSelfBound<'tcx> {
        let bound = self
            .impl_opt_trait_ref(impl_def_id)
            .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"))
            .instantiate(self, args);

        // The supertraits are normalized when they are used.
        let supertraits = self.erl_supertrait_bounds(bound.skip_normalization());
        let where_clauses =
            self.erl_bounds(impl_def_id).iter_instantiated_copied(self, args).collect();

        TraitImplSelfBound { impl_def_id, bound, supertraits, where_clauses }
    }

    /// [`TyCtxt::erl_witness_params`] with the item's identity args, for use within the item.
    pub fn erl_witness_params_identity(self, def_id: DefId) -> WitnessParams<'tcx> {
        self.erl_witness_params(def_id, ty::GenericArgs::identity_for_item(self, def_id))
    }

    fn erl_supertrait_bounds(
        self,
        bound: ty::TraitRef<'tcx>,
    ) -> IndexVec<SupertraitIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>> {
        self.erl_trait_supertraits(bound.def_id)
            .iter_instantiated_copied(self, bound.args)
            .collect()
    }
}

pub struct WitnessResolver<'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: DefId,
    env: FxHashMap<ty::TraitRef<'tcx>, WitnessSource<'tcx>>,
    seen: SmallVec<[ty::TraitRef<'tcx>; 1]>,
}

impl<'tcx> WitnessResolver<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, owner: DefId) -> WitnessResolver<'tcx> {
        let mut resolver =
            WitnessResolver { tcx, owner, env: FxHashMap::default(), seen: SmallVec::new() };
        let params = tcx.erl_witness_params_identity(owner);

        match params.self_bound {
            Some(SelfBound::Trait(self_bound)) => {
                resolver.add_to_env(self_bound, WitnessSource::Param(ParamBound::InSelf));
            }
            Some(SelfBound::TraitImpl(TraitImplSelfBound { bound, where_clauses, .. })) => {
                resolver.add_to_env(bound, WitnessSource::Param(ParamBound::InSelf));
                for (index, where_clause) in where_clauses.into_iter_enumerated() {
                    resolver.add_to_env(
                        where_clause,
                        WitnessSource::Param(ParamBound::ImplWhereClause(index)),
                    );
                }
            }
            None => {}
        }

        for (index, &bound) in params.bounds.iter_enumerated() {
            resolver.add_to_env(bound, WitnessSource::Param(ParamBound::ItemBound(index)));
        }

        resolver
    }

    /// Also adds the supertraits of `bound`. No effect if this trait was already processed.
    fn add_to_env(
        &mut self,
        bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>,
        source: WitnessSource<'tcx>,
    ) {
        let tcx = self.tcx;
        let bound = tcx.erl_normalize(self.owner, bound);
        let Entry::Vacant(entry) = self.env.entry(bound) else { return };
        entry.insert(source.clone());

        for (index, supertrait) in tcx.erl_supertrait_bounds(bound).into_iter_enumerated() {
            self.add_to_env(supertrait, WitnessSource::Supertrait(Box::new(source.clone()), index));
        }
    }

    pub fn witness_args(
        &mut self,
        span: Span,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> WitnessArgs<'tcx> {
        let tcx = self.tcx;

        let params = tcx.erl_witness_params(def_id, args);

        let self_witness = params.self_bound.map(|self_bound| match self_bound {
            SelfBound::Trait(bound) => Box::new(self.witness_source(span, bound)),
            SelfBound::TraitImpl(_) => span_bug!(
                span,
                "call to the trait impl item `{}`, not to its trait item",
                tcx.def_path_str(def_id)
            ),
        });

        let bounds =
            params.bounds.into_iter().map(|bound| self.witness_source(span, bound)).collect();

        WitnessArgs { self_witness, bounds }
    }

    pub fn witness_source(
        &mut self,
        span: Span,
        bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>,
    ) -> WitnessSource<'tcx> {
        let tcx = self.tcx;
        let bound = tcx.erl_normalize(self.owner, bound);

        if let Some(source) = self.env.get(&bound) {
            return source.clone();
        }

        if let Some(source) = self.witness_source_for_assoc_type_bound(span, bound) {
            return source;
        }

        let Some((impl_def_id, impl_args)) = tcx.erl_select_impl(self.owner, bound) else {
            tcx.dcx()
                .span_fatal(span, format!("witness for the bound not yet implemented: {bound:?}"));
        };

        if self.seen.contains(&bound) {
            tcx.dcx()
                .span_fatal(span, format!("recursive impl witness not yet implemented: {bound:?}"));
        }

        let limit = tcx.recursion_limit();
        if !limit.value_within_limit(self.seen.len()) {
            tcx.dcx().span_fatal(
                span,
                format!(
                    "the impl witness of `{}` nests deeper than the recursion limit ({limit})",
                    self.seen[0]
                ),
            );
        }
        self.seen.push(bound);

        let source = ensure_sufficient_stack(|| {
            let impl_self_bound = tcx.erl_trait_impl_self_bound(impl_def_id, impl_args);

            let supertraits = (impl_self_bound.supertraits.into_iter())
                .map(|supertrait| self.witness_source(span, supertrait))
                .collect();

            let where_clauses = (impl_self_bound.where_clauses.into_iter())
                .map(|where_clause| self.witness_source(span, where_clause))
                .collect();

            WitnessSource::Impl { impl_def_id, args: impl_args, supertraits, where_clauses }
        });

        self.seen.pop();
        source
    }

    /// The source of `bound`'s witness, if `bound` is a declared bound of an associated type.
    ///
    /// This needs `bound`'s self type to be a projection, like `<T as Trait>::A`.
    /// If `Trait` declares `type A: AnotherTrait`, only the impl behind `T: Trait` knows
    /// its witness.
    ///
    /// Note that in some situations the projection can be concretely known within this item,
    /// and then this function returns `None`.
    fn witness_source_for_assoc_type_bound(
        &mut self,
        span: Span,
        bound: ty::TraitRef<'tcx>,
    ) -> Option<WitnessSource<'tcx>> {
        let tcx = self.tcx;

        let mut self_ty = bound.self_ty();
        while let ty::Alias(alias) = self_ty.kind()
            && let ty::AliasTyKind::Projection { def_id: assoc_def_id } = alias.kind
        {
            let declared_bounds =
                tcx.erl_assoc_type_bounds(assoc_def_id).iter_instantiated_copied(tcx, alias.args);
            let found = declared_bounds.enumerate().find_map(|(index, declared)| {
                let index = AssocTypeBoundIdx::from_usize(index);
                let declared = tcx.erl_normalize(self.owner, declared);

                self.supertrait_path(declared, bound).map(|path| (index, path))
            });

            let Some((index, path)) = found else {
                self_ty = alias.self_ty();
                continue;
            };

            let witness_args = self.witness_args(span, assoc_def_id, alias.args);
            let source = WitnessSource::AssocTypeBound {
                assoc_def_id,
                args: alias.args,
                witness_args,
                index,
            };
            return Some(
                path.into_iter()
                    .fold(source, |of, index| WitnessSource::Supertrait(Box::new(of), index)),
            );
        }

        None
    }

    /// The supertraits that lead from the bound `from` to the bound `to`.
    fn supertrait_path(
        &self,
        from: ty::TraitRef<'tcx>,
        to: ty::TraitRef<'tcx>,
    ) -> Option<Vec<SupertraitIdx>> {
        if from == to {
            return Some(Vec::new());
        }
        let tcx = self.tcx;
        tcx.erl_supertrait_bounds(from).into_iter_enumerated().find_map(|(index, supertrait)| {
            let supertrait = tcx.erl_normalize(self.owner, supertrait);
            self.supertrait_path(supertrait, to).map(|mut path| {
                path.insert(0, index);
                path
            })
        })
    }
}
