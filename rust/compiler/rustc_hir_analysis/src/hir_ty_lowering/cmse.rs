use rustc_abi::ExternAbi;
use rustc_errors::{DiagCtxtHandle, E0781, struct_span_code_err};
use rustc_hir::{self as hir, HirId};
use rustc_middle::bug;
use rustc_middle::ty::layout::{LayoutError, ty_size_and_align_for_frontend};
use rustc_middle::ty::{self, TyCtxt, TypeVisitableExt};
use rustc_span::Span;

use crate::errors;

/// Check conditions on inputs and outputs that the cmse ABIs impose: arguments and results MUST be
/// returned via registers (i.e. MUST NOT spill to the stack). LLVM will also validate these
/// conditions, but by checking them here rustc can emit nicer error messages.
pub(crate) fn validate_cmse_abi<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    hir_id: HirId,
    abi: ExternAbi,
    fn_sig: ty::PolyFnSig<'tcx>,
) {
    let fn_decl = match abi {
        ExternAbi::CmseNonSecureCall => match tcx.hir_node(hir_id) {
            hir::Node::Ty(hir::Ty { kind: hir::TyKind::FnPtr(fn_ptr_ty), .. }) => fn_ptr_ty.decl,
            _ => {
                let span = match tcx.parent_hir_node(hir_id) {
                    hir::Node::Item(hir::Item {
                        kind: hir::ItemKind::ForeignMod { .. },
                        span,
                        ..
                    }) => *span,
                    _ => tcx.hir_span(hir_id),
                };
                struct_span_code_err!(
                    dcx,
                    span,
                    E0781,
                    "the `\"cmse-nonsecure-call\"` ABI is only allowed on function pointers"
                )
                .emit();
                return;
            }
        },
        ExternAbi::CmseNonSecureEntry => {
            let Some(hir::FnSig { decl, .. }) = tcx.hir_node(hir_id).fn_sig() else {
                return;
            };

            decl
        }
        _ => return,
    };

    if let Err((span, layout_err)) = is_valid_cmse_inputs(tcx, dcx, fn_sig, fn_decl, abi) {
        if should_emit_layout_error(abi, layout_err) {
            dcx.emit_err(errors::CmseGeneric { span, abi });
        }
    }

    if let Err(layout_err) = is_valid_cmse_output(tcx, dcx, fn_sig, fn_decl, abi) {
        if should_emit_layout_error(abi, layout_err) {
            dcx.emit_err(errors::CmseGeneric { span: fn_decl.output.span(), abi });
        }
    }
}

fn is_valid_cmse_inputs<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    fn_sig: ty::PolyFnSig<'tcx>,
    fn_decl: &hir::FnDecl<'tcx>,
    abi: ExternAbi,
) -> Result<(), (Span, &'tcx LayoutError<'tcx>)> {
    let mut accum = 0u64;
    let mut excess_argument_spans = Vec::new();

    let fn_sig = tcx.instantiate_bound_regions_with_erased(fn_sig);
    let fn_sig = tcx.erase_and_anonymize_regions(fn_sig);

    for (ty, hir_ty) in fn_sig.inputs().iter().zip(fn_decl.inputs) {
        if ty.has_infer_types() {
            let err = LayoutError::Unknown(*ty);
            return Err((hir_ty.span, tcx.arena.alloc(err)));
        }

        let Some((size, align)) = ty_size_and_align_for_frontend(tcx, *ty) else {
            let err = LayoutError::Unknown(*ty);
            return Err((hir_ty.span, tcx.arena.alloc(err)));
        };

        accum += size.bytes();
        accum = accum.next_multiple_of(Ord::max(4, align.bytes()));

        if accum > 16 {
            excess_argument_spans.push(hir_ty.span);
        }
    }

    if !excess_argument_spans.is_empty() {
        dcx.emit_err(errors::CmseInputsStackSpill { spans: excess_argument_spans, abi });
    }

    Ok(())
}

fn is_valid_cmse_output<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    fn_sig: ty::PolyFnSig<'tcx>,
    fn_decl: &hir::FnDecl<'tcx>,
    abi: ExternAbi,
) -> Result<(), &'tcx LayoutError<'tcx>> {
    let fn_sig = tcx.instantiate_bound_regions_with_erased(fn_sig);
    let fn_sig = tcx.erase_and_anonymize_regions(fn_sig);
    let return_type = fn_sig.output();

    if abi == ExternAbi::CmseNonSecureEntry && return_type.has_opaque_types() {
        dcx.emit_err(errors::CmseImplTrait { span: fn_decl.output.span(), abi });
        return Ok(());
    }

    if return_type.has_infer_types() {
        let err = LayoutError::Unknown(return_type);
        return Err(tcx.arena.alloc(err));
    }

    let Some((size, _)) = ty_size_and_align_for_frontend(tcx, return_type) else {
        let err = LayoutError::Unknown(return_type);
        return Err(tcx.arena.alloc(err));
    };

    if !is_valid_cmse_output_size(size.bytes()) {
        dcx.emit_err(errors::CmseOutputStackSpill { span: fn_decl.output.span(), abi });
    }

    Ok(())
}

fn is_valid_cmse_output_size(size: u64) -> bool {
    size <= 4 || size == 8
}

fn should_emit_layout_error<'tcx>(abi: ExternAbi, layout_err: &'tcx LayoutError<'tcx>) -> bool {
    match layout_err {
        LayoutError::TooGeneric(ty) => match abi {
            ExternAbi::CmseNonSecureCall => !ty.has_opaque_types(),
            ExternAbi::CmseNonSecureEntry => true,
            _ => bug!("invalid ABI: {abi}"),
        },
        LayoutError::Unknown(_) => false,
    }
}
