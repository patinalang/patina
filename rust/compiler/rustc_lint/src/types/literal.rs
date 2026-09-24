use rustc_apfloat::Float;
use rustc_apfloat::ieee::{DoubleS, HalfS, IeeeFloat, Semantics, SingleS};
use rustc_ast as ast;
use rustc_hir as hir;
use rustc_hir::HirId;
use rustc_middle::erl::binary_pattern::{BinarySegmentLayout, BinarySegmentSize};
use rustc_middle::{bug, ty};
use rustc_span::{Span, Symbol};

use crate::LateContext;
use crate::context::LintContext;
use crate::lints::{OverflowingBitSegment, OverflowingLiteral};
use crate::types::{OVERFLOWING_BINARY_PATTERN_LITERALS, OVERFLOWING_LITERALS, TypeLimits};

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

/// Lints a literal that doesn't fit the fixed size of its binary pattern segment, like `256:8`.
pub(crate) fn lint_binary_segment_literal(
    cx: &LateContext<'_>,
    layout: &BinarySegmentLayout<HirId>,
    lit: &hir::Lit,
    negated: bool,
    span: Span,
    in_pattern: bool,
) {
    let BinarySegmentSize::Fixed(size) = layout.size else { return };
    let bits = u128::from(size) * u128::from(layout.unit);
    let (ty, fits) = match (layout.ty, lit.node) {
        (hir::BinarySegmentTy::Int, ast::LitKind::Int(_) | ast::LitKind::Byte(_)) => {
            let value = match lit.node {
                ast::LitKind::Int(value) => value.get(),
                ast::LitKind::Byte(value) => value.into(),
                _ => unreachable!(),
            };
            if layout.signed {
                let fits = match bits {
                    0 => value == 0,
                    129.. => true,
                    _ => {
                        let max = 1u128 << (bits - 1);
                        if negated { value <= max } else { value < max }
                    }
                };
                ("a signed integer", fits)
            } else {
                let fits = (!negated || value == 0) && (bits >= 128 || value < 1u128 << bits);
                ("an unsigned integer", fits)
            }
        }
        (hir::BinarySegmentTy::Float, ast::LitKind::Float(value, _)) => {
            let is_infinite = match bits {
                16 => float_is_infinite::<HalfS>(value),
                32 => float_is_infinite::<SingleS>(value),
                _ => None,
            };
            ("a float", is_infinite != Some(true))
        }
        _ => return,
    };
    if fits {
        return;
    }

    let consequence = match (in_pattern, layout.ty) {
        (true, _) => "this segment can never match",
        (false, hir::BinarySegmentTy::Float) => "the value is encoded as infinity",
        (false, _) => "the value is truncated to its lowest bits",
    };
    let lit = cx.sess().source_map().span_to_snippet(lit.span).unwrap_or_default();
    let lit = if negated && !lit.starts_with('-') { format!("-{lit}") } else { lit };
    cx.emit_span_lint(
        OVERFLOWING_BINARY_PATTERN_LITERALS,
        span,
        OverflowingBitSegment { lit, ty, bits, consequence },
    );
}
