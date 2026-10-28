use rustc_data_structures::fx::{FxHashMap, FxIndexMap, FxIndexSet};
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hir::def_id::DefId;
use rustc_index::{Idx, IndexVec};
use rustc_middle::bug;
use rustc_middle::erl::witness::{BoundIdx, SupertraitIdx};
use rustc_middle::ty::{self, GenericArgsRef, TyCtxt, Unnormalized};
use rustc_span::Span;

use super::plan::{ActionId, FactId, Problem};

/// For a given bound `T: Trait`, describe where its witness (how we know the bound holds)
/// comes from in a given item, in a way that can be translated to runtime code.
#[derive(Clone, Debug)]
pub(super) enum WitnessSource<'tcx> {
    /// This item's `Self: Trait` is `T: Trait`, passed in as a parameter.
    SelfParam,
    /// At this point in code, we know exactly which impl proves the bound,
    /// so we get its witness by calling the impl's witness constructor.
    ImplCall { impl_def_id: DefId, where_clauses: IndexVec<BoundIdx, ty::TraitRef<'tcx>> },
    /// At this point in code, we know exactly which impl proves the bound,
    /// so we build its witness in place.
    ImplBuild {
        impl_def_id: DefId,
        supertraits: IndexVec<SupertraitIdx, ty::TraitRef<'tcx>>,
        where_clauses: IndexVec<BoundIdx, ty::TraitRef<'tcx>>,
    },
    /// The witness is the `usize`-th element of a `WitnessTuple`.
    Element(WitnessTuple<'tcx>, usize),
}

/// A tuple of witnesses that is obtained once, and then any number of its elements are read
/// at the cost of one read. Some of them are found in values the item already has, and others
/// are computed by a call.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum WitnessTuple<'tcx> {
    /// This item's bounds (see `erl_bounds`), passed in as a parameter.
    ItemBounds,
    /// This item belongs to a trait impl, and these are the impl's where-clauses,
    /// found in the self witness.
    ImplWhereClauses,
    /// The supertraits of the bound `of`, found in its witness: `T: Trait` is proven by
    /// the subtrait's bound `T: U` with `trait U: Trait`.
    Supertraits { of: ty::TraitRef<'tcx> },
    /// The bounds declared on an associated type of a trait,
    /// such as `trait U<A: Eq> { type T<B: Ord>: Trait<B> }`,
    /// returned all at once by a call through the vtable of `of`.
    ///
    /// In this example, `of` is a witness for `U<A: Eq>`,
    /// and `bounds` has witnesses for `[A: Eq, B: Ord]`.
    AssocTypeBounds {
        of: ty::TraitRef<'tcx>,
        assoc_def_id: DefId,
        bounds: IndexVec<BoundIdx, ty::TraitRef<'tcx>>,
    },
}

/// Witnesses passed in by a user of an item (e.g. function caller).
/// This is the user-side counterpart to the item-side `WitnessParams`.
#[derive(Clone, Debug, Default)]
pub(super) struct WitnessArgs<'tcx> {
    pub self_witness: Option<ty::TraitRef<'tcx>>,
    pub bounds: IndexVec<BoundIdx, ty::TraitRef<'tcx>>,
}

impl WitnessArgs<'_> {
    pub(super) fn is_empty(&self) -> bool {
        self.self_witness.is_none() && self.bounds.is_empty()
    }
}

/// Witnesses received by an item when used (e.g. the function definition).
/// This is the item-side counterpart to the user-side `WitnessArgs`.
#[derive(Clone, Debug)]
pub(super) struct WitnessParams<'tcx> {
    /// The `Self: Trait` bound of a trait item or a trait impl item.
    pub self_bound: Option<Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
    /// The impl's where-clauses, found in the self witness.
    pub impl_where_clauses: IndexVec<BoundIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
    /// The item's own bounds.
    pub bounds: IndexVec<BoundIdx, Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
}

pub(super) fn witness_params<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    args: GenericArgsRef<'tcx>,
) -> WitnessParams<'tcx> {
    let bounds = tcx.erl_bounds(def_id).iter_instantiated_copied(tcx, args).collect();

    match (tcx.erl_entry_trait(def_id), tcx.trait_impl_of_assoc(def_id)) {
        (Some(_trait_def_id), Some(impl_def_id)) => WitnessParams {
            self_bound: Some(tcx.impl_trait_ref(impl_def_id).instantiate(tcx, args)),
            impl_where_clauses: tcx
                .erl_bounds(impl_def_id)
                .iter_instantiated_copied(tcx, args)
                .collect(),
            bounds,
        },

        (Some(trait_def_id), None) => WitnessParams {
            self_bound: Some(Unnormalized::new(ty::TraitRef::from_assoc(tcx, trait_def_id, args))),
            impl_where_clauses: Default::default(),
            bounds,
        },

        (None, _) => {
            WitnessParams { self_bound: None, impl_where_clauses: Default::default(), bounds }
        }
    }
}

/// Where witnesses are built, which decides how the witness of an impl is obtained.
#[derive(Clone, Copy)]
pub(super) enum WitnessBuildSite {
    User,
    WitnessConstructor,
}

/// What an action does at runtime to obtain its witnesses.
///
/// Equal labels run the same code, and multiple witnesses can collapse
/// to the same label when their type bounds erase to the same value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ActionLabel<'tcx> {
    SelfParam,
    ImplCall {
        impl_def_id: DefId,
        where_clauses: IndexVec<BoundIdx, ty::TraitRef<'tcx>>,
    },
    ImplBuild {
        impl_def_id: DefId,
        supertraits: IndexVec<SupertraitIdx, ty::TraitRef<'tcx>>,
        where_clauses: IndexVec<BoundIdx, ty::TraitRef<'tcx>>,
    },
    Tuple(WitnessTuple<'tcx>),
}

impl<'tcx> ActionLabel<'tcx> {
    fn as_source(&self, index: usize) -> WitnessSource<'tcx> {
        match self {
            ActionLabel::SelfParam => WitnessSource::SelfParam,
            ActionLabel::ImplCall { impl_def_id, where_clauses } => WitnessSource::ImplCall {
                impl_def_id: *impl_def_id,
                where_clauses: where_clauses.clone(),
            },
            ActionLabel::ImplBuild { impl_def_id, supertraits, where_clauses } => {
                WitnessSource::ImplBuild {
                    impl_def_id: *impl_def_id,
                    supertraits: supertraits.clone(),
                    where_clauses: where_clauses.clone(),
                }
            }
            ActionLabel::Tuple(tuple) => WitnessSource::Element(tuple.clone(), index),
        }
    }

    fn preconditions(&self) -> impl Iterator<Item = ty::TraitRef<'tcx>> {
        let (of, first, second): (Option<_>, &[_], &[_]) = match self {
            ActionLabel::SelfParam
            | ActionLabel::Tuple(WitnessTuple::ItemBounds | WitnessTuple::ImplWhereClauses) => {
                (None, &[], &[])
            }
            ActionLabel::ImplCall { where_clauses, .. } => (None, &where_clauses.raw, &[]),
            ActionLabel::ImplBuild { supertraits, where_clauses, .. } => {
                (None, &where_clauses.raw, &supertraits.raw)
            }
            ActionLabel::Tuple(WitnessTuple::Supertraits { of }) => (Some(of), &[], &[]),
            ActionLabel::Tuple(WitnessTuple::AssocTypeBounds { of, bounds, .. }) => {
                (Some(of), &bounds.raw, &[])
            }
        };
        of.into_iter().chain(first).chain(second).copied()
    }

    /// The estimated runtime cost of the action.
    ///
    /// We don't model all accesses at this level, exactly. For example, `AssocTypeBounds` needs
    /// a call, but for that call to happen, we first need to retrieve the callee fun from
    /// the vtable of `AssocTypeBounds.of`, which can cost us a tuple check. In practice,
    /// the differences should very rarely affect the plan.
    ///
    /// To properly model this, we'd have to have a structure here that roughly follows `VarNodeKey`
    /// from `witness.rs`. That makes sense, especially if we return this structure for lowering
    /// (then lowering becomes more of a mirroring) - but it also increases the number of effects
    /// to solve for in LP solver, and may be net additional complexity with minor upside.
    fn cost(&self) -> u32 {
        const ELEMENT_COST: u32 = 1;
        const TUPLE_COST: u32 = 3;
        const CALL_COST: u32 = 8;

        match self {
            ActionLabel::SelfParam => 0,
            ActionLabel::ImplCall { .. } => CALL_COST,
            ActionLabel::ImplBuild { .. } => TUPLE_COST,
            ActionLabel::Tuple(WitnessTuple::ItemBounds) => ELEMENT_COST,
            ActionLabel::Tuple(
                WitnessTuple::ImplWhereClauses | WitnessTuple::Supertraits { .. },
            ) => 2 * ELEMENT_COST,
            ActionLabel::Tuple(WitnessTuple::AssocTypeBounds { .. }) => {
                2 * ELEMENT_COST + CALL_COST
            }
        }
    }
}

pub(super) struct WitnessResolver<'tcx> {
    tcx: TyCtxt<'tcx>,
    typing_env: ty::TypingEnv<'tcx>,
    build_site: WitnessBuildSite,

    /// The solver problem we're building up while proving witnesses.
    problem: Problem,
    /// The witnesses obtained by any action, indexed by `FactId`.
    facts: FxIndexSet<ty::TraitRef<'tcx>>,
    /// The witnesses obtained by each action, indexed by `ActionId`.
    /// The index of witness in `Vec` is also its tuple index within the WitnessSource
    /// (see `as_source`).
    actions: FxIndexMap<ActionLabel<'tcx>, Vec<ty::TraitRef<'tcx>>>,
    /// The witnesses directly used in the body - these are our solver goals.
    goals: FxIndexSet<FactId>,

    proving: Vec<ty::TraitRef<'tcx>>,
}

impl<'tcx> WitnessResolver<'tcx> {
    pub(super) fn new(
        tcx: TyCtxt<'tcx>,
        owner: DefId,
        site: WitnessBuildSite,
    ) -> WitnessResolver<'tcx> {
        let mut resolver = WitnessResolver {
            tcx,
            typing_env: ty::TypingEnv::post_analysis(tcx, owner),
            build_site: site,
            problem: Problem::new(),
            facts: FxIndexSet::default(),
            actions: FxIndexMap::default(),
            goals: FxIndexSet::default(),
            proving: Vec::new(),
        };

        let params = witness_params(tcx, owner, ty::GenericArgs::identity_for_item(tcx, owner));

        let self_bound = resolver.normalize_all(params.self_bound);
        resolver.add_action(ActionLabel::SelfParam, self_bound);

        let impl_where_clauses = resolver.normalize_all(params.impl_where_clauses);
        resolver.add_action(ActionLabel::Tuple(WitnessTuple::ImplWhereClauses), impl_where_clauses);

        let bounds = resolver.normalize_all(params.bounds);
        resolver.add_action(ActionLabel::Tuple(WitnessTuple::ItemBounds), bounds);

        resolver
    }

    /// Witness arguments for `def_id` parameterized by generic `args`, used directly by the body.
    pub(super) fn require_witness_args(
        &mut self,
        span: Span,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> WitnessArgs<'tcx> {
        let args = self.prove_args(span, def_id, args);
        for &bound in args.self_witness.iter().chain(&args.bounds) {
            let fact = self.fact(bound);
            self.goals.insert(fact);
        }
        args
    }

    /// The normalized `bound`, used directly by the body.
    pub(super) fn require_witness(
        &mut self,
        span: Span,
        bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>,
    ) -> ty::TraitRef<'tcx> {
        let bound = self.prove_bound(span, bound);
        let fact = self.fact(bound);
        self.goals.insert(fact);
        bound
    }

    /// Figure out how to built the required witnesses from the facts we have at function entry,
    /// at the lowest cost total. Returns the selected source of every witness kept.
    pub(super) fn resolve_witnesses(
        mut self,
    ) -> FxHashMap<ty::TraitRef<'tcx>, WitnessSource<'tcx>> {
        let goals: Box<[_]> = self.goals.iter().copied().collect();

        self.problem
            .solve(&goals)
            .into_iter()
            .map(|(fact, action)| {
                let (label, effects) = self
                    .actions
                    .get_index(action.index())
                    .unwrap_or_else(|| bug!("no label for {action:?}"));

                let &bound = self
                    .facts
                    .get_index(fact.as_usize())
                    .unwrap_or_else(|| bug!("no witness for {fact:?}"));

                let tuple_index = effects
                    .iter()
                    .position(|&effect| effect == bound)
                    .unwrap_or_else(|| bug!("{label:?} doesn't obtain {bound:?}"));

                (bound, label.as_source(tuple_index))
            })
            .collect()
    }

    /// The bounds of the witness arguments for `def_id` parameterized by generic `args`,
    /// after proving them.
    fn prove_args(
        &mut self,
        span: Span,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> WitnessArgs<'tcx> {
        let params = witness_params(self.tcx, def_id, args);
        let self_witness = params.self_bound.map(|bound| self.prove_bound(span, bound));
        let bounds = params.bounds.into_iter().map(|bound| self.prove_bound(span, bound)).collect();
        WitnessArgs { self_witness, bounds }
    }

    /// The normalized `bound`, proven by adding the actions that can obtain its witness,
    /// unless some action obtains it already.
    fn prove_bound(
        &mut self,
        span: Span,
        bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>,
    ) -> ty::TraitRef<'tcx> {
        let tcx = self.tcx;
        let bound = self.normalize(bound);
        if self.facts.contains(&bound) {
            return bound;
        }

        if self.proving.contains(&bound) {
            tcx.dcx()
                .span_fatal(span, format!("recursive impl witness not yet implemented: {bound:?}"));
        }

        let limit = tcx.recursion_limit();
        if !limit.value_within_limit(self.proving.len()) {
            tcx.dcx().span_fatal(
                span,
                format!(
                    "the impl witness of `{}` nests deeper than the recursion limit ({limit})",
                    self.proving[0]
                ),
            );
        }

        self.proving.push(bound);
        ensure_sufficient_stack(|| {
            if !self.prove_from_assoc_type_bounds(span, bound) {
                self.prove_from_impl(span, bound);
            }
        });
        self.proving.pop();

        bound
    }

    /// Adds the actions obtaining the declared bounds of the associated types projected in
    /// `bound`'s self type, from the outermost, until `bound` follows from them, and returns
    /// whether it does.
    ///
    /// This needs `bound`'s self type to be a projection, like `<T as Trait>::A`.
    /// If `Trait` declares `type A: AnotherTrait`, only the impl behind `T: Trait` knows
    /// its witness.
    ///
    /// Note that in some situations the projection can be concretely known within this item,
    /// and then there's no such action.
    fn prove_from_assoc_type_bounds(&mut self, span: Span, bound: ty::TraitRef<'tcx>) -> bool {
        let tcx = self.tcx;

        let mut self_ty = bound.self_ty();
        while let ty::Alias(alias) = self_ty.kind()
            && let ty::AliasTyKind::Projection { def_id: assoc_def_id } = alias.kind
        {
            self_ty = alias.self_ty();

            let declared = tcx.erl_assoc_type_bounds(assoc_def_id);
            let declared: Vec<_> =
                self.normalize_all(declared.iter_instantiated_copied(tcx, alias.args));
            if declared.is_empty() {
                continue;
            }

            let WitnessArgs { self_witness, bounds } =
                self.prove_args(span, assoc_def_id, alias.args);
            let of = self_witness.unwrap_or_else(|| bug!("associated type without a self witness"));
            let label =
                ActionLabel::Tuple(WitnessTuple::AssocTypeBounds { of, assoc_def_id, bounds });
            self.add_action(label, declared);

            if self.facts.contains(&bound) {
                return true;
            }
        }

        false
    }

    /// Adds the action building the witness of the impl proving `bound`.
    fn prove_from_impl(&mut self, span: Span, bound: ty::TraitRef<'tcx>) {
        let tcx = self.tcx;
        let Some((impl_def_id, impl_args)) = tcx.select_impl(self.typing_env, bound) else {
            tcx.dcx()
                .span_fatal(span, format!("witness for the bound not yet implemented: {bound:?}"));
        };

        let where_clauses: IndexVec<BoundIdx, _> = tcx
            .erl_bounds(impl_def_id)
            .iter_instantiated_copied(tcx, impl_args)
            .map(|where_clause| self.prove_bound(span, where_clause))
            .collect();

        let label = match self.build_site {
            WitnessBuildSite::User => ActionLabel::ImplCall { impl_def_id, where_clauses },
            WitnessBuildSite::WitnessConstructor => {
                // A witness built in place contains the witnesses of its supertraits.
                let supertraits: IndexVec<SupertraitIdx, _> = self
                    .supertraits(bound)
                    .into_iter()
                    .map(|supertrait| self.prove_bound(span, Unnormalized::new(supertrait)))
                    .collect();
                ActionLabel::ImplBuild { impl_def_id, supertraits, where_clauses }
            }
        };

        self.add_action(label, vec![bound]);
    }

    fn add_action(&mut self, label: ActionLabel<'tcx>, effects: Vec<ty::TraitRef<'tcx>>) {
        if effects.is_empty() {
            return;
        }

        let mut newly_added = Vec::new();
        for &bound in &effects {
            let (index, inserted) = self.facts.insert_full(bound);
            if inserted {
                assert_eq!(index, self.problem.push_fact().as_usize());
                newly_added.push(bound);
            }
        }
        let effect_facts: Vec<_> = effects.iter().map(|&bound| self.fact(bound)).collect();

        if let Some(index) = self.actions.get_index_of(&label) {
            self.actions[index].extend(effects);
            self.problem.add_effects(ActionId::new(index), effect_facts);
        } else {
            let preconditions = label.preconditions().map(|bound| self.fact(bound)).collect();
            let action = self.problem.push_action(preconditions, effect_facts, label.cost());
            let (index, _) = self.actions.insert_full(label, effects);
            assert_eq!(index, action.as_usize());
        }

        for bound in newly_added {
            let label = ActionLabel::Tuple(WitnessTuple::Supertraits { of: bound });
            self.add_action(label, self.supertraits(bound).raw);
        }
    }

    fn fact(&self, bound: ty::TraitRef<'tcx>) -> FactId {
        let index = self
            .facts
            .get_index_of(&bound)
            .unwrap_or_else(|| bug!("no action obtains the witness of {bound:?}"));

        FactId::new(index)
    }

    fn supertraits(
        &self,
        bound: ty::TraitRef<'tcx>,
    ) -> IndexVec<SupertraitIdx, ty::TraitRef<'tcx>> {
        let supertraits = self.tcx.erl_trait_supertraits(bound.def_id);
        self.normalize_all(supertraits.iter_instantiated_copied(self.tcx, bound.args))
    }

    fn normalize(&self, bound: Unnormalized<'tcx, ty::TraitRef<'tcx>>) -> ty::TraitRef<'tcx> {
        self.tcx.normalize_erasing_regions_or_unchanged(self.typing_env, bound)
    }

    fn normalize_all<C: FromIterator<ty::TraitRef<'tcx>>>(
        &self,
        bounds: impl IntoIterator<Item = Unnormalized<'tcx, ty::TraitRef<'tcx>>>,
    ) -> C {
        bounds.into_iter().map(|bound| self.normalize(bound)).collect()
    }
}
