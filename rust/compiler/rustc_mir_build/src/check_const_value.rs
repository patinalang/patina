use rustc_hir::def::DefKind;
use rustc_middle::thir::visit::{self, Visitor};
use rustc_middle::thir::{Expr, ExprKind, Thir};
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::ErrorGuaranteed;
use rustc_span::def_id::LocalDefId;

use crate::errors::UnsupportedConstValueExpr;

/// Check that the value of a const doesn't call a function, or refer to a function or closure.
pub(crate) fn check_const_value(tcx: TyCtxt<'_>, def: LocalDefId) {
    if !matches!(tcx.def_kind(def), DefKind::Const { .. } | DefKind::AssocConst { .. })
        || tcx.is_type_const(def)
    {
        return;
    }

    let Ok((thir, expr)) = tcx.thir_body(def) else { return };
    let thir = thir.borrow();
    ConstValueVisitor { tcx, thir: &thir, error: None }.visit_expr(&thir[expr]);
}

struct ConstValueVisitor<'thir, 'tcx> {
    tcx: TyCtxt<'tcx>,
    thir: &'thir Thir<'tcx>,
    error: Option<ErrorGuaranteed>,
}

impl<'thir, 'tcx> Visitor<'thir, 'tcx> for ConstValueVisitor<'thir, 'tcx> {
    fn thir(&self) -> &'thir Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'thir Expr<'tcx>) {
        if self.error.is_some() {
            return;
        }

        let emit_error = match expr.kind {
            ExprKind::Call { .. } | ExprKind::Closure(_) => true,
            ExprKind::ZstLiteral { .. } if let ty::FnDef(..) = expr.ty.kind() => true,
            _ => false,
        };

        if emit_error {
            self.error =
                Some(self.tcx.dcx().emit_err(UnsupportedConstValueExpr { span: expr.span }));
        }

        visit::walk_expr(self, expr);
    }
}
