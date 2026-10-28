//! Erlang call resolution and module placement.

pub mod binary_pattern;
pub mod etir;
pub mod shape;
pub mod witness;

use std::hash::Hash as _;

use convert_case::{Case, Casing};
use rustc_abi::{FIRST_VARIANT, FieldIdx};
use rustc_data_structures::stable_hasher::StableHasher;
use rustc_hashes::Hash128;
use rustc_hir::attrs::{AttributeKind, ErlTagStyle, ReprAttr, ReprAttrModifier};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId, LocalDefId};
use rustc_hir::definitions::DefPathData;
use rustc_hir::{Attribute, HirId, PrimTy};
use rustc_macros::{StableHash, TyDecodable, TyEncodable};
use rustc_span::{Span, Symbol, kw};
use smallvec::{SmallVec, smallvec};

use self::shape::ErlShape;
use crate::query::Providers;
use crate::ty::print::PrintTraitRefExt as _;
use crate::ty::{
    self, AdtDef, FieldDef, GenericArgsRef, Ty, TyCtxt, TypingEnv, Unnormalized, VariantDef,
    Visibility,
};

/// The identity of a module in the generated Erlang program.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, StableHash, TyEncodable, TyDecodable)]
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
#[derive(Copy, Clone, Debug, TyEncodable, TyDecodable)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, TyEncodable, TyDecodable)]
pub enum ErlModuleResolution {
    Static(ErlModuleId),
    Dynamic,
}

/// The kind of Erlang call we make to a given target, from a given caller.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErlCallKind {
    /// The callee is in the caller's module: `fun()`.
    Local,
    /// The callee is in a different module, or the call is forced extern: `Mod:fun()`.
    Extern,
    /// We're not calling a module.
    Dynamic,
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
                    let module = match self.resolve_erl_receiver_module(
                        self_ty,
                        TypingEnv::post_analysis(self, def_id),
                    ) {
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

    pub fn erl_runtime_fields<'a>(
        self,
        variant: &'a VariantDef,
    ) -> impl Iterator<Item = (FieldIdx, &'a FieldDef)> {
        variant.fields.iter_enumerated().filter(move |(_, field)| {
            !self.type_of(field.did).instantiate_identity().skip_normalization().is_phantom_data()
        })
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

    /// Returns the trait of a trait method or trait impl item.
    pub fn erl_entry_trait(self, def_id: DefId) -> Option<DefId> {
        if let Some(trait_def_id) = self.trait_of_assoc(def_id) {
            return Some(trait_def_id);
        }
        let impl_def_id = self.trait_impl_of_assoc(def_id)?;
        self.impl_opt_trait_id(impl_def_id)
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

    /// The arity of the Erlang function generated for `def_id`.
    pub fn erl_fn_arity(self, def_id: DefId) -> usize {
        let params = match self.def_kind(def_id) {
            DefKind::Fn | DefKind::AssocFn => {
                self.fn_sig(def_id).skip_binder().inputs().skip_binder().len()
            }
            DefKind::AssocConst { .. } | DefKind::AssocTy => 0,
            kind => bug!("no Erlang function arity for {kind:?}"),
        };

        params
            + usize::from(self.erl_entry_trait(def_id).is_some())
            + usize::from(!self.erl_bounds(def_id).skip_binder().is_empty())
    }

    pub fn is_forced_extern(self, hir_id: HirId) -> bool {
        self.hir_attrs(hir_id)
            .iter()
            .any(|attr| matches!(attr, Attribute::Parsed(AttributeKind::ExternCall(..))))
    }

    /// Returns whether the function generated for `def_id` must be exported from `module`.
    ///
    /// All `pub(x)` where `x != self` are always exported.
    /// The only complication is with associated functions, as `pub(self)` associated functions
    /// can be called from the parent module of the `impl` block, or from local `impl`s for other
    /// types. In that case, we additionally export those private associated functions that
    /// are actually called externally.
    pub fn is_erl_exported_fn(self, module: ErlModuleId, def_id: DefId) -> bool {
        // We use external funs for trait vtables, so all of their fns are exported.
        if self.erl_entry_trait(def_id).is_some() {
            return true;
        }

        let visible_outside_rust_module = !self.is_erl_nested_item(def_id)
            && match def_id.as_local() {
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

    /// Does `caller`->`callee` require an extern call?
    /// `None` when the `caller` has no module at all.
    pub fn is_erl_cross_module_call(self, caller: DefId, callee: ErlModuleId) -> Option<bool> {
        // An inlined constant's calls land in whichever module uses its value.
        Some(self.is_in_erl_inlined_const(caller) || callee != self.erl_definition_module(caller)?)
    }

    /// Classifies a call from `caller` to a function of `callee_module`.
    pub fn erl_call_kind(
        self,
        caller: DefId,
        callee_module: ErlModuleResolution,
        forced_extern: bool,
    ) -> Option<ErlCallKind> {
        let kind = match callee_module {
            ErlModuleResolution::Static(callee) => {
                if forced_extern || self.is_erl_cross_module_call(caller, callee)? {
                    ErlCallKind::Extern
                } else {
                    ErlCallKind::Local
                }
            }
            ErlModuleResolution::Dynamic => ErlCallKind::Dynamic,
        };

        Some(kind)
    }

    /// Classifies a callable definition, leaving a trait item unresolved.
    pub fn erl_call_target(self, def_id: DefId, args: GenericArgsRef<'tcx>) -> ErlCallTarget<'tcx> {
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

    /// Resolves a trait item to the impl item which `caller` calls, if it's known statically.
    pub fn erl_resolve_trait_item_target(
        self,
        caller: DefId,
        target: ErlCallTarget<'tcx>,
    ) -> ErlCallTarget<'tcx> {
        let ErlCallTarget::Item { def_id, args } = target else { return target };
        if self.trait_of_assoc(def_id).is_none() {
            return target;
        }

        let typing_env = TypingEnv::post_analysis(self, caller);
        match ty::Instance::try_resolve(self, typing_env, def_id, args) {
            Ok(Some(ty::Instance { def: ty::InstanceKind::Item(def_id), args })) => {
                ErlCallTarget::Item { def_id, args }
            }
            _ => target,
        }
    }

    /// The name of the Erlang function for `member` of the impl: `[TraitPath::]member[<SelfTag>]`.
    pub fn erl_impl_member_name(self, impl_def_id: DefId, member: &str) -> Symbol {
        let mut name = String::new();
        if let Some(trait_ref) = self.impl_opt_trait_ref(impl_def_id) {
            let trait_ref =
                trait_ref.instantiate(self, self.erl_tag_args(impl_def_id)).skip_normalization();

            name.push_str(&with_erl_paths(|| trait_ref.print_only_trait_path().to_string()));
            name.push_str("::");
        }
        name.push_str(member);

        if let Some(self_tag) = erl_impl_self_tag(self, impl_def_id) {
            name.push_str(&format!("<{self_tag}>"));
        }
        to_atom_symbol(name)
    }

    /// The trait items that comprise a trait impl's vtable.
    /// The order of items returned here is the canonical order of the vtable funs.
    pub fn erl_vtable_items(
        self,
        trait_def_id: DefId,
    ) -> impl Iterator<Item = &'tcx ty::AssocItem> {
        self.associated_items(trait_def_id)
            .in_definition_order()
            .filter(move |item| self.erl_has_fn_name(item.def_id))
    }

    pub fn erl_module_for_target(
        self,
        owner: DefId,
        target: ErlCallTarget<'tcx>,
    ) -> Option<ErlModuleResolution> {
        let typing_env = TypingEnv::post_analysis(self, owner);
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
                        match ty::Instance::try_resolve(self, typing_env, def_id, args) {
                            Ok(Some(ty::Instance {
                                def: ty::InstanceKind::Item(item), ..
                            })) => {
                                self.erl_definition_module(item).map(ErlModuleResolution::Static)
                            }
                            Ok(Some(_)) => {
                                self.resolve_erl_receiver_module(args.type_at(0), typing_env)
                            }
                            Ok(None) | Err(_) => None,
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
        let name = path.data[module_depth..]
            .iter()
            .map(|segment| segment.as_sym(true).to_string())
            .intersperse("::".to_owned())
            .collect();

        return to_atom_symbol(name);
    }

    // `[$patina_type::|$patina_const::]item`
    let member = match tcx.def_kind(def_id) {
        DefKind::AssocTy => format!("$patina_type::{item_name}"),
        DefKind::AssocConst { .. } => format!("$patina_const::{item_name}"),
        _ => item_name.to_string(),
    };

    let parent = tcx.opt_parent(def_id);
    match parent.map(|parent| tcx.def_kind(parent)) {
        Some(DefKind::Impl { .. }) => tcx.erl_impl_member_name(parent.unwrap(), &member),
        Some(DefKind::Trait) => {
            let trait_path = with_erl_paths(|| tcx.def_path_str(parent.unwrap()));
            to_atom_symbol(format!("{trait_path}::{member}"))
        }
        _ => to_atom_symbol(member),
    }
}

/// The optional `Self` suffix in an impl member's Erlang name, without angle brackets.
///
/// Impls may define same-named members that land in one Erlang module, for example when
/// defining `impl From<int> for Foo<int>` and `impl From<int> for Foo<str>` - both can
/// land in the same module for `Foo`.
///
/// We use a suffix <tag> to preemptively disambiguate such functions. For the `From` example,
/// the functions would be defined as `From<int>::from<int>` and `From<int>::from<str>`.
/// This function you're looking at is responsible for figuring out the tag in the suffix.
/// The prefix tag comes from the trait path.
fn erl_impl_self_tag(tcx: TyCtxt<'_>, impl_def_id: DefId) -> Option<String> {
    let self_ty = tcx
        .type_of(impl_def_id)
        .instantiate(tcx, tcx.erl_tag_args(impl_def_id))
        .skip_normalization();

    // First remove the information already supplied by the host module.
    let tag_args: SmallVec<[ty::GenericArg<'_>; 2]> = match *self_ty.kind() {
        ty::Adt(_, args) => args.iter().collect(),
        ty::Slice(item) => smallvec![item.into()],
        ty::Map(key, value) => smallvec![key.into(), value.into()],
        ty::Bool
        | ty::Char
        | ty::Int
        | ty::Float
        | ty::Str
        | ty::Term
        | ty::Atom
        | ty::Bin
        | ty::Bits
        | ty::Param(_) => smallvec![],
        _ => smallvec![self_ty.into()],
    };

    // Then omit bare parameters, but retain nested structure such as `[_]` or `(_,)`.
    let needs_tag = tag_args.iter().any(|arg| match arg.kind() {
        ty::GenericArgKind::Lifetime(_) => false,
        ty::GenericArgKind::Type(ty) => !matches!(ty.kind(), ty::Param(_)),
        ty::GenericArgKind::Const(ct) => !matches!(ct.kind(), ty::ConstKind::Param(_)),
    });
    needs_tag.then(|| tcx.erl_tag_text(tag_args))
}

/// Prints paths from the crate name, so that every crate prints the same text.
fn with_erl_paths<R>(print: impl FnOnce() -> R) -> R {
    ty::print::with_resolve_crate_name!(ty::print::with_no_visible_paths!(
        ty::print::with_no_trimmed_paths!(ty::print::with_disambiguated_paths!(print()))
    ))
}

/// `text`, as a symbol, shortened with a hash suffix if it is longer than the longest atom.
pub fn to_atom_symbol(mut text: String) -> Symbol {
    const MAX_ATOM_CHARS: usize = 255;
    if text.chars().count() <= MAX_ATOM_CHARS {
        return Symbol::intern(&text);
    }

    let mut hasher = StableHasher::new();
    text.hash(&mut hasher);
    let hash: Hash128 = hasher.finish();
    let suffix = format!("#{:016x}", hash.as_u128() as u64);
    let (end, _) = text.char_indices().nth(MAX_ATOM_CHARS - suffix.len()).unwrap();
    text.truncate(end);
    Symbol::intern(&(text + &suffix))
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

pub fn provide(providers: &mut Providers) {
    *providers = Providers {
        erl_adt_reprs,
        erl_fn_name,
        erl_trait_needs_witness: witness::erl_trait_needs_witness,
        erl_trait_supertraits: witness::erl_trait_supertraits,
        erl_assoc_type_bounds: witness::erl_assoc_type_bounds,
        erl_bounds: witness::erl_bounds,
        erl_module_name,
        erl_shape_of_ty: shape::erl_shape_of_ty,
        erl_variant_wrap_guards: shape::erl_variant_wrap_guards,
        ..*providers
    };
}
