use rustc_ast::LitKind;
use rustc_hir;
use rustc_macros::StableHash;

use crate::ty::{self, Ty, TyCtxt};

/// Input argument for `tcx.lit_to_const`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, StableHash)]
pub struct LitToConstInput<'tcx> {
    /// The absolute value of the resultant constant.
    pub lit: LitKind,
    /// The type of the constant.
    ///
    /// `None` is used by const generics when the type of the constant is unknown, e.g.
    /// if there are inference variables
    pub ty: Option<Ty<'tcx>>,
    /// If the constant is negative.
    pub neg: bool,
}

/// Checks whether a literal can be interpreted as a const of the given type.
pub fn const_lit_matches_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    kind: &LitKind,
    ty: Ty<'tcx>,
    _neg: bool,
) -> bool {
    match (*kind, ty.kind()) {
        (LitKind::Str(..), ty::Ref(_, inner_ty, _)) if inner_ty.is_str() => true,
        (LitKind::Str(..), ty::Str) => true,
        (LitKind::ByteStr(..), ty::Ref(_, inner_ty, _))
            if let ty::Slice(ty) | ty::Array(ty, _) = inner_ty.kind()
                && matches!(ty.kind(), ty::Int) =>
        {
            true
        }
        (LitKind::ByteStr(..), ty::Slice(inner_ty) | ty::Array(inner_ty, _))
            if tcx.features().deref_patterns() && matches!(inner_ty.kind(), ty::Int) =>
        {
            true
        }
        (LitKind::Byte(..), ty::Int) => true,
        (LitKind::CStr(..), ty::Ref(_, inner_ty, _))
            if matches!(inner_ty.kind(), ty::Adt(def, _)
                if tcx.is_lang_item(def.did(), rustc_hir::LangItem::CStr)) =>
        {
            true
        }
        (LitKind::Int(..), ty::Int) => true,
        (LitKind::Bool(..), ty::Bool) => true,
        (LitKind::Float(..), ty::Float) => true,
        (LitKind::Char(..), ty::Char) => true,
        (LitKind::Err(..), _) => true,
        _ => false,
    }
}
