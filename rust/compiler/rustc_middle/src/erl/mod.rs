//! Erlang call resolution and module placement.

pub mod binary_pattern;
pub mod shape;

use std::hash::Hash as _;

use convert_case::{Case, Casing};
use rustc_abi::{FIRST_VARIANT, FieldIdx};
use rustc_data_structures::fx::FxIndexSet;
use rustc_data_structures::stable_hasher::StableHasher;
use rustc_hashes::Hash128;
use rustc_hir::attrs::{AttributeKind, ErlTagStyle, ReprAttr, ReprAttrModifier};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId, LocalDefId};
use rustc_hir::definitions::DefPathData;
use rustc_hir::{Attribute, HirId, PrimTy};
use rustc_macros::StableHash;
use rustc_span::{Span, Symbol, kw};
use smallvec::SmallVec;

use self::shape::ErlShape;
use crate::query::Providers;
use crate::traits::ImplSource;
use crate::ty::print::PrintTraitRefExt as _;
use crate::ty::{
    self, AdtDef, FieldDef, GenericArgsRef, Ty, TyCtxt, TypeFoldable, TypeVisitableExt, TypingEnv,
    Unnormalized, VariantDef, Visibility,
};

/// The identity of a module in the generated Erlang program.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, StableHash)]
pub enum ErlModuleId {
    /// A module generated for a Rust `mod`.
    Mod(DefId),
    /// A module generated for a `Struct` or `Enum`.
    Adt(DefId),
    /// A module named by an `extern mod`.
    Extern(DefId),
    /// The runtime module implementing a primitive type.
    Prim(PrimTy),
    /// The trait impls of a crate on an ADT of another crate.
    AdtImplHost(CrateNum, DefId),
    /// The trait impls of a crate other than Core on a primitive type.
    PrimImplHost(CrateNum, PrimTy),
    /// Impls without a host type land here, e.g. `impl<T: Show> Display for T`.
    BlanketImplHost(CrateNum),
}

/// The semantic target of an Erlang call.
#[derive(Copy, Clone, Debug)]
pub enum ErlCallTarget<'tcx> {
    /// A free function, inherent method, trait method, or trait associated const.
    Item { def_id: DefId, args: GenericArgsRef<'tcx> },
    /// Construction of a struct or enum variant.
    Constructor { nominal_type: DefId, variant: DefId },
    /// A function named by an `extern mod`.
    Extern { module: DefId, function: DefId },
    /// A function value or genuinely dynamic dispatch.
    Dynamic,
}

/// The result of resolving which Erlang module contains a caller or callee.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErlModuleResolution {
    Static(ErlModuleId),
    Dynamic,
}

/// The kind of Erlang call we can make to a given target, from a given caller.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErlCallKind {
    /// The callee is a private function in the caller's module.
    /// Only `priv_fn()` call is possible.
    Local,
    /// The callee is a public function in a different module.
    /// Only `Mod:pub_fn()` call is possible.
    Extern,
    /// The callee is a public function in the caller's module.
    /// Either `Mod:pub_fn()` or `pub_fn()` call are possible.
    LocalOrExtern,
    /// We're not calling a module.
    Dynamic,
}

impl ErlCallKind {
    pub fn can_be_local(self) -> bool {
        matches!(self, ErlCallKind::Local | ErlCallKind::LocalOrExtern)
    }

    pub fn can_be_extern(self) -> bool {
        matches!(self, ErlCallKind::Extern | ErlCallKind::LocalOrExtern)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, StableHash)]
pub enum ErlReprKind {
    Elixir,
    Tagged,
    Atom,
    Int,
    NativeRecord,
    Transparent {
        /// Whether the payload's identity-instantiated shape is [`ErlShape::Any`].
        /// In that case, it may need a wrapper to distinguish it from its siblings.
        open_payload: bool,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, StableHash)]
pub struct ErlVariantRepr {
    pub kind: ErlReprKind,
    /// The explicit repr attribute which selected `kind` (including an inherited enum attribute).
    pub repr_attribute: Option<(ReprAttr, Span)>,
    pub tag: Symbol,
    /// The module of the native record to use for this variant.
    /// This attribute is set even if the variant is not represented as a native record.
    pub record_module: Symbol,
}

#[derive(Copy, Clone)]
enum ErlTag {
    Explicit(Symbol),
    Style(ErlTagStyle),
}

impl<'tcx> TyCtxt<'tcx> {
    /// Determines the generated module in which a Rust definition is emitted.
    pub fn erl_definition_module(self, def_id: impl Into<DefId>) -> Option<ErlModuleId> {
        let mut def_id = def_id.into();

        loop {
            if self.is_erl_nested_item(def_id) {
                return Some(ErlModuleId::Mod(self.lexical_erl_module(def_id)));
            }

            match self.def_kind(def_id) {
                DefKind::Mod => return Some(ErlModuleId::Mod(def_id)),

                DefKind::Struct | DefKind::Enum | DefKind::Union => {
                    return Some(ErlModuleId::Adt(def_id));
                }

                DefKind::ExternMod => return Some(ErlModuleId::Extern(def_id)),

                DefKind::Impl { .. } => {
                    let self_ty = self.type_of(def_id).instantiate_identity().skip_normalization();
                    let module = match self
                        .resolve_erl_receiver_module(self_ty, self.erl_typing_env(def_id))
                    {
                        Some(ErlModuleResolution::Static(module)) => Some(module),
                        Some(ErlModuleResolution::Dynamic) | None => None,
                    };

                    if self.impl_opt_trait_id(def_id).is_none() {
                        return module;
                    }

                    let Some(module) = module else {
                        return Some(ErlModuleId::BlanketImplHost(def_id.krate));
                    };

                    return Some(match module {
                        ErlModuleId::Adt(adt) if adt.krate != def_id.krate => {
                            ErlModuleId::AdtImplHost(def_id.krate, adt)
                        }
                        ErlModuleId::Prim(prim) if !self.is_erl_core_crate(def_id.krate) => {
                            ErlModuleId::PrimImplHost(def_id.krate, prim)
                        }
                        ErlModuleId::Adt(_) | ErlModuleId::Prim(_) => module,
                        ErlModuleId::Mod(_)
                        | ErlModuleId::Extern(_)
                        | ErlModuleId::AdtImplHost(..)
                        | ErlModuleId::PrimImplHost(..)
                        | ErlModuleId::BlanketImplHost(_) => {
                            bug!("self type of {def_id:?} resolves to module {module:?}")
                        }
                    });
                }

                _ => def_id = self.parent(def_id),
            }
        }
    }

    pub fn is_erl_phantom_field(self, field_did: impl Into<DefId>) -> bool {
        self.type_of(field_did.into()).instantiate_identity().skip_normalization().is_phantom_data()
    }

    pub fn erl_runtime_fields<'a>(
        self,
        variant: &'a VariantDef,
    ) -> impl Iterator<Item = (FieldIdx, &'a FieldDef)> {
        variant
            .fields
            .iter_enumerated()
            .filter(move |(_, field)| !self.is_erl_phantom_field(field.did))
    }

    pub fn erl_runtime_field_count(self, variant: &VariantDef) -> usize {
        self.erl_runtime_fields(variant).count()
    }

    pub fn erl_single_runtime_field<'a>(
        self,
        variant: &'a VariantDef,
    ) -> Option<(FieldIdx, &'a FieldDef)> {
        let mut fields = self.erl_runtime_fields(variant);
        let field = fields.next()?;
        fields.next().is_none().then_some(field)
    }

    pub fn erl_runtime_field_position(self, variant: &VariantDef, idx: FieldIdx) -> Option<usize> {
        self.erl_runtime_fields(variant).position(|(field_idx, _)| field_idx == idx)
    }

    /// Whether `def_id` has an Erlang function name (`erl_fn_name`): the items that become
    /// Erlang functions.
    pub fn erl_has_fn_name(self, def_id: DefId) -> bool {
        matches!(
            self.def_kind(def_id),
            DefKind::Fn | DefKind::AssocFn | DefKind::AssocConst { .. } | DefKind::AssocTy
        ) && !self.is_impl_trait_in_trait(def_id)
    }

    /// Whether `krate` is Core. Core is the crate that defines the lang items.
    pub fn is_erl_core_crate(self, krate: CrateNum) -> bool {
        self.lang_items().sized_trait().is_some_and(|sized| sized.krate == krate)
    }

    /// Whether `def_id` is a `fn` or `trait` item declared inside a body, like another function's.
    pub fn is_erl_nested_item(self, def_id: DefId) -> bool {
        matches!(self.def_kind(def_id), DefKind::Fn | DefKind::Trait)
            && !matches!(
                self.def_kind(self.parent(def_id)),
                DefKind::Mod | DefKind::ExternMod | DefKind::ForeignMod
            )
    }

    /// The trait `def_id` is a method of, for a trait method or a trait impl item.
    pub fn erl_entry_trait(self, def_id: DefId) -> Option<DefId> {
        if let Some(trait_def_id) = self.trait_of_assoc(def_id) {
            return Some(trait_def_id);
        }
        let impl_def_id = self.trait_impl_of_assoc(def_id)?;
        self.impl_opt_trait_id(impl_def_id)
    }

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

    /// The generic arguments of the trait item that the trait impl item `def_id` implements, in
    /// `def_id`'s generics: the trait arguments of the impl, then the item's own parameters.
    pub fn erl_trait_item_args_for_impl_item(self, def_id: DefId) -> GenericArgsRef<'tcx> {
        let impl_def_id = self.parent(def_id);
        let trait_ref = self
            .impl_opt_trait_ref(impl_def_id)
            .unwrap_or_else(|| bug!("{def_id:?} is not in a trait impl"))
            .instantiate_identity()
            .skip_normalization();
        ty::GenericArgs::identity_for_item(self, def_id).rebase_onto(
            self,
            impl_def_id,
            trait_ref.args,
        )
    }

    /// The generic arguments of the self type of the inherent impl `impl_def_id`, as in
    /// `erl_impl_tag`. `None` if the arguments are all parameters, like `impl<T> S<T>` or
    /// `impl<T> S<T, T>`: any two such impls overlap, so their method names need no tag.
    fn erl_inherent_impl_tag(self, impl_def_id: DefId) -> Option<String> {
        let self_ty = self.type_of(impl_def_id);
        let ty::Adt(_, args) = self_ty.instantiate_identity().skip_normalization().kind() else {
            return None;
        };

        let fully_generic = args.iter().all(|arg| match arg.kind() {
            ty::GenericArgKind::Lifetime(_) => true,
            ty::GenericArgKind::Type(ty) => matches!(ty.kind(), ty::Param(_)),
            ty::GenericArgKind::Const(ct) => matches!(ct.kind(), ty::ConstKind::Param(_)),
        });
        if fully_generic {
            return None;
        }

        let self_ty =
            self_ty.instantiate(self, self.erl_tag_args(impl_def_id)).skip_normalization();
        let ty::Adt(_, args) = self_ty.kind() else { unreachable!() };
        Some(self.erl_tag_text(args.iter()))
    }

    /// The identity arguments of `impl_def_id`, with type and const parameters named `_`.
    fn erl_tag_args(self, impl_def_id: DefId) -> GenericArgsRef<'tcx> {
        ty::GenericArgs::for_item(self, impl_def_id, |param, _| match param.kind {
            ty::GenericParamDefKind::Lifetime => bug!("lifetime parameter {param:?}"),
            ty::GenericParamDefKind::Type { .. } => {
                Ty::new_param(self, param.index, kw::Underscore).into()
            }
            ty::GenericParamDefKind::Const { .. } => {
                ty::Const::new_param(self, ty::ParamConst::new(param.index, kw::Underscore)).into()
            }
        })
    }

    /// `args`, separated by `, `.
    fn erl_tag_text(self, args: impl IntoIterator<Item = ty::GenericArg<'tcx>>) -> String {
        with_erl_paths(|| {
            args.into_iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ")
        })
    }

    /// The impl that proves `trait_ref` in `owner`'s environment, if an impl proves it.
    pub fn erl_select_impl(
        self,
        owner: DefId,
        trait_ref: ty::TraitRef<'tcx>,
    ) -> Option<(DefId, GenericArgsRef<'tcx>)> {
        match self.erl_select(self.erl_typing_env(owner), trait_ref)? {
            ImplSource::UserDefined(data) => Some((data.impl_def_id, data.args)),
            ImplSource::Param(_) | ImplSource::Builtin(..) => None,
        }
    }

    /// What proves `trait_ref` in `typing_env`: an impl, a where-clause (`Param`), or a builtin
    /// impl. `None` if nothing proves it or the choice is ambiguous.
    fn erl_select(
        self,
        typing_env: TypingEnv<'tcx>,
        trait_ref: ty::TraitRef<'tcx>,
    ) -> Option<&'tcx ImplSource<'tcx, ()>> {
        let trait_ref = self
            .try_normalize_erasing_regions(typing_env, Unnormalized::new_wip(trait_ref))
            .ok()?;
        self.codegen_select_candidate(typing_env.as_query_input(trait_ref)).ok()
    }

    /// The arity of the Erlang function generated for `def_id`.
    pub fn erl_fn_arity(self, def_id: DefId) -> usize {
        let def_id = self.erl_fn_item(def_id);

        let params = match self.def_kind(def_id) {
            DefKind::Fn | DefKind::AssocFn => {
                self.fn_sig(def_id).skip_binder().inputs().skip_binder().len()
            }
            DefKind::AssocConst { .. } | DefKind::AssocTy => 0,
            kind => bug!("no Erlang function arity for {kind:?}"),
        };

        params
            + usize::from(self.erl_entry_trait(def_id).is_some())
            + usize::from(!self.erl_bounds(def_id).is_empty())
    }

    /// Normalizes `value` in `owner`'s environment. A projection whose self type is a type
    /// parameter, like `<T as Iterator>::Item`, has no impl to resolve to, and stays as it is.
    /// If normalization fails, returns `value` unchanged.
    pub fn erl_normalize<T: TypeFoldable<TyCtxt<'tcx>> + Copy>(self, owner: DefId, value: T) -> T {
        self.try_normalize_erasing_regions(self.erl_typing_env(owner), Unnormalized::new_wip(value))
            .unwrap_or(value)
    }

    pub fn is_forced_extern(self, hir_id: HirId) -> bool {
        self.hir_attrs(hir_id)
            .iter()
            .any(|attr| matches!(attr, Attribute::Parsed(AttributeKind::ExternCall(..))))
    }

    /// The item that identifies the Erlang function generated for `def_id` in its module.
    pub fn erl_fn_item(self, def_id: DefId) -> DefId {
        self.trait_item_of(def_id).unwrap_or(def_id)
    }

    /// Returns whether the function generated for `def_id` must be exported from `module`.
    ///
    /// All `pub(x)` where `x != self` are always exported.
    /// The only complication is with associated functions, as `pub(self)` associated functions
    /// can be called from the parent module of the `impl` block, or from local `impl`s for other
    /// types. In that case, we additionally export those private associated functions that
    /// are actually called externally.
    pub fn is_erl_exported_fn(self, module: ErlModuleId, def_id: DefId) -> bool {
        let def_id = self.erl_fn_item(def_id);

        // In the trait's own module, a trait item is its default body. The delegation stubs
        // of the impls that use the default body call it from other (external) modules and crates.
        if self.trait_of_assoc(def_id).is_some()
            && self.erl_definition_module(def_id) == Some(module)
        {
            return true;
        }

        let visible_outside_rust_module = !self.is_erl_nested_item(def_id)
            && match def_id.as_local() {
                // A foreign item here is a trait item that this crate implements. Other crates
                // can dispatch to its override or delegation stub.
                None => true,
                Some(local_def_id) => match self.visibility(local_def_id) {
                    Visibility::Public => true,
                    Visibility::Restricted(restricted) => {
                        restricted != self.parent_module_from_def_id(local_def_id).to_def_id()
                    }
                },
            };

        visible_outside_rust_module || self.fns_called_externally(()).contains(&(module, def_id))
    }

    /// Returns whether `def_id` is, or is nested in, a constant whose value is inlined at each
    /// use. Such a value can land in any module. Consts of traits and trait impls are
    /// functions instead.
    pub fn is_in_erl_inlined_const(self, def_id: DefId) -> bool {
        let def_id = self.typeck_root_def_id(def_id);
        match self.def_kind(def_id) {
            DefKind::Const { .. } => true,
            DefKind::AssocConst { .. } => self.erl_entry_trait(def_id).is_none(),
            _ => false,
        }
    }

    /// Classifies a call target identified by `caller` and `target`.
    pub fn erl_call_kind(
        self,
        caller: DefId,
        target: ErlCallTarget<'tcx>,
        forced_extern: Option<bool>,
    ) -> Option<ErlCallKind> {
        let callee = self.erl_module_for_target(caller, target)?;

        let kind = match callee {
            ErlModuleResolution::Static(callee) => {
                // An inlined constant's calls land in whichever module uses its value.
                if self.is_in_erl_inlined_const(caller)
                    || callee != self.erl_definition_module(caller)?
                {
                    ErlCallKind::Extern
                } else if self.erl_call_target_is_exported(callee, target) {
                    match forced_extern {
                        Some(true) => ErlCallKind::Extern,
                        Some(false) => ErlCallKind::Local,
                        None => ErlCallKind::LocalOrExtern,
                    }
                } else {
                    ErlCallKind::Local
                }
            }
            ErlModuleResolution::Dynamic => ErlCallKind::Dynamic,
        };

        Some(kind)
    }

    /// Classifies a statically resolved callable definition.
    pub fn erl_call_target_for_def(
        self,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> ErlCallTarget<'tcx> {
        match self.def_kind(def_id) {
            DefKind::Fn | DefKind::AssocFn | DefKind::AssocConst { .. } => {
                ErlCallTarget::Item { def_id, args }
            }

            // Patina models a foreign function as the leaf `ExternMod`;
            // its parent identifies the named foreign module.
            // IOW, in `Elixir::IO::puts` every segment is an `ExternMod`; if we try
            // to call it, the target is `Elixir::IO`.
            DefKind::ExternMod => {
                ErlCallTarget::Extern { module: self.parent(def_id), function: def_id }
            }

            kind => bug!("non-callable definition passed as an Erlang call target: {kind:?}"),
        }
    }

    fn erl_typing_env(self, owner: impl Into<DefId>) -> TypingEnv<'tcx> {
        let owner = owner.into();
        TypingEnv::new(self.param_env(owner), ty::TypingMode::non_body_analysis())
    }

    fn erl_call_target_is_exported(self, module: ErlModuleId, target: ErlCallTarget<'tcx>) -> bool {
        match target {
            ErlCallTarget::Item { def_id, args: _ } => self.is_erl_exported_fn(module, def_id),
            ErlCallTarget::Constructor { variant, .. } => self.is_erl_exported_fn(module, variant),
            ErlCallTarget::Extern { .. } | ErlCallTarget::Dynamic => false,
        }
    }

    pub fn erl_module_for_target(
        self,
        owner: DefId,
        target: ErlCallTarget<'tcx>,
    ) -> Option<ErlModuleResolution> {
        let typing_env = self.erl_typing_env(owner);
        match target {
            ErlCallTarget::Item { def_id, args } => {
                self.erl_module_for_target_item(def_id, args, typing_env)
            }

            ErlCallTarget::Constructor { nominal_type, .. } => {
                Some(ErlModuleResolution::Static(ErlModuleId::Adt(nominal_type)))
            }

            ErlCallTarget::Extern { module, .. } => {
                Some(ErlModuleResolution::Static(ErlModuleId::Extern(module)))
            }

            ErlCallTarget::Dynamic => Some(ErlModuleResolution::Dynamic),
        }
    }

    fn erl_module_for_target_item(
        self,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
        typing_env: TypingEnv<'tcx>,
    ) -> Option<ErlModuleResolution> {
        match self.def_kind(def_id) {
            DefKind::Fn => {
                Some(ErlModuleResolution::Static(ErlModuleId::Mod(self.lexical_erl_module(def_id))))
            }

            DefKind::AssocFn | DefKind::AssocConst { .. } | DefKind::AssocTy => {
                let parent = self.parent(def_id);

                match self.def_kind(parent) {
                    // The host module is not always the module of the self type.
                    DefKind::Impl { .. } if self.impl_opt_trait_id(parent).is_some() => {
                        self.erl_definition_module(parent).map(ErlModuleResolution::Static)
                    }

                    DefKind::Impl { .. } => {
                        let self_ty =
                            self.type_of(parent).instantiate(self, args).skip_normalization();
                        self.resolve_erl_receiver_module(self_ty, typing_env)
                    }

                    DefKind::Trait => {
                        let trait_ref = ty::TraitRef::from_assoc(self, parent, args);
                        match self.erl_select(typing_env, trait_ref) {
                            Some(ImplSource::UserDefined(data)) => self
                                .erl_definition_module(data.impl_def_id)
                                .map(ErlModuleResolution::Static),
                            // Dispatch, also for a concrete self type: in `where Celsius:
                            // From<T>`, `T` picks the impl, and so the host, which can be in
                            // another crate. A failed selection also dispatches, so that
                            // `resolve_witness` reports it.
                            Some(ImplSource::Param(_)) | None => None,
                            Some(ImplSource::Builtin(..)) => {
                                self.resolve_erl_receiver_module(args.type_at(0), typing_env)
                            }
                        }
                    }

                    _ => bug!("associated function has unexpected parent: {parent:?}"),
                }
            }

            kind => bug!("non-callable item passed to erl_module_for_target_item: {kind:?}"),
        }
    }

    /// Finds the module used to dispatch an associated callable on `receiver`.
    fn resolve_erl_receiver_module(
        self,
        receiver: Ty<'tcx>,
        typing_env: TypingEnv<'tcx>,
    ) -> Option<ErlModuleResolution> {
        use ErlModuleResolution::*;

        let ty =
            self.try_normalize_erasing_regions(typing_env, Unnormalized::new_wip(receiver)).ok()?;

        match ty.kind() {
            ty::Adt(def, _) => Some(Static(ErlModuleId::Adt(def.did()))),
            ty::Bool => Some(Static(ErlModuleId::Prim(PrimTy::Bool))),
            ty::Char => Some(Static(ErlModuleId::Prim(PrimTy::Char))),
            ty::Int => Some(Static(ErlModuleId::Prim(PrimTy::Int))),
            ty::Float => Some(Static(ErlModuleId::Prim(PrimTy::Float))),
            ty::Str => Some(Static(ErlModuleId::Prim(PrimTy::Str))),
            ty::Term => Some(Static(ErlModuleId::Prim(PrimTy::Term))),
            ty::Atom => Some(Static(ErlModuleId::Prim(PrimTy::Atom))),
            ty::Bin => Some(Static(ErlModuleId::Prim(PrimTy::Bin))),
            ty::Bits => Some(Static(ErlModuleId::Prim(PrimTy::Bits))),
            ty::Slice(_) => Some(Static(ErlModuleId::Prim(PrimTy::List))),
            ty::Map(..) => Some(Static(ErlModuleId::Prim(PrimTy::Map))),
            ty::Tuple(_) => Some(Static(ErlModuleId::Prim(PrimTy::Tuple))),

            // An alias or parameter which survived normalization has not selected a
            // static dispatch owner. In particular, an opaque does not acquire a
            // module from either its definition or its hidden type.
            ty::Alias(_) | ty::Param(_) => None,

            ty::Dynamic(..) | ty::FnPtr(..) => Some(Dynamic),

            _ => Some(Dynamic),
        }
    }

    fn lexical_erl_module(self, mut def_id: DefId) -> DefId {
        loop {
            if self.def_kind(def_id) == DefKind::Mod {
                return def_id;
            }
            def_id = self.parent(def_id);
        }
    }

    /// Resolves the runtime representation of a struct or enum variant.
    pub fn erl_variant_repr(self, variant_def_id: DefId) -> ErlVariantRepr {
        let (adt_def_id, index) = match self.def_kind(variant_def_id) {
            DefKind::Struct => (variant_def_id, FIRST_VARIANT),
            DefKind::Variant => {
                let adt_def_id = self.parent(variant_def_id);
                let index = self.adt_def(adt_def_id).variant_index_with_id(variant_def_id);
                (adt_def_id, index)
            }
            kind => bug!("expected struct or enum variant, got {kind:?}"),
        };

        self.erl_adt_reprs(adt_def_id)[index.index()]
    }

    fn erl_variant_repr_with_kind(
        self,
        adt: AdtDef<'tcx>,
        variant_def_id: DefId,
        repr_attribute: Option<(ReprAttr, Span)>,
        kind: ErlReprKind,
    ) -> ErlVariantRepr {
        let tag = self.erl_variant_tag(variant_def_id, kind);
        let record_module = self
            .find_erl_record_module_attr(variant_def_id)
            .map(|(module, _)| module)
            .unwrap_or_else(|| self.erl_module_name(ErlModuleId::Adt(adt.did())));
        ErlVariantRepr { kind, repr_attribute, tag, record_module }
    }

    /// Returns the nearest explicit representation attribute, inheriting from an enum when needed.
    /// An absent attribute and an explicit `Patina` both select inference, but remain distinct here.
    pub fn effective_erl_repr_kind_attr(self, def_id: DefId) -> Option<(ReprAttr, Span)> {
        if let Some(attribute) = self.find_erl_repr_kind_attr(def_id) {
            Some(attribute)
        } else if self.def_kind(def_id) == DefKind::Variant {
            self.find_erl_repr_kind_attr(self.parent(def_id))
        } else {
            None
        }
    }

    fn find_erl_repr_kind_attr(self, def_id: DefId) -> Option<(ReprAttr, Span)> {
        let reprs = rustc_hir::find_attr!(self, def_id, Repr { reprs, .. } => reprs)?;
        reprs.first().copied()
    }

    /// Resolves the whole enum: reserve fixed representations first, then assign `transparent`
    /// to remaining variants in source order.
    fn resolve_enum_variant_reprs(self, adt: AdtDef<'tcx>) -> SmallVec<[ErlVariantRepr; 4]> {
        let args = ty::GenericArgs::identity_for_item(self, adt.did());

        let mut decided =
            SmallVec::<[Option<ErlVariantRepr>; 4]>::from_elem(None, adt.variants().len());
        let mut open_payload_variant_decided = false;
        let mut reserved_shapes = SmallVec::<[ErlShape; 4]>::new();

        // Prevent cyclic newtypes from becoming transparent before inspecting any payload shapes.
        for &index in self.recursive_transparent_variants(adt.did()) {
            let variant = &adt.variants()[index];
            let repr_attribute = self.effective_erl_repr_kind_attr(variant.def_id);
            let (repr, shape) =
                self.reserve_erl_variant_repr(adt, variant, repr_attribute, ReprAttr::NativeRecord);
            decided[index.index()] = Some(repr);
            reserved_shapes.push(shape);
        }

        // Decide explicit representations and non-newtypes. They take precedence over inferred
        // newtypes regardless of declaration order.
        for (index, variant) in adt.variants().iter_enumerated() {
            if decided[index.index()].is_some() {
                continue;
            }
            let repr_attribute = self.effective_erl_repr_kind_attr(variant.def_id);

            let repr_attr = match erl_repr_attr(repr_attribute) {
                Some(repr_attr) => repr_attr,
                None => match self.erl_runtime_field_count(variant) {
                    0 => ReprAttr::Atom,
                    1 => continue,
                    _ => ReprAttr::NativeRecord,
                },
            };

            let (repr, reserved_shape) =
                self.reserve_erl_variant_repr(adt, variant, repr_attribute, repr_attr);

            if matches!(repr.kind, ErlReprKind::Transparent { open_payload: true }) {
                open_payload_variant_decided = true;
            }

            decided[index.index()] = Some(repr);
            reserved_shapes.push(reserved_shape);
        }

        // Decide every open newtype before closed newtypes can claim wrapper shapes. At most one
        // can be transparent, and only when its wrapper does not overlap an already fixed shape.
        for (index, variant) in adt.variants().iter_enumerated() {
            if decided[index.index()].is_some() {
                continue;
            }

            let Some((_, field)) = self.erl_single_runtime_field(variant) else {
                bug!("undecided variant with more than one field: {variant:?}")
            };

            if !matches!(self.erl_shape_of_ty(field.ty(self, args)), ErlShape::Any) {
                continue;
            }

            let (repr, reserved_shape) =
                if !open_payload_variant_decided && adt.variants().len() <= 4 {
                    let transparent =
                        self.reserve_erl_variant_repr(adt, variant, None, ReprAttr::Transparent);

                    if reserved_shapes.iter().any(|shape| transparent.1.overlaps(shape)) {
                        self.reserve_erl_variant_repr(adt, variant, None, ReprAttr::NativeRecord)
                    } else {
                        open_payload_variant_decided = true;
                        transparent
                    }
                } else {
                    self.reserve_erl_variant_repr(adt, variant, None, ReprAttr::NativeRecord)
                };

            decided[index.index()] = Some(repr);
            reserved_shapes.push(reserved_shape);
        }

        // Only closed newtypes remain. Assign transparent when their payload shape is still free.
        for (index, variant) in adt.variants().iter_enumerated() {
            if decided[index.index()].is_some() {
                continue;
            }

            let Some((_, field)) = self.erl_single_runtime_field(variant) else {
                bug!("undecided variants must have exactly one field")
            };

            let payload_shape = self.erl_shape_of_ty(field.ty(self, args));
            if matches!(payload_shape, ErlShape::Any) {
                bug!("open payload variant left undecided")
            }
            let overlaps = reserved_shapes.iter().any(|shape| payload_shape.overlaps(shape));
            let repr_attr = if overlaps { ReprAttr::NativeRecord } else { ReprAttr::Transparent };
            let (repr, reserved_shape) =
                self.reserve_erl_variant_repr(adt, variant, None, repr_attr);
            decided[index.index()] = Some(repr);
            reserved_shapes.push(reserved_shape);
        }

        decided.into_iter().map(|repr| repr.unwrap_or_else(|| bug!("undecided variant"))).collect()
    }

    fn reserve_erl_variant_repr(
        self,
        adt: AdtDef<'tcx>,
        variant: &VariantDef,
        repr_attribute: Option<(ReprAttr, Span)>,
        repr_attr: ReprAttr,
    ) -> (ErlVariantRepr, ErlShape) {
        let kind = match repr_attr {
            ReprAttr::Elixir => ErlReprKind::Elixir,
            ReprAttr::Tagged => ErlReprKind::Tagged,
            ReprAttr::Atom => ErlReprKind::Atom,
            ReprAttr::ReprInt => ErlReprKind::Int,
            ReprAttr::NativeRecord => ErlReprKind::NativeRecord,
            ReprAttr::Transparent => {
                let open_payload = if let Some((_, field)) = self.erl_single_runtime_field(variant)
                {
                    let args = ty::GenericArgs::identity_for_item(self, adt.did());
                    matches!(self.erl_shape_of_ty(field.ty(self, args)), ErlShape::Any)
                } else {
                    false
                };
                ErlReprKind::Transparent { open_payload }
            }
            ReprAttr::Patina => bug!("unmapped Patina repr attribute"),
        };

        let repr = self.erl_variant_repr_with_kind(adt, variant.def_id, repr_attribute, kind);
        let reserved_shape = self.erl_variant_reserved_shape(adt, variant, repr);
        (repr, reserved_shape)
    }

    pub fn find_erl_record_module_attr(self, def_id: DefId) -> Option<(Symbol, Span)> {
        let modifiers = rustc_hir::find_attr!(self, def_id, Repr { modifiers, .. } => modifiers)?;

        modifiers.iter().find_map(|(modifier, span)| match modifier {
            ReprAttrModifier::RecordModule(module) => Some((*module, *span)),
            _ => None,
        })
    }

    pub fn find_erl_dynamic_wrap_attr(self, def_id: DefId) -> Option<Span> {
        let modifiers = rustc_hir::find_attr!(self, def_id, Repr { modifiers, .. } => modifiers)?;

        modifiers.iter().find_map(|(modifier, span)| match modifier {
            ReprAttrModifier::DynamicWrap => Some(*span),
            _ => None,
        })
    }

    /// Returns the atom used to tag a struct or enum variant.
    pub fn erl_variant_tag(self, variant_def_id: DefId, repr: ErlReprKind) -> Symbol {
        assert!(matches!(self.def_kind(variant_def_id), DefKind::Struct | DefKind::Variant));

        let own_tag = self.find_erl_tag_choice(variant_def_id);
        let inherited_tag = (self.def_kind(variant_def_id) == DefKind::Variant)
            .then(|| self.find_erl_tag_choice(self.parent(variant_def_id)))
            .flatten();

        match own_tag.or(inherited_tag) {
            Some(ErlTag::Explicit(tag)) => tag,
            Some(ErlTag::Style(ErlTagStyle::SnakeCase)) => {
                Symbol::intern(&self.item_name(variant_def_id).as_str().to_case(Case::Snake))
            }
            None if repr == ErlReprKind::NativeRecord => self.item_name(variant_def_id),
            None => match self.def_kind(variant_def_id) {
                DefKind::Struct => self.erl_module_name(ErlModuleId::Adt(variant_def_id)),
                DefKind::Variant => {
                    let enum_module =
                        self.erl_module_name(ErlModuleId::Adt(self.parent(variant_def_id)));
                    Symbol::intern(&format!("{enum_module}.{}", self.item_name(variant_def_id)))
                }
                _ => unreachable!(),
            },
        }
    }

    fn find_erl_tag_choice(self, def_id: DefId) -> Option<ErlTag> {
        let modifiers = rustc_hir::find_attr!(self, def_id, Repr { modifiers, .. } => modifiers)?;

        if let Some(tag) = modifiers.iter().find_map(|(modifier, _)| match modifier {
            ReprAttrModifier::Tag(tag) => Some(*tag),
            _ => None,
        }) {
            return Some(ErlTag::Explicit(tag));
        }

        modifiers.iter().find_map(|(modifier, _)| match modifier {
            ReprAttrModifier::TagStyle(style) => Some(ErlTag::Style(*style)),
            _ => None,
        })
    }
}

/// Provider for [`TyCtxt::erl_adt_reprs`].
fn erl_adt_reprs<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> &'tcx [ErlVariantRepr] {
    let adt = tcx.adt_def(def_id);

    if adt.is_enum() {
        let decided = tcx.resolve_enum_variant_reprs(adt);
        return tcx.arena.alloc_from_iter(decided);
    }

    let variant = adt.non_enum_variant();
    let repr_attribute = tcx.effective_erl_repr_kind_attr(variant.def_id);
    let repr_attr = if tcx.recursive_transparent_variants(def_id).contains(&FIRST_VARIANT) {
        ReprAttr::NativeRecord
    } else {
        erl_repr_attr(repr_attribute).unwrap_or_else(|| struct_repr_from_arity(tcx, variant))
    };

    let (repr, _reserved_shape) =
        tcx.reserve_erl_variant_repr(adt, variant, repr_attribute, repr_attr);
    tcx.arena.alloc_from_iter([repr])
}

/// Provider for [`TyCtxt::erl_fn_name`].
fn erl_fn_name(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Symbol {
    let def_id = def_id.to_def_id();
    if !tcx.erl_has_fn_name(def_id) {
        bug!("`{}` has no Erlang function name", tcx.def_path_str(def_id));
    }
    let item_name = tcx.item_name(def_id);

    if tcx.is_erl_nested_item(def_id) {
        let module_depth = tcx.def_path(tcx.lexical_erl_module(def_id)).data.len();
        let path = tcx.def_path(def_id);
        let segments =
            path.data[module_depth..].iter().map(|segment| segment.as_sym(true).to_string());
        return Symbol::intern(&erl_fit_atom(segments.collect::<Vec<_>>().join("::")));
    }

    let Some(trait_def_id) = tcx.erl_entry_trait(def_id) else {
        // Inherent impls on different arguments of one type can have methods of the same name,
        // like `impl S<int> { fn f() }` and `impl S<str> { fn f() }`. We disambiguate by including
        // the generic arguments in the functtion name.
        if let Some(parent) = tcx.opt_parent(def_id)
            && matches!(tcx.def_kind(parent), DefKind::Impl { .. })
            && let Some(tag) = tcx.erl_inherent_impl_tag(parent)
        {
            return Symbol::intern(&erl_fit_atom(format!("{item_name}<{tag}>")));
        }
        return item_name;
    };
    let mut segments = erl_def_path_segments(tcx, trait_def_id);

    match tcx.def_kind(def_id) {
        DefKind::AssocTy => segments.push("type".to_owned()),
        DefKind::AssocConst { .. } => segments.push("const".to_owned()),
        _ => {}
    }

    segments.push(item_name.as_str().to_owned());
    Symbol::intern(&segments.join("::"))
}

/// Prints paths from the crate name, so that every crate prints the same text.
fn with_erl_paths<R>(print: impl FnOnce() -> R) -> R {
    ty::print::with_resolve_crate_name!(ty::print::with_no_visible_paths!(
        ty::print::with_no_trimmed_paths!(print())
    ))
}

/// `text`, shortened with a hash suffix if it is longer than the longest atom.
pub fn erl_fit_atom(mut text: String) -> String {
    const MAX_ATOM_CHARS: usize = 255;
    if text.chars().count() <= MAX_ATOM_CHARS {
        return text;
    }

    let mut hasher = StableHasher::new();
    text.hash(&mut hasher);
    let hash: Hash128 = hasher.finish();
    let suffix = format!("#{:016x}", hash.as_u128() as u64);
    let (end, _) = text.char_indices().nth(MAX_ATOM_CHARS - suffix.len()).unwrap();
    text.truncate(end);
    text + &suffix
}

/// The crate name followed by the definition path segments of `def_id`.
fn erl_def_path_segments(tcx: TyCtxt<'_>, def_id: DefId) -> SmallVec<[String; 6]> {
    let path = tcx.def_path(def_id);

    let mut segments =
        SmallVec::<[String; 6]>::from_iter([tcx.crate_name(path.krate).as_str().to_owned()]);

    for data in &path.data {
        if data.data != DefPathData::Ctor {
            segments.push(data.as_sym(true).as_str().to_owned());
        }
    }

    segments
}

/// Provider for [`TyCtxt::erl_module_name`].
fn erl_module_name(tcx: TyCtxt<'_>, module: ErlModuleId) -> Symbol {
    match module {
        ErlModuleId::Mod(def_id) | ErlModuleId::Adt(def_id) => {
            if let Some(name) = rustc_hir::find_attr!(tcx, def_id, ErlModule(name, _) => *name) {
                return name;
            }

            let segments = erl_def_path_segments(tcx, def_id);
            Symbol::intern(&format!("Elixir.Patina.{}", segments.join(".")))
        }
        ErlModuleId::Extern(mut def_id) => {
            let mut segments = SmallVec::<[String; 6]>::new();

            while tcx.def_kind(def_id) == DefKind::ExternMod {
                segments.push(tcx.item_name(def_id).as_str().to_owned());
                def_id = tcx.parent(def_id);
            }

            segments.reverse();
            Symbol::intern(&segments.join("."))
        }
        ErlModuleId::Prim(prim_ty) => {
            // The primitive modules are capitalized to be easy to call from Elixir.
            let name = prim_ty.name_str().to_case(Case::Pascal);
            Symbol::intern(&format!("Elixir.Patina.Core.{name}"))
        }
        ErlModuleId::AdtImplHost(krate, adt) => {
            erl_impl_host_name(tcx, krate, tcx.erl_module_name(ErlModuleId::Adt(adt)))
        }
        ErlModuleId::PrimImplHost(krate, prim_ty) => {
            erl_impl_host_name(tcx, krate, tcx.erl_module_name(ErlModuleId::Prim(prim_ty)))
        }
        // `Elixir.Patina.<Crate>.$impl`
        ErlModuleId::BlanketImplHost(krate) => {
            Symbol::intern(&format!("Elixir.Patina.{}.$impl", tcx.crate_name(krate)))
        }
    }
}

/// `Elixir.Patina.<Crate>.$impl.<SelfTypePath>`
fn erl_impl_host_name(tcx: TyCtxt<'_>, krate: CrateNum, self_module: Symbol) -> Symbol {
    let self_module = self_module.as_str();
    let self_path = self_module.strip_prefix("Elixir.Patina.").unwrap_or(self_module);
    Symbol::intern(&format!("Elixir.Patina.{}.$impl.{self_path}", tcx.crate_name(krate)))
}

fn erl_repr_attr(attribute: Option<(ReprAttr, Span)>) -> Option<ReprAttr> {
    match attribute.map(|(attribute, _)| attribute) {
        None | Some(ReprAttr::Patina) => None,
        Some(attribute) => Some(attribute),
    }
}

fn struct_repr_from_arity(tcx: TyCtxt<'_>, variant: &VariantDef) -> ReprAttr {
    match tcx.erl_runtime_field_count(variant) {
        0 => ReprAttr::Atom,
        1 => ReprAttr::Transparent,
        _ => ReprAttr::NativeRecord,
    }
}

/// Provider for [`TyCtxt::erl_trait_supertraits`].
fn erl_trait_supertraits<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
) -> &'tcx [ty::TraitRef<'tcx>] {
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
    tcx.arena.alloc_from_iter(supertraits)
}

/// Provider for [`TyCtxt::erl_assoc_type_bounds`].
fn erl_assoc_type_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    assoc_def_id: DefId,
) -> &'tcx [ty::TraitRef<'tcx>] {
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
    tcx.arena.alloc_from_iter(bounds)
}

/// Provider for [`TyCtxt::erl_bounds`].
fn erl_bounds<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> &'tcx [ty::TraitRef<'tcx>] {
    if let Some(trait_def_id) = tcx.trait_of_assoc(def_id) {
        let self_trait_ref = ty::TraitRef::identity(tcx, trait_def_id);
        let supertraits = tcx.erl_trait_supertraits(trait_def_id);
        let clauses = tcx
            .predicates_of(trait_def_id)
            .predicates
            .iter()
            .chain(tcx.predicates_of(def_id).predicates);
        let bounds = tcx
            .erl_witnessed_trait_refs(clauses)
            .into_iter()
            .filter(|trait_ref| *trait_ref != self_trait_ref && !supertraits.contains(trait_ref));
        return tcx.arena.alloc_from_iter(bounds);
    }

    if let Some(trait_item_def_id) = tcx.trait_item_of(def_id) {
        let args = tcx.erl_trait_item_args_for_impl_item(def_id);
        return tcx.arena.alloc_from_iter(tcx.erl_bounds(trait_item_def_id).iter().map(|&bound| {
            ty::EarlyBinder::bind(bound).instantiate(tcx, args).skip_normalization()
        }));
    }

    let predicates = tcx.predicates_of(def_id);
    let parent_clauses = predicates
        .parent
        .filter(|&parent| matches!(tcx.def_kind(parent), DefKind::Impl { .. }))
        .into_iter()
        .flat_map(|parent| tcx.predicates_of(parent).predicates);
    tcx.arena
        .alloc_from_iter(tcx.erl_witnessed_trait_refs(parent_clauses.chain(predicates.predicates)))
}

/// Provider for [`TyCtxt::erl_impl_tag`].
fn erl_impl_tag(tcx: TyCtxt<'_>, impl_def_id: LocalDefId) -> Symbol {
    let impl_def_id = impl_def_id.to_def_id();
    let trait_ref = tcx
        .impl_opt_trait_ref(impl_def_id)
        .unwrap_or_else(|| bug!("{impl_def_id:?} is not a trait impl"))
        .instantiate(tcx, tcx.erl_tag_args(impl_def_id))
        .skip_normalization();

    let text = with_erl_paths(|| {
        format!("impl {} for {}", trait_ref.print_only_trait_path(), trait_ref.self_ty())
    });
    Symbol::intern(&erl_fit_atom(text))
}

/// Provider for [`TyCtxt::erl_trait_needs_witness`].
fn erl_trait_needs_witness(tcx: TyCtxt<'_>, trait_def_id: DefId) -> bool {
    // A closure is an Erlang fun, so a call through `Fn*` needs no witness.
    !tcx.is_fn_trait(trait_def_id)
        && tcx.associated_items(trait_def_id).in_definition_order().next().is_some()
}

pub fn provide(providers: &mut Providers) {
    *providers = Providers {
        erl_adt_reprs,
        erl_fn_name,
        erl_trait_needs_witness,
        erl_trait_supertraits,
        erl_assoc_type_bounds,
        erl_bounds,
        erl_impl_tag,
        erl_module_name,
        erl_shape_of_ty: shape::erl_shape_of_ty,
        erl_variant_wrap_guards: shape::erl_variant_wrap_guards,
        ..*providers
    };
}
