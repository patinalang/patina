use rustc_middle::erl::ErlCallTarget;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{Expr, ExprKind, Thir};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::def_id::LocalDefId;

use crate::errors::FnrefNeedsWitness;

/// Check that every `fnref` names a static Erlang item.
/// If it needs witnesses to run, it can't be used as `fnref`.
pub(crate) fn check_fnrefs(tcx: TyCtxt<'_>, def: LocalDefId) {
    let Ok((thir, expr)) = tcx.thir_body(def) else { return };
    let thir = thir.borrow();
    FnrefVisitor { tcx, owner: def, thir: &thir }.visit_expr(&thir[expr]);
}

struct FnrefVisitor<'thir, 'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: LocalDefId,
    thir: &'thir Thir<'tcx>,
}

impl<'thir, 'tcx> Visitor<'thir, 'tcx> for FnrefVisitor<'thir, 'tcx> {
    fn thir(&self) -> &'thir Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'thir Expr<'tcx>) {
        if let ExprKind::PointerCoercion {
            cast: PointerCoercion::ReifyFnPointer(_), source, ..
        } = expr.kind
            && expr.ty.is_fnref()
            && let ty::FnDef(def_id, args) = *self.thir[source].ty.kind()
            && let ErlCallTarget::Item { def_id, .. } =
                self.tcx.erl_call_target_for_def(self.owner.to_def_id(), def_id, args)
            && self.tcx.erl_needs_witnesses(def_id)
        {
            let item = self.tcx.def_path_str(def_id);
            self.tcx.dcx().emit_err(FnrefNeedsWitness { span: expr.span, item });
        }

        visit::walk_expr(self, expr);
    }
}
