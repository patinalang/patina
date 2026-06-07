use std::fmt;

pub use rustc_abi::MAX_SIMD_LANES;
use rustc_abi::{Align, ExternAbi, HasDataLayout, Size};
use rustc_errors::{DiagArgValue, IntoDiagArg};
use rustc_hir::LangItem;
use rustc_hir::def_id::DefId;
use rustc_macros::{StableHash, extension};

use crate::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use crate::ty::{self, Ty, TyCtxt};

#[extension(pub trait IntegerExt)]
impl rustc_abi::Integer {
    #[inline]
    fn to_ty<'tcx>(&self, tcx: TyCtxt<'tcx>, _signed: bool) -> Ty<'tcx> {
        tcx.types.int
    }
}

#[derive(Copy, Clone, Debug, StableHash)]
pub enum LayoutError<'tcx> {
    /// A type doesn't have a sensible layout.
    ///
    /// This variant is used for layout errors that don't necessarily cause
    /// compile errors.
    ///
    /// For example, this can happen if a struct contains an unsized type in a
    /// non-tail field, but has an unsatisfiable bound like `str: Sized`.
    Unknown(Ty<'tcx>),
    /// The layout can vary due to a generic parameter.
    ///
    /// Unlike `Unknown`, this variant is a "soft" error and indicates that the layout
    /// may become computable after further instantiating the generic parameter(s).
    TooGeneric(Ty<'tcx>),
}

impl<'tcx> fmt::Display for LayoutError<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LayoutError::Unknown(ty) => write!(f, "the type `{ty}` has an unknown layout"),
            LayoutError::TooGeneric(ty) => {
                write!(f, "the type `{ty}` does not have a fixed layout")
            }
        }
    }
}

impl<'tcx> IntoDiagArg for LayoutError<'tcx> {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        self.to_string().into_diag_arg(&mut None)
    }
}

pub fn ty_size_for_frontend<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Size> {
    match ty.kind() {
        ty::Bool => Some(Size::from_bytes(1)),
        ty::Char => Some(Size::from_bytes(4)),
        ty::Int | ty::Float => Some(ty.primitive_size(tcx)),
        ty::Ref(..) | ty::RawPtr(..) | ty::FnPtr(..) => Some(tcx.data_layout().pointer_size()),
        ty::Never => Some(Size::ZERO),
        ty::Tuple(list) if list.is_empty() => Some(Size::ZERO),
        _ => None,
    }
}

pub fn ty_size_and_align_for_frontend<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
) -> Option<(Size, Align)> {
    let size = ty_size_for_frontend(tcx, ty)?;
    Some((size, Align::ONE))
}

/// Calculates whether a function's ABI can unwind or not.
#[inline]
#[tracing::instrument(level = "debug", skip(tcx))]
pub fn fn_can_unwind(tcx: TyCtxt<'_>, fn_def_id: Option<DefId>, abi: ExternAbi) -> bool {
    if let Some(did) = fn_def_id {
        if tcx.codegen_fn_attrs(did).flags.contains(CodegenFnAttrFlags::NEVER_UNWIND) {
            return false;
        }

        if !tcx.sess.panic_strategy().unwinds() && !tcx.is_foreign_item(did) {
            return false;
        }

        if !tcx.sess.opts.unstable_opts.panic_in_drop.unwinds()
            && tcx.is_lang_item(did, LangItem::DropInPlace)
        {
            return false;
        }
    }

    use ExternAbi::*;
    match abi {
        C { unwind }
        | System { unwind }
        | Cdecl { unwind }
        | Stdcall { unwind }
        | Fastcall { unwind }
        | Vectorcall { unwind }
        | Thiscall { unwind }
        | Aapcs { unwind }
        | Win64 { unwind }
        | SysV64 { unwind } => unwind,
        PtxKernel
        | Msp430Interrupt
        | X86Interrupt
        | GpuKernel
        | EfiApi
        | AvrInterrupt
        | AvrNonBlockingInterrupt
        | CmseNonSecureCall
        | CmseNonSecureEntry
        | Custom
        | RiscvInterruptM
        | RiscvInterruptS
        | RustInvalid
        | Unadjusted => false,
        Rust | RustCall | RustCold | RustPreserveNone => tcx.sess.panic_strategy().unwinds(),
    }
}
