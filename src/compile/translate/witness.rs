//! The runtime representation of witnesses.
//!
//! The witness of `Ty: Trait` names the impl that proves the bound:
//! `{Vtable, {S1, …, Sk}, {W1, …, Wn}}`.
//! - `Vtable` holds a fun reference for each of `Trait`'s items in `erl_vtable_items` order:
//!   the impl's item, or the trait's default body.
//!   Dispatch calls `(element(Slot, element(1, W)))(W, …)`.
//! - `S1 … Sk` are the witnesses of `Ty` for the direct supertraits of `Trait`.
//! - `W1 … Wn` are the witnesses of the where-clauses of the impla.
//!
//! The impl's host module builds the witness tuple in `$patina_witness` function.
//! Everything else calls that function, so we can share literals within ERTS.

use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::erl::witness::{
    BoundIdx, ParamBound, SelfBound, SupertraitIdx, WitnessArgs, WitnessParams, WitnessResolver,
    WitnessSource,
};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, kw, sym};

use super::scopes::Scopes;
use super::{Function, Translator};
use crate::compile::eaf_builder::EafBuilder;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

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
    /// Whether impl witnesses are built in place, or whether they're built by calling
    /// the impls $patina_witness function.
    /// $patina_witness functions themselves build in-place, so the full built witness
    /// can possibly be a literal. Other witness users call $patina_witness instead.
    builds_impl_witnesses: bool,
}

/// A witness parameter of a function.
#[derive(Clone, Copy)]
pub(super) struct WitnessParam<'arena> {
    /// The pattern of the parameter.
    pub(super) pat: eaf::Pattern<'arena>,
    /// The variable that holds the whole witness.
    pub(super) var: eaf::Expr<'arena>,
}

/// The 1-based positions of the parts of an impl witness.
const VTABLE_INDEX: u32 = 1;
const SUPERTRAITS_INDEX: u32 = 2;

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
    /// If `pat` is given, e.g. `{_, {S1}, {}}`, the code emits a match
    /// `foo(..., var = {_, {S1}, {}}, ...)` at the parameter's location.
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
    pub(super) fn translate_witness_params(
        &self,
        params: &WitnessParams<'tcx>,
        span: Span,
        scopes: &mut Scopes,
    ) -> WitnessEnv<'arena> {
        let afb = &self.afb;
        let mut env = WitnessEnv { span, ..WitnessEnv::default() };
        let vars_pat =
            |vars: &[Symbol]| afb.pat_tuple(span, vars.iter().map(|&var| afb.pat_var(span, var)));

        match &params.self_bound {
            Some(SelfBound::TraitImpl(self_bound)) => {
                let self_var = scopes.fresh_var(sym::witness_self).erl_name;
                let elements = ImplElements {
                    supertraits: self_bound
                        .supertraits
                        .iter()
                        .map(|_| scopes.fresh_var(sym::witness_supertrait).erl_name)
                        .collect(),
                    where_clauses: self_bound
                        .where_clauses
                        .iter()
                        .map(|_| scopes.fresh_var(sym::witness_where_clause).erl_name)
                        .collect(),
                };
                let pat = afb.pat_tuple(
                    span,
                    [
                        afb.pat_var(span, kw::Underscore),
                        vars_pat(&elements.supertraits.raw),
                        vars_pat(&elements.where_clauses.raw),
                    ],
                );

                env.self_witness = Some(self_var);
                env.impl_elements = Some(elements);
                env.push_param(afb, self_var, Some(pat));
            }

            Some(SelfBound::Trait(_)) => {
                let self_var = scopes.fresh_var(sym::witness_self).erl_name;
                env.self_witness = Some(self_var);
                env.push_param(afb, self_var, None);
            }

            None => {}
        }

        if !params.bounds.is_empty() {
            env.bounds = params
                .bounds
                .iter()
                .map(|_| scopes.fresh_var(sym::witness_bound).erl_name)
                .collect();

            let pat = vars_pat(&env.bounds.raw);
            env.push_param(afb, scopes.fresh_var(sym::witness_bounds).erl_name, Some(pat));
        }

        env
    }

    pub(super) fn translate_witness_args(
        &self,
        span: Span,
        witnesses: &WitnessArgs<'tcx>,
        env: &WitnessEnv,
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
        env: &WitnessEnv,
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
                let span = env.span_of(span, of);
                let supertraits = self.element(span, SUPERTRAITS_INDEX, of_expr);
                self.element(span, index.as_u32() + 1, supertraits)
            }

            WitnessSource::Impl { impl_def_id, supertraits, where_clauses, .. } => {
                let mut translate = |element| self.translate_witness(span, element, env, scopes);
                let bounds: SmallVec<[_; 2]> = where_clauses.iter().map(&mut translate).collect();
                if env.builds_impl_witnesses {
                    let supertraits = afb.expr_tuple(span, supertraits.iter().map(translate));
                    let bounds = afb.expr_tuple(span, bounds);
                    let vtable = self.vtable(span, *impl_def_id);
                    return afb.expr_tuple(span, [vtable, supertraits, bounds]);
                }

                let Some(host) = tcx.erl_definition_module(*impl_def_id) else {
                    span_bug!(span, "trait impl {impl_def_id:?} has no host module");
                };
                let args = (!bounds.is_empty()).then(|| afb.expr_tuple(span, bounds));
                afb.expr_remote_call(
                    span,
                    afb.expr_atom(span, tcx.erl_module_name(host)),
                    afb.expr_atom(span, self.witness_fn_name(*impl_def_id)),
                    args,
                )
            }

            // The impl's function for the associated type returns the witnesses
            // of the type's bounds.
            WitnessSource::AssocTypeBound { assoc_def_id, witness_args, index, .. } => {
                let mut args =
                    self.translate_witness_args(span, witness_args, env, scopes).into_iter();
                let of =
                    args.next().unwrap_or_else(|| bug!("associated type without a self witness"));
                let bounds = self.dispatch_call(span, *assoc_def_id, of, args, scopes);
                self.element(span, index.as_u32() + 1, bounds)
            }
        }
    }

    /// The `$patina_witness` function of the trait impl.
    pub(super) fn translate_witness_fn(&self, impl_def_id: DefId, span: Span) -> Function<'arena> {
        let tcx = self.tcx;
        let afb = &self.afb;
        let mut scopes = Scopes::new();

        let params = tcx.erl_witness_params_identity(impl_def_id);
        let mut env = self.translate_witness_params(&params, span, &mut scopes);
        env.builds_impl_witnesses = true;

        let mut resolver = WitnessResolver::new(tcx, impl_def_id);
        let trait_ref = tcx.impl_trait_ref(impl_def_id).instantiate_identity().skip_normalization();
        let supertraits = tcx
            .erl_trait_supertraits(trait_ref.def_id)
            .iter_instantiated_copied(tcx, trait_ref.args)
            .map(|bound| {
                let source = resolver.witness_source(span, bound);
                self.translate_witness(span, &source, &env, &mut scopes)
            });

        let vtable = self.vtable(span, impl_def_id);
        let supertraits = afb.expr_tuple(span, supertraits);

        let bounds = match env.params.first() {
            Some(param) => param.var,
            None => afb.expr_tuple(span, []),
        };

        let witness = afb.expr_tuple(span, [vtable, supertraits, bounds]);

        Function {
            span,
            name: self.witness_fn_name(impl_def_id),
            clause: afb.clause(span, env.params.iter().map(|param| param.pat), None, witness),
            exported: true,
        }
    }

    /// The vtable of the trait impl is a tuple literal.
    /// For each trait item, it's an external fun pointing either at the impl's item,
    /// or at the default body if the impl doesn't override it.
    fn vtable(&self, span: Span, impl_def_id: DefId) -> eaf::Expr<'arena> {
        let tcx = self.tcx;
        let afb = &self.afb;

        let trait_def_id = tcx
            .impl_opt_trait_id(impl_def_id)
            .unwrap_or_else(|| span_bug!(span, "{impl_def_id:?} is not a trait impl"));

        let implemented = tcx.impl_item_implementor_ids(impl_def_id);

        let funs = tcx.erl_vtable_items(trait_def_id).map(|trait_item| {
            let def_id = match implemented.get(&trait_item.def_id) {
                Some(&impl_item_def_id) => impl_item_def_id,
                None if trait_item.is_type() => {
                    not_implemented!(tcx, span, "associated type default", trait_item.def_id)
                }
                None => trait_item.def_id,
            };
            let module = tcx.erl_definition_module(def_id).unwrap_or_else(|| {
                span_bug!(span, "`{}` has no definition module", tcx.def_path_str(def_id))
            });

            afb.expr_remote_fun(
                span,
                tcx.erl_module_name(module),
                tcx.erl_fn_name(def_id),
                tcx.erl_fn_arity(def_id) as u32,
            )
        });

        afb.expr_tuple(span, funs)
    }

    fn element(&self, span: Span, index: u32, tuple: eaf::Expr<'arena>) -> eaf::Expr<'arena> {
        let afb = &self.afb;
        afb.erlang_bif(span, sym::element, [afb.expr_int(span, index.into()), tuple])
    }

    /// Call the trait item through the vtable of `receiver_witness`:
    /// `(element(Slot, element(VTABLE_INDEX, W)))(W, Args…)`.
    pub(super) fn dispatch_call(
        &self,
        span: Span,
        trait_item: DefId,
        receiver_witness: eaf::Expr<'arena>,
        args: impl IntoIterator<Item = eaf::Expr<'arena>>,
        scopes: &mut Scopes,
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx;
        let afb = &self.afb;

        let slot = tcx
            .erl_vtable_items(tcx.parent(trait_item))
            .position(|item| item.def_id == trait_item)
            .unwrap_or_else(|| {
                span_bug!(span, "`{}` has no vtable slot", tcx.def_path_str(trait_item))
            });

        let (binding, receiver_witness) = match receiver_witness {
            eaf::Expr::Var(..) => (None, receiver_witness),
            _ => {
                let var = scopes.fresh_var(sym::witness).erl_name;
                let binding = afb.expr_match(span, afb.pat_var(span, var), receiver_witness);
                (Some(binding), afb.expr_var(span, var))
            }
        };

        let vtable = self.element(span, VTABLE_INDEX, receiver_witness);
        let fun = self.element(span, slot as u32 + 1, vtable);
        let call = afb.expr_local_call(span, fun, [receiver_witness].into_iter().chain(args));

        match binding {
            Some(binding) => afb.expr_block(span, [binding, call]),
            None => call,
        }
    }

    pub fn witness_fn_name(&self, impl_def_id: DefId) -> Symbol {
        self.tcx.erl_impl_member_name(impl_def_id, "$patina_witness")
    }
}
