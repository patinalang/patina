//! Erlang call resolution and module placement.

use rustc_hir::attrs::AttributeKind;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::{Attribute, HirId, PrimTy};

use crate::ty::{self, AdtKind, GenericArgsRef, Ty, TyCtxt, TypingEnv, Unnormalized, Visibility};

/// The identity of a module in the generated Erlang program.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErlAdtRepr {
    Elixir,
    NativeRecord,
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

    pub fn erl_adt_repr(self, def_id: DefId) -> ErlAdtRepr {
        let adt = self.adt_def(def_id);
        match adt.adt_kind() {
            AdtKind::Struct => ErlAdtRepr::Elixir,
            AdtKind::Enum => ErlAdtRepr::NativeRecord,
            AdtKind::Union => unimplemented!(),
        }
    }
}
