//! This module contains logic for determining whether a type is inhabited or
//! uninhabited. The [`InhabitedPredicate`] type captures the minimum
//! information needed to determine whether a type is inhabited given a
//! `ParamEnv` and module ID.
//!
//! # Example
//! ```rust
//! #![feature(never_type)]
//! mod a {
//!     pub mod b {
//!         pub struct SecretlyUninhabited {
//!             _priv: !,
//!         }
//!     }
//! }
//!
//! mod c {
//!     enum Void {}
//!     pub struct AlsoSecretlyUninhabited {
//!         _priv: Void,
//!     }
//!     mod d {
//!     }
//! }
//!
//! struct Foo {
//!     x: a::b::SecretlyUninhabited,
//!     y: c::AlsoSecretlyUninhabited,
//! }
//! ```
//! In this code, the type `Foo` will only be visibly uninhabited inside the
//! modules `b`, `c` and `d`. Calling `inhabited_predicate` on `Foo` will
//! return `NotInModule(b) AND NotInModule(c)`.
//!
//! We need this information for pattern-matching on `Foo` or types that contain
//! `Foo`.
//!
//! # Example
//! ```ignore(illustrative)
//! let foo_result: Result<T, Foo> = ... ;
//! let Ok(t) = foo_result;
//! ```
//! This code should only compile in modules where the uninhabitedness of `Foo`
//! is visible.

use rustc_type_ir::TyKind::*;
use smallvec::{SmallVec, smallvec};
use tracing::instrument;

use crate::query::Providers;
use crate::ty::{self, DefId, Ty, TyCtxt, TypeVisitableExt, VariantDef, Visibility};

pub mod inhabited_predicate;

pub use inhabited_predicate::InhabitedPredicate;

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { inhabited_predicate_adt, inhabited_predicate_type, ..*providers };
}

/// Returns an `InhabitedPredicate` that is generic over type parameters and
/// requires calling [`InhabitedPredicate::instantiate`]
fn inhabited_predicate_adt(tcx: TyCtxt<'_>, def_id: DefId) -> InhabitedPredicate<'_> {
    let args = ty::GenericArgs::identity_for_item(tcx, def_id);
    inhabited_predicate_rec(tcx, Ty::new_adt(tcx, tcx.adt_def(def_id), args), &mut smallvec![])
}

impl<'tcx> VariantDef {
    /// Calculates this variant's inhabitedness with one context through its recursive fields.
    pub fn inhabited_predicate(
        &self,
        tcx: TyCtxt<'tcx>,
        adt: ty::AdtDef<'tcx>,
    ) -> InhabitedPredicate<'tcx> {
        let args = ty::GenericArgs::identity_for_item(tcx, adt.did());
        let ty = Ty::new_adt(tcx, adt, args);
        self.inhabited_predicate_rec(tcx, adt, args, &mut smallvec![ty])
    }

    fn inhabited_predicate_rec(
        &self,
        tcx: TyCtxt<'tcx>,
        adt: ty::AdtDef<'tcx>,
        args: ty::GenericArgsRef<'tcx>,
        active: &mut SmallVec<[Ty<'tcx>; 2]>,
    ) -> InhabitedPredicate<'tcx> {
        debug_assert!(!adt.is_union());
        InhabitedPredicate::all(
            tcx,
            self.fields.iter().map(|field| {
                let pred = inhabited_predicate_rec(tcx, field.ty(tcx, args), active);
                if adt.is_enum() {
                    return pred;
                }
                match field.vis {
                    Visibility::Public => pred,
                    Visibility::Restricted(from) => {
                        pred.or(tcx, InhabitedPredicate::NotInModule(from))
                    }
                }
            }),
        )
    }
}

fn inhabited_predicate_rec<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    active: &mut SmallVec<[Ty<'tcx>; 2]>,
) -> InhabitedPredicate<'tcx> {
    match *ty.kind() {
        ty::Adt(adt, _args)
            if adt.is_union()
                || adt.variant_list_has_applicable_non_exhaustive()
                || active.iter().any(|&outer| {
                    matches!(outer.kind(), ty::Adt(outer_adt, _) if outer_adt.did() == adt.did())
                        && !outer.walk().skip(1).any(|arg| arg.as_type() == Some(ty))
                }) =>
        {
            InhabitedPredicate::True
        }

        ty::Adt(adt, args) => {
            active.push(ty);
            let result = InhabitedPredicate::any(
                tcx,
                adt.variants()
                    .iter()
                    .map(|variant| variant.inhabited_predicate_rec(tcx, adt, args, active)),
            );
            active.pop();
            result
        }

        ty::Tuple(tys) => InhabitedPredicate::all(
            tcx,
            tys.iter().map(|ty| inhabited_predicate_rec(tcx, ty, active)),
        ),

        ty::Array(inner, len) => match len.try_to_target_usize(tcx) {
            Some(0) => InhabitedPredicate::True,
            Some(_) => inhabited_predicate_rec(tcx, inner, active),
            None => inhabited_predicate_rec(tcx, inner, active)
                .or(tcx, InhabitedPredicate::ConstIsZero(len)),
        },

        // Preserve generic predicates, opaque types, and ordinary primitive behavior.
        _ => ty.inhabited_predicate(tcx),
    }
}

impl<'tcx> Ty<'tcx> {
    #[instrument(level = "debug", skip(tcx), ret)]
    pub fn inhabited_predicate(self, tcx: TyCtxt<'tcx>) -> InhabitedPredicate<'tcx> {
        debug_assert!(!self.has_infer());
        match self.kind() {
            // For now, unions are always considered inhabited
            Adt(adt, _) if adt.is_union() => InhabitedPredicate::True,
            // Non-exhaustive ADTs from other crates are always considered inhabited
            Adt(adt, _) if adt.variant_list_has_applicable_non_exhaustive() => {
                InhabitedPredicate::True
            }
            Never => InhabitedPredicate::False,
            Param(_)
            | Alias(ty::AliasTy {
                kind: ty::Inherent { .. } | ty::Projection { .. } | ty::Free { .. },
                ..
            }) => InhabitedPredicate::GenericType(self),
            &Alias(ty::AliasTy { kind: ty::Opaque { def_id }, args, .. }) => {
                match def_id.as_local() {
                    // Foreign opaque is considered inhabited.
                    None => InhabitedPredicate::True,
                    // Local opaque type may possibly be revealed.
                    Some(local_def_id) => {
                        let key = ty::OpaqueTypeKey { def_id: local_def_id, args };
                        InhabitedPredicate::OpaqueType(key)
                    }
                }
            }
            Tuple(tys) if tys.is_empty() => InhabitedPredicate::True,
            // use a query for more complex cases
            Adt(..) | Array(..) | Tuple(_) => tcx.inhabited_predicate_type(self),
            // references and other types are inhabited
            _ => InhabitedPredicate::True,
        }
    }

    /// Checks whether a type is visibly uninhabited from a particular module.
    ///
    /// # Example
    /// ```
    /// #![feature(never_type)]
    /// # fn main() {}
    /// enum Void {}
    /// mod a {
    ///     pub mod b {
    ///         pub struct SecretlyUninhabited {
    ///             _priv: !,
    ///         }
    ///     }
    /// }
    ///
    /// mod c {
    ///     use super::Void;
    ///     pub struct AlsoSecretlyUninhabited {
    ///         _priv: Void,
    ///     }
    ///     mod d {
    ///     }
    /// }
    ///
    /// struct Foo {
    ///     x: a::b::SecretlyUninhabited,
    ///     y: c::AlsoSecretlyUninhabited,
    /// }
    /// ```
    /// In this code, the type `Foo` will only be visibly uninhabited inside the
    /// modules b, c and d. This effects pattern-matching on `Foo` or types that
    /// contain `Foo`.
    ///
    /// # Example
    /// ```ignore (illustrative)
    /// let foo_result: Result<T, Foo> = ... ;
    /// let Ok(t) = foo_result;
    /// ```
    /// This code should only compile in modules where the uninhabitedness of Foo is
    /// visible.
    pub fn is_inhabited_from(
        self,
        tcx: TyCtxt<'tcx>,
        module: DefId,
        typing_env: ty::TypingEnv<'tcx>,
    ) -> bool {
        self.inhabited_predicate(tcx).apply(tcx, typing_env, module)
    }

    /// Returns true if the type is uninhabited without regard to visibility
    pub fn is_privately_uninhabited(
        self,
        tcx: TyCtxt<'tcx>,
        typing_env: ty::TypingEnv<'tcx>,
    ) -> bool {
        !self.inhabited_predicate(tcx).apply_ignore_module(tcx, typing_env)
    }
}

/// N.B. this query should only be called through `Ty::inhabited_predicate`
fn inhabited_predicate_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> InhabitedPredicate<'tcx> {
    match *ty.kind() {
        Adt(adt, args) => tcx.inhabited_predicate_adt(adt.did()).instantiate(tcx, args),

        Tuple(tys) => {
            InhabitedPredicate::all(tcx, tys.iter().map(|ty| ty.inhabited_predicate(tcx)))
        }

        // If we can evaluate the array length before having a `ParamEnv`, then
        // we can simplify the predicate. This is an optimization.
        Array(ty, len) => match len.try_to_target_usize(tcx) {
            Some(0) => InhabitedPredicate::True,
            Some(1..) => ty.inhabited_predicate(tcx),
            None => ty.inhabited_predicate(tcx).or(tcx, InhabitedPredicate::ConstIsZero(len)),
        },

        _ => bug!("unexpected TyKind, use `Ty::inhabited_predicate`"),
    }
}
