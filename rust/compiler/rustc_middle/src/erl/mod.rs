//! Erlang call resolution and module placement.

use rustc_hir::def::{CtorKind, DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::{Expr, ExprKind, PrimTy};

use crate::ty::{self, GenericArgsRef, Ty, TyCtxt, TypingEnv, Unnormalized, Visibility};

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

/// A Rust item together with the instantiation at a particular use site.
#[derive(Copy, Clone, Debug)]
pub struct ErlItemInstance<'tcx> {
    pub def_id: DefId,
    pub args: GenericArgsRef<'tcx>,
}

/// The semantic result of resolving a Rust call expression.
#[derive(Copy, Clone, Debug)]
pub enum ErlCallTarget<'tcx> {
    /// A free function, inherent method, or statically selected trait method.
    Item(ErlItemInstance<'tcx>),
    /// A struct or enum-variant constructor.
    Constructor { nominal_type: DefId, variant: Option<DefId>, args: GenericArgsRef<'tcx> },
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

    /// Resolves and classifies a call without lowering it.
    pub fn erl_call_kind(self, call: &Expr<'tcx>) -> Option<ErlCallKind> {
        let target = self.resolve_erl_call_target(call)?;
        self.erl_call_kind_from(call.hir_id.owner.to_def_id(), target)
    }

    pub fn erl_call_kind_from(
        self,
        caller: DefId,
        target: ErlCallTarget<'tcx>,
    ) -> Option<ErlCallKind> {
        let callee = self.erl_module_for_target(caller, target)?;
        let caller = self.erl_definition_module(caller)?;

        let kind = match callee {
            ErlModuleResolution::Static(callee) => {
                if callee != caller {
                    ErlCallKind::Extern
                } else if self.erl_call_target_is_exported(target) {
                    ErlCallKind::LocalOrExtern
                } else {
                    ErlCallKind::Local
                }
            }
            ErlModuleResolution::Dynamic => ErlCallKind::Dynamic,
        };

        Some(kind)
    }

    pub fn resolve_erl_call_target(self, call: &Expr<'tcx>) -> Option<ErlCallTarget<'tcx>> {
        let typeck = self.typeck(call.hir_id.owner.def_id);
        let typing_env = self.erl_typing_env(call.hir_id.owner.def_id);

        match call.kind {
            ExprKind::Call(callee, _) => match callee.kind {
                ExprKind::Path(ref qpath) => {
                    let res = typeck.qpath_res(qpath, callee.hir_id);
                    let args = typeck.node_args(callee.hir_id);

                    match res {
                        Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) => self
                            .resolve_erl_item_target_with_env(
                                ErlItemInstance { def_id, args },
                                typing_env,
                            ),

                        Res::Def(DefKind::Ctor(_, CtorKind::Fn), ctor_def_id) => {
                            let adt = typeck.expr_ty(call).ty_adt_def()?;
                            let variant = adt.variant_with_ctor_id(ctor_def_id).def_id;

                            Some(ErlCallTarget::Constructor {
                                nominal_type: adt.did(),
                                variant: Some(variant),
                                args,
                            })
                        }

                        Res::SelfCtor(_) => {
                            let adt = typeck.expr_ty(call).ty_adt_def()?;
                            Some(ErlCallTarget::Constructor {
                                nominal_type: adt.did(),
                                variant: Some(adt.non_enum_variant().def_id),
                                args,
                            })
                        }

                        // Patina models a foreign function as the leaf `ExternMod`;
                        // its parent identifies the named foreign module.
                        // IOW, in `Elixir::IO::puts` every segment is an `ExternMod`; if we try
                        // to call it, the target is `Elixir::IO`.
                        Res::Def(DefKind::ExternMod, function) => {
                            Some(ErlCallTarget::Extern { module: self.parent(function), function })
                        }

                        _ => Some(ErlCallTarget::Dynamic),
                    }
                }

                // Calling any expression other than a path means calling a value.
                _ => Some(ErlCallTarget::Dynamic),
            },

            ExprKind::MethodCall(..) => {
                let Some(def_id) = typeck.type_dependent_def_id(call.hir_id) else {
                    return Some(ErlCallTarget::Dynamic);
                };
                let args = typeck.node_args(call.hir_id);
                self.resolve_erl_item_target_with_env(ErlItemInstance { def_id, args }, typing_env)
            }

            _ => bug!("resolve_erl_call_target called on a non-call expression"),
        }
    }

    pub fn resolve_erl_item_target(
        self,
        owner: DefId,
        item: ErlItemInstance<'tcx>,
    ) -> Option<ErlCallTarget<'tcx>> {
        self.resolve_erl_item_target_with_env(item, self.erl_typing_env(owner))
    }

    fn resolve_erl_item_target_with_env(
        self,
        item: ErlItemInstance<'tcx>,
        typing_env: TypingEnv<'tcx>,
    ) -> Option<ErlCallTarget<'tcx>> {
        let instance =
            ty::Instance::try_resolve(self, typing_env, item.def_id, item.args).ok()??;

        match instance.def {
            ty::InstanceKind::Item(def_id) => {
                Some(ErlCallTarget::Item(ErlItemInstance { def_id, args: instance.args }))
            }

            ty::InstanceKind::Virtual(..) => Some(ErlCallTarget::Dynamic),

            _ => None,
        }
    }

    fn erl_typing_env(self, owner: impl Into<DefId>) -> TypingEnv<'tcx> {
        let owner = owner.into();
        TypingEnv::new(self.param_env(owner), ty::TypingMode::non_body_analysis())
    }

    fn erl_call_target_is_exported(self, target: ErlCallTarget<'tcx>) -> bool {
        let def_id = match target {
            ErlCallTarget::Item(item) => item.def_id,
            ErlCallTarget::Constructor { nominal_type, variant, .. } => {
                variant.unwrap_or(nominal_type)
            }
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
            ErlCallTarget::Item(item) => self.place_erl_item_instance(item, typing_env),

            ErlCallTarget::Constructor { nominal_type, .. } => {
                Some(ErlModuleResolution::Static(ErlModuleId::Adt(nominal_type)))
            }

            ErlCallTarget::Extern { module, .. } => {
                Some(ErlModuleResolution::Static(ErlModuleId::Extern(module)))
            }

            ErlCallTarget::Dynamic => Some(ErlModuleResolution::Dynamic),
        }
    }

    fn place_erl_item_instance(
        self,
        item: ErlItemInstance<'tcx>,
        typing_env: TypingEnv<'tcx>,
    ) -> Option<ErlModuleResolution> {
        match self.def_kind(item.def_id) {
            DefKind::Fn => Some(ErlModuleResolution::Static(ErlModuleId::Mod(
                self.lexical_erl_module(item.def_id),
            ))),

            DefKind::AssocFn => {
                let parent = self.parent(item.def_id);

                match self.def_kind(parent) {
                    DefKind::Impl { .. } => {
                        let self_ty =
                            self.type_of(parent).instantiate(self, item.args).skip_normalization();
                        self.resolve_erl_receiver_module(self_ty, typing_env)
                    }

                    DefKind::Trait => {
                        // Reaching this arm means method selection did not replace the
                        // trait item with a concrete implementation.
                        None
                    }

                    _ => bug!("associated function has unexpected parent: {parent:?}"),
                }
            }

            kind => bug!("non-callable item passed to place_erl_item_instance: {kind:?}"),
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
}
