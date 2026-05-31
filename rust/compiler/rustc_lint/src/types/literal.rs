use rustc_apfloat::Float;
use rustc_apfloat::ieee::{DoubleS, IeeeFloat, Semantics};
use rustc_ast as ast;
use rustc_hir as hir;
use rustc_hir::HirId;
use rustc_middle::{bug, ty};
use rustc_span::{Span, Symbol};

use crate::LateContext;
use crate::context::LintContext;
use crate::lints::OverflowingLiteral;
use crate::types::{OVERFLOWING_LITERALS, TypeLimits};

/// `None` if `v` does not parse as the float type, otherwise indicates whether a literal rounds
/// to infinity.
fn float_is_infinite<S: Semantics>(v: Symbol) -> Option<bool> {
    let x: IeeeFloat<S> = v.as_str().parse().ok()?;
    Some(x.is_infinite())
}

pub(crate) fn lint_literal<'tcx>(
    cx: &LateContext<'tcx>,
    _type_limits: &TypeLimits,
    hir_id: HirId,
    span: Span,
    lit: &hir::Lit,
    _negated: bool,
) {
    match *cx.typeck_results().node_type(hir_id).kind() {
        ty::Int => {}
        ty::Float => {
            let ast::LitKind::Float(v, _) = lit.node else {
                bug!();
            };

            let is_infinite = float_is_infinite::<DoubleS>(v);

            if is_infinite == Some(true) {
                cx.emit_span_lint(
                    OVERFLOWING_LITERALS,
                    span,
                    OverflowingLiteral {
                        ty: "float",
                        lit: cx
                            .sess()
                            .source_map()
                            .span_to_snippet(lit.span)
                            .unwrap_or_else(|_| v.to_string()),
                    },
                );
            }
        }
        _ => {}
    }
}
