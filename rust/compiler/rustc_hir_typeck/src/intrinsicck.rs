use rustc_hir::HirId;
use rustc_middle::ty::{self, Ty, TyCtxt, Unnormalized};
use rustc_span::def_id::LocalDefId;
use tracing::trace;

fn check_transmute<'tcx>(
    tcx: TyCtxt<'tcx>,
    typing_env: ty::TypingEnv<'tcx>,
    from: Ty<'tcx>,
    to: Ty<'tcx>,
    hir_id: HirId,
) {
    let span = || tcx.hir_span(hir_id);
    let normalize = |ty| {
        if let Ok(ty) = tcx.try_normalize_erasing_regions(typing_env, Unnormalized::new_wip(ty)) {
            ty
        } else {
            Ty::new_error_with_message(
                tcx,
                span(),
                format!("tried to normalize non-wf type {ty:#?} in check_transmute"),
            )
        }
    };

    let from = normalize(from);
    let to = normalize(to);
    trace!(?from, ?to);

    // Transmutes that are only changing lifetimes are always ok.
    if from == to {
        return;
    }
}

pub(crate) fn check_transmutes(tcx: TyCtxt<'_>, owner: LocalDefId) {
    assert!(!tcx.is_typeck_child(owner.to_def_id()));
    let typeck_results = tcx.typeck(owner);
    let None = typeck_results.tainted_by_errors else { return };

    let typing_env = ty::TypingEnv::post_analysis(tcx, owner);
    for &(from, to, hir_id) in &typeck_results.transmutes_to_check {
        check_transmute(tcx, typing_env, from, to, hir_id);
    }
}
