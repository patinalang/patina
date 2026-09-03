//! Erlang call resolution and module placement.

use convert_case::{Case, Casing};
use rustc_data_structures::fx::FxIndexSet;
use rustc_hir::attrs::{AttributeKind, ErlTagStyle, ReprAttr};
use rustc_hir::def::{CtorKind, DefKind};
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::definitions::DefPathData;
use rustc_hir::{Attribute, HirId, PrimTy};
use rustc_macros::StableHash;
use rustc_span::{Span, Symbol};
use smallvec::SmallVec;

use crate::query::Providers;
use crate::ty::{self, AdtKind, GenericArgsRef, Ty, TyCtxt, TypingEnv, Unnormalized, Visibility};

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
}

/// The semantic target of an Erlang call.
#[derive(Copy, Clone, Debug)]
pub enum ErlCallTarget<'tcx> {
    /// A free function, inherent method, or trait method.
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
pub enum ErlRepr {
    Elixir,
    Tagged,
    Atom,
    Transparent,
    NativeRecord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, StableHash)]
pub struct ErlVariantRepr {
    pub kind: ErlRepr,
    pub tag: Symbol,
    pub record_module: Option<Symbol>,
}

enum ErlTag {
    Explicit(Symbol),
    Style(ErlTagStyle),
}

impl<'tcx> TyCtxt<'tcx> {
    /// Determines the generated module in which a Rust definition is emitted.
    pub fn erl_definition_module(self, def_id: impl Into<DefId>) -> Option<ErlModuleId> {
        let mut def_id = def_id.into();

        loop {
            match self.def_kind(def_id) {
                DefKind::Mod => return Some(ErlModuleId::Mod(def_id)),

                DefKind::Struct | DefKind::Enum | DefKind::Union => {
                    return Some(ErlModuleId::Adt(def_id));
                }

                DefKind::ExternMod => return Some(ErlModuleId::Extern(def_id)),

                DefKind::Impl { .. } => {
                    let self_ty = self.type_of(def_id).instantiate_identity().skip_normalization();
                    return match self
                        .resolve_erl_receiver_module(self_ty, self.erl_typing_env(def_id))?
                    {
                        ErlModuleResolution::Static(module) => Some(module),
                        ErlModuleResolution::Dynamic => None,
                    };
                }

                _ => def_id = self.parent(def_id),
            }
        }
    }

    /// The trait `def_id` is a method of, for a trait method or a trait impl item.
    pub fn erl_entry_trait(self, def_id: DefId) -> Option<DefId> {
        if let Some(trait_def_id) = self.trait_of_assoc(def_id) {
            return Some(trait_def_id);
        }
        let impl_def_id = self.trait_impl_of_assoc(def_id)?;
        self.impl_opt_trait_id(impl_def_id)
    }

    /// The trait's own generic arguments for a trait impl item, excluding the
    /// leading `Self` argument. Empty for inherent impl items and free items.
    pub fn erl_trait_args(self, def_id: impl Into<DefId>) -> &'tcx [ty::GenericArg<'tcx>] {
        let def_id = def_id.into();
        let Some(impl_def_id) = self.trait_impl_of_assoc(def_id) else {
            return &[];
        };
        let Some(trait_ref) = self.impl_opt_trait_ref(impl_def_id) else {
            return &[];
        };
        let trait_ref = trait_ref.skip_binder();
        &trait_ref.args[1..]
    }

    /// The type parameters (in `def_id`'s own generic space) that appear in a
    /// trait bound, and therefore need a witness at runtime.
    ///
    /// For associated items, the parent impl's predicates are also walked, so
    /// an inherent method's enclosing `impl<T: Bound>` marks `T`. Unbounded
    /// parameters get no witness.
    ///
    /// This is callable for foreign items: predicates are encoded in rmeta.
    pub fn erl_witness_params(self, def_id: impl Into<DefId>) -> SmallVec<[ty::ParamTy; 6]> {
        let def_id = def_id.into();
        let mut params = FxIndexSet::default();

        self.collect_witness_params(def_id, &mut params);

        if let Some(parent) = self.predicates_of(def_id).parent {
            self.collect_witness_params(parent, &mut params);
        }

        let mut params = params.into_iter().collect::<SmallVec<[ty::ParamTy; 6]>>();
        params.sort_unstable();
        params
    }

    fn collect_witness_params(self, def_id: DefId, params: &mut FxIndexSet<ty::ParamTy>) {
        for &(clause, _span) in self.predicates_of(def_id).predicates {
            let ty::ClauseKind::Trait(trait_pred) = clause.kind().skip_binder() else {
                continue;
            };

            // Skip implicit rustc traits from determining whether T is bounded or not.
            let lang_items = self.lang_items();
            if lang_items.sized_trait() == Some(trait_pred.def_id())
                || lang_items.meta_sized_trait() == Some(trait_pred.def_id())
                || lang_items.pointee_sized_trait() == Some(trait_pred.def_id())
            {
                continue;
            }

            for arg in trait_pred.trait_ref.args.iter() {
                let ty::GenericArgKind::Type(ty) = arg.kind() else { continue };

                // for each type parameter in the trait argument (outer loop), recursively collect
                // it and its sub-type parameters (inner loop)
                for arg in ty.walk() {
                    if let ty::GenericArgKind::Type(ty) = arg.kind()
                        && let ty::TyKind::Param(param) = ty.kind()
                    {
                        params.insert(*param);
                    }
                }
            }
        }
    }

    /// The witness types a call site must pass for `def_id`, in order: the
    /// receiver witness (Self-type; needed for trait entries only, so only materialized for them),
    /// the trait-argument witnesses (for trait's generic params; trait entries only), then the
    /// witnesses for the callee's own generic parameters that appear in a trait bound.
    pub fn erl_call_witness_tys(
        self,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> SmallVec<[Ty<'tcx>; 6]> {
        let mut tys = SmallVec::new();
        let is_trait_entry = self.erl_entry_trait(def_id).is_some();

        if is_trait_entry {
            let Some(receiver) = self.erl_call_target_self_ty(ErlCallTarget::Item { def_id, args })
            else {
                bug!("trait entry call must have a receiver");
            };
            tys.push(receiver);

            let trait_args = match self.def_kind(def_id) {
                DefKind::AssocFn if self.def_kind(self.parent(def_id)) == DefKind::Trait => {
                    // A trait method's args are `[Self, trait args..., method args...]`.
                    let parent_count = self.generics_of(def_id).parent_count;
                    &args[1..parent_count]
                }
                _ => bug!("trait entry call resolved to impl item `{}`", self.def_path_str(def_id)),
            };
            tys.extend(trait_args.iter().filter_map(|arg| arg.as_type()));
        }

        let parent_count = self.generics_of(def_id).parent_count;
        for param in self.erl_witness_params(def_id) {
            // Trait entries recover the receiver and trait arguments via the self-type witness
            // and the leading trait-argument witnesses, so only the callee's *own* generics
            // need a witness here.
            if !is_trait_entry || param.index as usize >= parent_count {
                tys.push(args.type_at(param.index as usize));
            }
        }

        tys
    }

    /// Resolves which Erlang module implements `ty`.
    pub fn erl_module_for_type(self, owner: DefId, ty: Ty<'tcx>) -> Option<ErlModuleResolution> {
        self.resolve_erl_receiver_module(ty, self.erl_typing_env(owner))
    }

    /// For a given adt or tuple type, list its subtypes.
    /// For example, `Result<T, E>` has subtypes `T` and `E`.
    pub fn subtypes_of(self, ty: Ty<'tcx>) -> SmallVec<[Ty<'tcx>; 3]> {
        match ty.kind() {
            ty::Adt(_, args) => args.iter().filter_map(|arg| arg.as_type()).collect(),
            ty::Tuple(types) => types.iter().collect(),
            ty::Slice(element) => SmallVec::from_iter([*element]),
            _ => SmallVec::new(),
        }
    }

    pub fn is_forced_extern(self, hir_id: HirId) -> bool {
        self.hir_attrs(hir_id)
            .iter()
            .any(|attr| matches!(attr, Attribute::Parsed(AttributeKind::ExternCall(..))))
    }

    /// Returns whether a function must be exported from its generated module.
    ///
    /// All `pub(x)` where `x != self` are always exported.
    /// The only complication is with associated functions, as `pub(self)` associated functions
    /// can be called from the parent module of the `impl` block, or from local `impl`s for other
    /// types. In that case, we additionally export those private associated functions that
    /// are actually called externally.
    pub fn is_erl_exported_fn(self, def_id: LocalDefId) -> bool {
        let visible_outside_rust_module = match self.visibility(def_id) {
            Visibility::Public => true,
            Visibility::Restricted(restricted) => {
                restricted != self.parent_module_from_def_id(def_id).to_def_id()
            }
        };

        visible_outside_rust_module
            || (self.def_kind(def_id) == DefKind::AssocFn
                && self.assoc_fns_called_externally(()).contains(&def_id))
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
                if callee != self.erl_definition_module(caller)? {
                    ErlCallKind::Extern
                } else if self.erl_call_target_is_exported(target) {
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
            DefKind::Fn | DefKind::AssocFn => ErlCallTarget::Item { def_id, args },

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

    fn erl_call_target_is_exported(self, target: ErlCallTarget<'tcx>) -> bool {
        let def_id = match target {
            ErlCallTarget::Item { def_id, args: _ } => def_id,
            ErlCallTarget::Constructor { variant, .. } => variant,
            ErlCallTarget::Extern { .. } | ErlCallTarget::Dynamic => return false,
        };

        def_id.as_local().is_some_and(|local_def_id| self.is_erl_exported_fn(local_def_id))
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

            DefKind::AssocFn => {
                let parent = self.parent(def_id);

                match self.def_kind(parent) {
                    DefKind::Impl { .. } => {
                        let self_ty =
                            self.type_of(parent).instantiate(self, args).skip_normalization();
                        self.resolve_erl_receiver_module(self_ty, typing_env)
                    }

                    DefKind::Trait => self.resolve_erl_receiver_module(args.type_at(0), typing_env),

                    _ => bug!("associated function has unexpected parent: {parent:?}"),
                }
            }

            kind => bug!("non-callable item passed to erl_module_for_target_item: {kind:?}"),
        }
    }

    /// The receiver type for a method call.
    ///
    /// Returns `Some` for a trait method (`args.type_at(0)`) or an inherent
    /// method (the impl's self type), and `None` for non-associated items.
    pub fn erl_call_target_self_ty(self, target: ErlCallTarget<'tcx>) -> Option<Ty<'tcx>> {
        let ErlCallTarget::Item { def_id, args } = target else { return None };
        let DefKind::AssocFn = self.def_kind(def_id) else { return None };

        let parent = self.parent(def_id);
        match self.def_kind(parent) {
            DefKind::Impl { .. } => {
                Some(self.type_of(parent).instantiate(self, args).skip_normalization())
            }
            DefKind::Trait => Some(args.type_at(0)),
            _ => None,
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

    pub fn erl_adt_repr(self, def_id: DefId) -> ErlRepr {
        if let Some(kind) = self.explicit_erl_repr(def_id) {
            return kind;
        }

        let adt = self.adt_def(def_id);
        match adt.adt_kind() {
            AdtKind::Struct => ErlRepr::Elixir,
            AdtKind::Enum => ErlRepr::NativeRecord,
            AdtKind::Union => unimplemented!(),
        }
    }

    /// Resolves the runtime representation of a struct or enum variant.
    pub fn erl_variant_repr(self, variant_def_id: DefId) -> ErlVariantRepr {
        let adt = match self.def_kind(variant_def_id) {
            DefKind::Struct => self.adt_def(variant_def_id),
            DefKind::Variant => self.adt_def(self.parent(variant_def_id)),
            kind => bug!("expected struct or enum variant, got {kind:?}"),
        };
        let variant = if adt.is_enum() {
            adt.variant_with_id(variant_def_id)
        } else {
            adt.non_enum_variant()
        };

        let kind = match self.explicit_erl_repr(variant_def_id) {
            Some(repr) => repr,
            None if variant.ctor_kind() == Some(CtorKind::Const) => ErlRepr::Atom,
            None => self.erl_adt_repr(adt.did()),
        };

        ErlVariantRepr { kind, tag: self.erl_adt_tag(variant_def_id), record_module: None }
    }

    /// Returns the nearest explicitly selected representation, inheriting from an enum when needed.
    fn explicit_erl_repr(self, def_id: DefId) -> Option<ErlRepr> {
        if let Some((repr, _)) = self.find_primary_erl_repr_attr(def_id) {
            return Some(repr);
        }

        if self.def_kind(def_id) == DefKind::Variant
            && let Some((repr, _)) = self.find_primary_erl_repr_attr(self.parent(def_id))
        {
            return Some(repr);
        }

        None
    }

    pub fn find_primary_erl_repr_attr(self, def_id: DefId) -> Option<(ErlRepr, Span)> {
        let reprs = rustc_hir::find_attr!(self, def_id, Repr { reprs, .. } => reprs)?;

        reprs.iter().find_map(|(repr, span)| match repr {
            ReprAttr::Elixir => Some((ErlRepr::Elixir, *span)),
            ReprAttr::Tagged => Some((ErlRepr::Tagged, *span)),
            ReprAttr::Atom => Some((ErlRepr::Atom, *span)),
            ReprAttr::NativeRecord => Some((ErlRepr::NativeRecord, *span)),
            _ => None,
        })
    }

    /// Returns the atom used to tag a struct or enum variant.
    pub fn erl_adt_tag(self, variant_def_id: DefId) -> Symbol {
        assert!(matches!(self.def_kind(variant_def_id), DefKind::Struct | DefKind::Variant));

        let own_tag = self.explicit_erl_tag(variant_def_id);
        let inherited_tag = (self.def_kind(variant_def_id) == DefKind::Variant)
            .then(|| self.explicit_erl_tag(self.parent(variant_def_id)))
            .flatten();

        match own_tag.or(inherited_tag) {
            Some(ErlTag::Explicit(tag)) => tag,
            Some(ErlTag::Style(ErlTagStyle::SnakeCase)) => {
                Symbol::intern(&self.item_name(variant_def_id).as_str().to_case(Case::Snake))
            }
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

    fn explicit_erl_tag(self, def_id: DefId) -> Option<ErlTag> {
        let reprs = rustc_hir::find_attr!(self, def_id, Repr { reprs, .. } => reprs)?;

        if let Some(tag) = reprs.iter().find_map(|(repr, _)| match repr {
            ReprAttr::Tag(tag) => Some(*tag),
            _ => None,
        }) {
            return Some(ErlTag::Explicit(tag));
        }

        reprs.iter().find_map(|(repr, _)| match repr {
            ReprAttr::TagStyle(style) => Some(ErlTag::Style(*style)),
            _ => None,
        })
    }
}

/// Provider for [`TyCtxt::erl_fn_name`].
fn erl_fn_name(tcx: TyCtxt<'_>, def_id: DefId) -> Symbol {
    let item_name = tcx.item_name(def_id);

    let Some(trait_def_id) = tcx.erl_entry_trait(def_id) else { return item_name };

    let path = tcx.def_path_str(trait_def_id);
    let qualified = if trait_def_id.is_local() {
        let crate_name = tcx.crate_name(trait_def_id.krate);
        format!("{crate_name}::{path}")
    } else {
        path
    };

    Symbol::intern(&format!("{qualified}::{item_name}"))
}

/// Provider for [`TyCtxt::erl_module_name`].
fn erl_module_name(tcx: TyCtxt<'_>, module: ErlModuleId) -> Symbol {
    match module {
        ErlModuleId::Mod(def_id) | ErlModuleId::Adt(def_id) => {
            if let Some(name) = rustc_hir::find_attr!(tcx, def_id, ErlModule(name, _) => *name) {
                return name;
            }

            let path = tcx.def_path(def_id);

            let mut segments = SmallVec::<[String; 6]>::from_iter([
                "Elixir.Patina".to_owned(),
                tcx.crate_name(path.krate).as_str().to_owned(),
            ]);

            for data in &path.data {
                if data.data != DefPathData::Ctor {
                    segments.push(data.as_sym(true).as_str().to_owned());
                }
            }

            Symbol::intern(&segments.join("."))
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
            Symbol::intern(&format!("Elixir.Patina.Core.{}", prim_ty.name()))
        }
    }
}

pub fn provide(providers: &mut Providers) {
    *providers = Providers { erl_fn_name, erl_module_name, ..*providers };
}
