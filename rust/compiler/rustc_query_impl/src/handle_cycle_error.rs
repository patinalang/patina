use std::iter;

use rustc_errors::Diag;
use rustc_middle::query::Cycle;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::DefId;

// Default cycle handler used for all queries that don't use the `handle_cycle_error` query
// modifier.
pub(crate) fn default(err: Diag<'_>) -> ! {
    let guar = err.emit();
    guar.raise_fatal()
}

pub(crate) fn fn_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    _: Cycle<'tcx>,
    err: Diag<'_>,
) -> ty::EarlyBinder<'tcx, ty::PolyFnSig<'tcx>> {
    let guar = err.delay_as_bug();

    let err = Ty::new_error(tcx, guar);

    let arity = if let Some(node) = tcx.hir_get_if_local(def_id)
        && let Some(sig) = node.fn_sig()
    {
        sig.decl.inputs.len()
    } else {
        tcx.dcx().abort_if_errors();
        unreachable!()
    };

    ty::EarlyBinder::bind(ty::Binder::dummy(
        tcx.mk_fn_sig_safe_rust_abi(std::iter::repeat_n(err, arity), err),
    ))
}

pub(crate) fn variances_of<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    _cycle: Cycle<'tcx>,
    err: Diag<'_>,
) -> &'tcx [ty::Variance] {
    let _guar = err.delay_as_bug();
    let n = tcx.generics_of(def_id).own_params.len();
    tcx.arena.alloc_from_iter(iter::repeat_n(ty::Bivariant, n))
}
