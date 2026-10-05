//! Impl witnesses: runtime evidence that a trait bound holds.
//!
//! The witness of `Ty: Trait` names the impl that the site proving the bound selected:
//! `{Tag, HostModule, S1, …, Sk, W1, …, Wn}`.
//! - `Tag` identifies the impl in `HostModule` (see `TyCtxt::erl_impl_tag`).
//! - `HostModule` contains the functions of the impl. Dispatch calls `(element(2, W)):F(W, …)`.
//! - `S1 … Sk` are the witnesses of `Ty` for the direct supertraits of `Trait`.
//! - `W1 … Wn` are the witnesses of the where-clauses of the impl.

use std::collections::hash_map::Entry;

use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::ty::{self, EarlyBinder, GenericArgsRef, TyCtxt};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};

use super::Translator;
use super::scopes::Scopes;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

/// The witnesses that a body has from its witness parameters.
#[derive(Default)]
pub(crate) struct WitnessEnv<'tcx, 'arena> {
    /// For each bound that the body can prove, the expression of its witness.
    bounds: FxHashMap<ty::TraitRef<'tcx>, eaf::Expr<'arena>>,
    /// The witness parameter variables
    pub(super) vars: SmallVec<[Symbol; 4]>,
}

/// The 1-based position of the host module in an impl witness. The tag is at position 1.
const HOST_INDEX: u32 = 2;

/// The 1-based position of the first element (supertrait or where-clause witness) in an impl
/// witness.
const FIRST_ELEMENT_INDEX: u32 = 3;

fn instantiate<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_ref: ty::TraitRef<'tcx>,
    args: GenericArgsRef<'tcx>,
) -> ty::TraitRef<'tcx> {
    EarlyBinder::bind(trait_ref).instantiate(tcx, args).skip_normalization()
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    fn supertrait_refs(&self, trait_ref: ty::TraitRef<'tcx>) -> SmallVec<[ty::TraitRef<'tcx>; 4]> {
        let tcx = self.tcx;
        tcx.erl_trait_supertraits(trait_ref.def_id)
            .iter()
            .map(|&supertrait| instantiate(tcx, supertrait, trait_ref.args))
            .collect()
    }

    /// Also adds the supertrait witnesses. A bound that `env` already has keeps its witness.
    pub(super) fn add_witness_to_env(
        &self,
        owner: DefId,
        span: Span,
        env: &mut WitnessEnv<'tcx, 'arena>,
        trait_ref: ty::TraitRef<'tcx>,
        expr: eaf::Expr<'arena>,
    ) {
        let trait_ref = self.tcx.erl_normalize(owner, trait_ref);
        let Entry::Vacant(entry) = env.bounds.entry(trait_ref) else { return };
        entry.insert(expr);

        for (index, supertrait) in self.supertrait_refs(trait_ref).into_iter().enumerate() {
            let element = self.element_expr(span, expr, FIRST_ELEMENT_INDEX + index as u32);
            self.add_witness_to_env(owner, span, env, supertrait, element);
        }
    }

    fn element_expr(&self, span: Span, tuple: eaf::Expr<'arena>, index: u32) -> eaf::Expr<'arena> {
        let afb = &self.afb;
        afb.expr_remote_call(
            span,
            afb.expr_atom(span, sym::erlang),
            afb.expr_atom(span, sym::element),
            [afb.expr_int(span, index.into()), tuple],
        )
    }

    pub(super) fn impl_witness_pat(
        &self,
        impl_def_id: DefId,
        span: Span,
        elements: impl IntoIterator<Item = eaf::Pattern<'arena>>,
    ) -> eaf::Pattern<'arena> {
        let afb = &self.afb;
        let tag = self.tcx.erl_impl_tag(impl_def_id);
        let head = [afb.pat_atom(span, tag), afb.pat_var(span, kw::Underscore)];
        afb.pat_tuple(span, head.into_iter().chain(elements))
    }

    /// The trait refs whose witnesses are the elements of an impl witness of `impl_def_id`, in
    /// order: the supertraits of the implemented trait, then the where-clauses of the impl.
    fn impl_witness_bounds(
        &self,
        impl_def_id: DefId,
        impl_args: GenericArgsRef<'tcx>,
    ) -> SmallVec<[ty::TraitRef<'tcx>; 2]> {
        let tcx = self.tcx;
        let trait_ref = tcx
            .impl_opt_trait_ref(impl_def_id)
            .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"))
            .instantiate(tcx, impl_args)
            .skip_normalization();
        let where_clauses =
            tcx.erl_bounds(impl_def_id).iter().map(|&bound| instantiate(tcx, bound, impl_args));
        self.supertrait_refs(trait_ref).into_iter().chain(where_clauses).collect()
    }

    pub(super) fn impl_witness_element_count(&self, impl_def_id: DefId) -> usize {
        let tcx = self.tcx;
        let trait_def_id = tcx
            .impl_opt_trait_id(impl_def_id)
            .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"));
        tcx.erl_trait_supertraits(trait_def_id).len() + tcx.erl_bounds(impl_def_id).len()
    }

    /// The witness parameter patterns of `def_id`, and the witnesses of its body.
    pub(super) fn translate_witness_params(
        &self,
        def_id: LocalDefId,
        span: Span,
        scopes: &mut Scopes,
    ) -> (SmallVec<[eaf::Pattern<'arena>; 8]>, WitnessEnv<'tcx, 'arena>) {
        let tcx = self.tcx;
        let afb = &self.afb;
        let owner = def_id.to_def_id();

        let mut params = SmallVec::new();
        let mut env = WitnessEnv::default();

        if let Some(trait_def_id) = tcx.erl_entry_trait(owner) {
            let self_var = scopes.fresh_var(sym::witness).erl_name;
            let self_expr = afb.expr_var(span, self_var);
            env.vars.push(self_var);

            match tcx.trait_impl_of_assoc(owner) {
                Some(impl_def_id) => {
                    let trait_ref = tcx
                        .impl_opt_trait_ref(impl_def_id)
                        .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"))
                        .instantiate_identity()
                        .skip_normalization();

                    // Bind the elements first, so that the body uses these variables and not
                    // `element/2` calls on the self witness.
                    let identity = ty::GenericArgs::identity_for_item(tcx, impl_def_id);
                    let elements = self
                        .impl_witness_bounds(impl_def_id, identity)
                        .into_iter()
                        .map(|bound| {
                            let var = scopes.fresh_var(sym::witness).erl_name;
                            env.vars.push(var);
                            self.add_witness_to_env(
                                owner,
                                span,
                                &mut env,
                                bound,
                                afb.expr_var(span, var),
                            );
                            afb.pat_var(span, var)
                        })
                        .collect::<SmallVec<[_; 2]>>();
                    self.add_witness_to_env(owner, span, &mut env, trait_ref, self_expr);

                    let witness_pat = self.impl_witness_pat(impl_def_id, span, elements);
                    params.push(afb.pat_match(span, afb.pat_var(span, self_var), witness_pat));
                }

                // A default body serves every impl, so it does not match a tag.
                None => {
                    let trait_ref = ty::TraitRef::identity(tcx, trait_def_id);
                    self.add_witness_to_env(owner, span, &mut env, trait_ref, self_expr);
                    params.push(afb.pat_var(span, self_var));
                }
            }
        }

        let bounds = tcx.erl_bounds(owner);
        if !bounds.is_empty() {
            let bound_pats = bounds.iter().map(|&bound| {
                let var = scopes.fresh_var(sym::witness).erl_name;
                env.vars.push(var);
                self.add_witness_to_env(owner, span, &mut env, bound, afb.expr_var(span, var));
                afb.pat_var(span, var)
            });
            params.push(afb.pat_tuple(span, bound_pats));
        }

        (params, env)
    }

    pub(super) fn call_witness_exprs(
        &self,
        owner: DefId,
        span: Span,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
        env: &WitnessEnv<'tcx, 'arena>,
        scopes: &mut Scopes,
    ) -> SmallVec<[eaf::Expr<'arena>; 8]> {
        let tcx = self.tcx;
        if tcx.trait_item_of(def_id).is_some() {
            span_bug!(
                span,
                "call to the trait impl item `{}`, not to its trait item",
                tcx.def_path_str(def_id)
            );
        }

        let mut exprs = SmallVec::new();
        if let Some(trait_def_id) = tcx.trait_of_assoc(def_id) {
            let trait_ref = ty::TraitRef::from_assoc(tcx, trait_def_id, args);
            exprs.push(self.resolve_witness(owner, span, trait_ref, env, scopes));
        }

        let bounds = tcx.erl_bounds(def_id);
        if !bounds.is_empty() {
            let witnesses = bounds.iter().map(|&bound| {
                self.resolve_witness(owner, span, instantiate(tcx, bound, args), env, scopes)
            });
            exprs.push(self.afb.expr_tuple(span, witnesses));
        }

        exprs
    }

    /// The expression of the witness of `trait_ref`.
    pub(super) fn resolve_witness(
        &self,
        owner: DefId,
        span: Span,
        trait_ref: ty::TraitRef<'tcx>,
        env: &WitnessEnv<'tcx, 'arena>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx;
        let afb = &self.afb;
        let trait_ref = tcx.erl_normalize(owner, trait_ref);

        if let Some(&expr) = env.bounds.get(&trait_ref) {
            return expr;
        }

        if let Some(expr) = self.resolve_assoc_type_bound(owner, span, trait_ref, env, scopes) {
            return expr;
        }

        let Some((impl_def_id, impl_args)) = tcx.erl_select_impl(owner, trait_ref) else {
            not_implemented!(tcx, span, "witness for the bound", trait_ref);
        };

        let Some(host) = tcx.erl_definition_module(impl_def_id) else {
            not_implemented!(tcx, span, "witness for an impl with a non-ADT self type", trait_ref);
        };

        // An impl witness holds its elements, so a bound that needs itself has no finite witness.
        // A bound can also need ever larger versions of itself, like `S<S<T>>: Derived`.
        {
            let mut in_progress = self.witnesses_in_progress.borrow_mut();
            if in_progress.contains(&trait_ref) {
                not_implemented!(tcx, span, "recursive impl witness", trait_ref);
            }
            let limit = tcx.recursion_limit();
            if !limit.value_within_limit(in_progress.len()) {
                span_fatal!(
                    tcx,
                    span,
                    "the impl witness of `{}` nests deeper than the recursion limit ({limit})",
                    in_progress[0]
                );
            }
            in_progress.push(trait_ref);
        }

        let elements = ensure_sufficient_stack(|| {
            self.impl_witness_bounds(impl_def_id, impl_args)
                .into_iter()
                .map(|bound| self.resolve_witness(owner, span, bound, env, scopes))
                .collect::<SmallVec<[_; 2]>>()
        });

        self.witnesses_in_progress.borrow_mut().pop();

        let head = [
            afb.expr_atom(span, tcx.erl_impl_tag(impl_def_id)),
            afb.expr_atom(span, tcx.erl_module_name(host)),
        ];
        afb.expr_tuple(span, head.into_iter().chain(elements))
    }

    /// If a declared bound of an associated type proves `trait_ref`, whose self type is a
    /// projection like `<X as Tr>::A`. The bound can be on `A`, or on an associated type that
    /// `A` is nested in, like `Outer::Inner` in `<<T as Outer>::Inner as HasItem>::Item`.
    fn resolve_assoc_type_bound(
        &self,
        owner: DefId,
        span: Span,
        trait_ref: ty::TraitRef<'tcx>,
        env: &WitnessEnv<'tcx, 'arena>,
        scopes: &mut Scopes,
    ) -> Option<eaf::Expr<'arena>> {
        let tcx = self.tcx;

        let mut self_ty = trait_ref.self_ty();
        while let ty::Alias(alias) = self_ty.kind()
            && let ty::AliasTyKind::Projection { def_id: assoc_def_id } = alias.kind
        {
            let found = tcx.erl_assoc_type_bounds(assoc_def_id).iter().enumerate().find_map(
                |(index, &bound)| {
                    let bound = tcx.erl_normalize(owner, instantiate(tcx, bound, alias.args));
                    self.supertrait_path(owner, bound, trait_ref).map(|path| (index, path))
                },
            );
            let Some((index, path)) = found else {
                self_ty = alias.self_ty();
                continue;
            };

            // The projection does not normalize, so the impl is not known and the call
            // dispatches.
            let mut args = self
                .call_witness_exprs(owner, span, assoc_def_id, alias.args, env, scopes)
                .into_iter();
            let of = args.next().unwrap_or_else(|| bug!("associated type without a self witness"));
            let bounds = self.dispatch_call(span, tcx.erl_fn_name(assoc_def_id), of, args, scopes);

            let witness = self.element_expr(span, bounds, index as u32 + 1);
            return Some(path.into_iter().fold(witness, |witness, supertrait| {
                self.element_expr(span, witness, FIRST_ELEMENT_INDEX + supertrait as u32)
            }));
        }

        None
    }

    /// The supertrait indices from the witness of `from` to the witness of `to`.
    fn supertrait_path(
        &self,
        owner: DefId,
        from: ty::TraitRef<'tcx>,
        to: ty::TraitRef<'tcx>,
    ) -> Option<Vec<usize>> {
        if from == to {
            return Some(Vec::new());
        }
        self.supertrait_refs(from).into_iter().enumerate().find_map(|(index, supertrait)| {
            let supertrait = self.tcx.erl_normalize(owner, supertrait);
            self.supertrait_path(owner, supertrait, to).map(|mut path| {
                path.insert(0, index);
                path
            })
        })
    }

    /// Call `name` through `receiver_witness`: `(element(HOST_INDEX, W)):name(W, Args…)`.
    pub(super) fn dispatch_call(
        &self,
        span: Span,
        name: Symbol,
        receiver_witness: eaf::Expr<'arena>,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let (binding, receiver_witness) = match receiver_witness {
            eaf::Expr::Var(..) => (None, receiver_witness),
            _ => {
                let var = scopes.fresh_var(sym::witness).erl_name;
                let binding = afb.expr_match(span, afb.pat_var(span, var), receiver_witness);
                (Some(binding), afb.expr_var(span, var))
            }
        };

        let module = self.element_expr(span, receiver_witness, HOST_INDEX);
        let call = afb.expr_remote_call(
            span,
            module,
            afb.expr_atom(span, name),
            [receiver_witness].into_iter().chain(args),
        );

        match binding {
            Some(binding) => afb.expr_block(span, [binding, call]),
            None => call,
        }
    }
}
