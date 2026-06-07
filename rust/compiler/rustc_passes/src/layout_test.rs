use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Span;
use rustc_trait_selection::error_reporting::InferCtxtErrorExt;
use rustc_trait_selection::infer::TyCtxtInferExt;
use rustc_trait_selection::traits;

pub fn test_layout(tcx: TyCtxt<'_>) {
    if !tcx.features().rustc_attrs() {
        // if the `rustc_attrs` feature is not enabled, don't bother testing layout
        return;
    }
}

pub fn ensure_wf<'tcx>(
    tcx: TyCtxt<'tcx>,
    typing_env: ty::TypingEnv<'tcx>,
    ty: Ty<'tcx>,
    def_id: LocalDefId,
    span: Span,
) -> bool {
    let (infcx, param_env) = tcx.infer_ctxt().build_with_typing_env(typing_env);
    let ocx = traits::ObligationCtxt::new_with_diagnostics(&infcx);
    let pred = ty::ClauseKind::WellFormed(ty.into());
    let obligation = traits::Obligation::new(
        tcx,
        traits::ObligationCause::new(
            span,
            def_id,
            traits::ObligationCauseCode::WellFormed(Some(traits::WellFormedLoc::Ty(def_id))),
        ),
        param_env,
        pred,
    );
    ocx.register_obligation(obligation);
    let errors = ocx.evaluate_obligations_error_on_ambiguity();
    if !errors.is_empty() {
        infcx.err_ctxt().report_fulfillment_errors(errors);
        false
    } else {
        // looks WF!
        true
    }
}
