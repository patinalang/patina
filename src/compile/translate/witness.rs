//! The runtime representation of witnesses.
//!
//! The witness of `Ty: Trait` names the impl that proves the bound:
//! `{Tag, HostModule, S1, …, Sk, W1, …, Wn}`.
//! - `Tag` identifies the impl in `HostModule` (see `TyCtxt::erl_impl_tag`).
//! - `HostModule` contains the functions of the impl. Dispatch calls `(element(2, W)):F(W, …)`.
//! - `S1 … Sk` are the witnesses of `Ty` for the direct supertraits of `Trait`.
//! - `W1 … Wn` are the witnesses of the where-clauses of the impl.

use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::erl::witness::{
    BoundIdx, ParamBound, SelfBound, SupertraitIdx, WitnessArgs, WitnessParams, WitnessSource,
};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};

use super::Translator;
use super::scopes::Scopes;
use crate::compile::eaf_builder::EafBuilder;
use crate::erlang::abstract_format as eaf;

/// The witnesses that a body has from its witness parameters.
#[derive(Default)]
pub(crate) struct WitnessEnv<'arena> {
    /// The span of the witness parameters.
    span: Span,
    /// The variable name of this item's `Self: Trait` witness.
    /// This is present for trait impl items and for trait items (e.g. trait default functions).
    self_witness: Option<Symbol>,
    /// For a trait impl item, the variable names of the elements of the self witness.
    impl_elements: Option<ImplElements>,
    /// The variable names of the witnesses of this item's bounds, in `erl_bounds` order.
    bounds: IndexVec<BoundIdx, Symbol>,
    /// The witness parameters of the function. Patterns within bind the variables of `bounds`.
    pub(super) params: SmallVec<[WitnessParam<'arena>; 2]>,
}

/// A witness parameter of a function.
#[derive(Clone, Copy)]
pub(super) struct WitnessParam<'arena> {
    /// The pattern of the parameter.
    pub(super) pat: eaf::Pattern<'arena>,
    /// The variable that holds the whole witness.
    pub(super) var: eaf::Expr<'arena>,
}

/// The 1-based position of the host module in an impl witness. The tag is at position 1.
const HOST_INDEX: u32 = 2;

/// The 1-based position of the first element (supertrait or where-clause witness) in an impl
/// witness.
const FIRST_ELEMENT_INDEX: u32 = 3;

struct ImplElements {
    /// The variable names of the supertrait witnesses, in `erl_trait_supertraits` order.
    supertraits: IndexVec<SupertraitIdx, Symbol>,
    /// The variable names of the witnesses of the impl's where-clauses, in `erl_bounds` order.
    where_clauses: IndexVec<BoundIdx, Symbol>,
}

impl<'arena> WitnessEnv<'arena> {
    fn var_name_for_param(&self, param: ParamBound) -> Symbol {
        match param {
            ParamBound::InSelf => {
                self.self_witness.unwrap_or_else(|| bug!("no self witness parameter"))
            }
            ParamBound::ImplWhereClause(index) => {
                let elements = self
                    .impl_elements
                    .as_ref()
                    .unwrap_or_else(|| bug!("no trait impl witness parameter"));

                elements.where_clauses[index]
            }
            ParamBound::ItemBound(index) => self.bounds[index],
        }
    }

    fn span_of(&self, span: Span, witness: &WitnessSource<'_>) -> Span {
        match witness {
            WitnessSource::Param(_) => self.span,
            WitnessSource::Supertrait(source, _) => self.span_of(span, source),
            WitnessSource::Impl { .. } | WitnessSource::AssocTypeBound { .. } => span,
        }
    }

    /// Adds a witness parameter.
    ///
    /// If `pat` is given, e.g. `{tag, _, S1}`, the code emits a match
    /// `foo(..., var = {tag, _, S1}, ...)` at the parameter's location.
    ///
    /// If `pat` isn't given, the whole witness is matched :`foo(..., var, ...)`.
    fn push_param(
        &mut self,
        afb: &EafBuilder<'_, 'arena>,
        var_name: Symbol,
        pattern: Option<eaf::Pattern<'arena>>,
    ) {
        let var = afb.expr_var(self.span, var_name);

        let var_pat = afb.pat_var(self.span, var_name);
        let pat = match pattern {
            Some(pat) => afb.pat_match(self.span, var_pat, pat),
            None => var_pat,
        };

        self.params.push(WitnessParam { pat, var });
    }
}

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    /// The number of supertrait and where-clause elements in a witness of the trait impl.
    pub(super) fn impl_witness_element_count(&self, impl_def_id: DefId) -> usize {
        let tcx = self.tcx;
        let trait_def_id = tcx
            .impl_opt_trait_id(impl_def_id)
            .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"));

        tcx.erl_trait_supertraits(trait_def_id).skip_binder().len()
            + tcx.erl_bounds(impl_def_id).skip_binder().len()
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

    pub(super) fn translate_witness_params(
        &self,
        params: &WitnessParams<'tcx>,
        span: Span,
        scopes: &mut Scopes,
    ) -> WitnessEnv<'arena> {
        let afb = &self.afb;
        let mut fresh_var = || scopes.fresh_var(sym::witness).erl_name;
        let mut env = WitnessEnv { span, ..WitnessEnv::default() };

        match &params.self_bound {
            Some(SelfBound::TraitImpl(self_bound)) => {
                let self_var = fresh_var();
                let elements = ImplElements {
                    supertraits: self_bound.supertraits.iter().map(|_| fresh_var()).collect(),
                    where_clauses: self_bound.where_clauses.iter().map(|_| fresh_var()).collect(),
                };
                let element_pats = (elements.supertraits.iter())
                    .chain(&elements.where_clauses)
                    .map(|&var| afb.pat_var(span, var));
                let pat = self.impl_witness_pat(self_bound.impl_def_id, span, element_pats);

                env.self_witness = Some(self_var);
                env.impl_elements = Some(elements);
                env.push_param(afb, self_var, Some(pat));
            }

            // A default body serves every impl, so it does not match a tag.
            Some(SelfBound::Trait(_)) => {
                let self_var = fresh_var();
                env.self_witness = Some(self_var);
                env.push_param(afb, self_var, None);
            }

            None => {}
        }

        if !params.bounds.is_empty() {
            env.bounds = params.bounds.iter().map(|_| fresh_var()).collect();
            let pat = afb.pat_tuple(span, env.bounds.iter().map(|&var| afb.pat_var(span, var)));
            env.push_param(afb, fresh_var(), Some(pat));
        }

        env
    }

    pub(super) fn translate_witness_args(
        &self,
        span: Span,
        witnesses: &WitnessArgs<'tcx>,
        env: &WitnessEnv<'arena>,
        scopes: &mut Scopes,
    ) -> SmallVec<[eaf::Expr<'arena>; 8]> {
        let mut exprs = SmallVec::new();
        if let Some(self_witness) = &witnesses.self_witness {
            exprs.push(self.translate_witness(span, self_witness, env, scopes));
        }
        if !witnesses.bounds.is_empty() {
            let bounds = witnesses
                .bounds
                .iter()
                .map(|witness| self.translate_witness(span, witness, env, scopes));

            exprs.push(self.afb.expr_tuple(span, bounds));
        }
        exprs
    }

    pub(super) fn translate_witness(
        &self,
        span: Span,
        witness: &WitnessSource<'tcx>,
        env: &WitnessEnv<'arena>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx;
        let afb = &self.afb;
        match witness {
            WitnessSource::Param(param) => afb.expr_var(env.span, env.var_name_for_param(*param)),

            WitnessSource::Supertrait(of, index) => {
                if let WitnessSource::Param(ParamBound::InSelf) = **of
                    && let Some(elements) = &env.impl_elements
                {
                    return afb.expr_var(env.span, elements.supertraits[*index]);
                }
                let of_expr = self.translate_witness(span, of, env, scopes);
                let index = FIRST_ELEMENT_INDEX + index.as_u32();
                let span = env.span_of(span, of);
                afb.erlang_bif(span, sym::element, [afb.expr_int(span, index.into()), of_expr])
            }

            WitnessSource::Impl { impl_def_id, supertraits, where_clauses, .. } => {
                let Some(host) = tcx.erl_definition_module(*impl_def_id) else {
                    span_bug!(span, "trait impl {impl_def_id:?} has no host module");
                };

                let elements = (supertraits.iter())
                    .chain(where_clauses)
                    .map(|element| self.translate_witness(span, element, env, scopes));

                let head = [
                    afb.expr_atom(span, tcx.erl_impl_tag(*impl_def_id)),
                    afb.expr_atom(span, tcx.erl_module_name(host)),
                ];

                afb.expr_tuple(span, head.into_iter().chain(elements))
            }

            // The impl's function for the associated type returns the witnesses
            // of the type's bounds.
            WitnessSource::AssocTypeBound { assoc_def_id, witness_args, index, .. } => {
                let mut args =
                    self.translate_witness_args(span, witness_args, env, scopes).into_iter();
                let of =
                    args.next().unwrap_or_else(|| bug!("associated type without a self witness"));
                let name = tcx.erl_fn_name(*assoc_def_id);
                let bounds = self.dispatch_call(span, name, of, args, scopes);
                let index = afb.expr_int(span, (index.as_u32() + 1).into());
                afb.erlang_bif(span, sym::element, [index, bounds])
            }
        }
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

        let host_index = afb.expr_int(span, HOST_INDEX.into());
        let module = afb.erlang_bif(span, sym::element, [host_index, receiver_witness]);
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
