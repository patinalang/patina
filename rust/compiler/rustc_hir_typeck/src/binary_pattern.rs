//! Types of binary pattern segments, shared by binary patterns in pattern position (`pat.rs`)
//! and in expression position (`expr.rs`).

use rustc_ast::LitKind;
use rustc_errors::ErrorGuaranteed;
use rustc_hir::{self as hir, BinarySegmentFlag, BinarySegmentTy, HirId};
use rustc_middle::erl::binary_pattern::{
    BinaryEncoding, BinaryEndianness, BinarySegmentLayout, BinarySegmentSize,
};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use crate::{FnCtxt, errors};

/// What a binary pattern segment holds, as far as choosing and checking its type is concerned.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SegmentValue<'tcx> {
    /// A binding or `_` in a pattern. It takes the type of the segment.
    Binding,
    /// A literal, possibly negated.
    Lit(&'tcx hir::Lit, bool),
    /// A pinned variable or constant in a pattern, or any expression in a construction.
    Value { ty: Ty<'tcx>, in_pattern: bool },
}

impl<'tcx> SegmentValue<'tcx> {
    pub(crate) fn from_pat(pat: &'tcx hir::Pat<'tcx>, ty: Ty<'tcx>) -> Self {
        match pat.kind {
            hir::PatKind::Binding(..) | hir::PatKind::Wild => SegmentValue::Binding,
            hir::PatKind::Expr(hir::PatExpr {
                kind: hir::PatExprKind::Lit { lit, negated },
                ..
            }) => SegmentValue::Lit(lit, *negated),
            _ => SegmentValue::Value { ty, in_pattern: true },
        }
    }

    pub(crate) fn from_expr(expr: &'tcx hir::Expr<'tcx>, ty: Ty<'tcx>) -> Self {
        match &expr.kind {
            hir::ExprKind::Lit(lit) => SegmentValue::Lit(lit, false),
            hir::ExprKind::Unary(
                hir::UnOp::Neg,
                hir::Expr { kind: hir::ExprKind::Lit(lit), .. },
            ) => SegmentValue::Lit(lit, true),
            _ => SegmentValue::Value { ty, in_pattern: false },
        }
    }
}

pub(crate) fn binary_segment_value_ty<'tcx>(
    tcx: ty::TyCtxt<'tcx>,
    ty: BinarySegmentTy,
) -> Ty<'tcx> {
    match ty {
        BinarySegmentTy::Int => tcx.types.int,
        BinarySegmentTy::Float => tcx.types.float,
        BinarySegmentTy::Char => tcx.types.char,
        BinarySegmentTy::Str => tcx.types.str_,
        BinarySegmentTy::Bin => tcx.types.bin,
        BinarySegmentTy::Bits => tcx.types.bits,
    }
}

/// The segment type used when a segment has no `as ty(..)`, or why `value` can't be a segment
/// without one.
fn default_binary_segment_ty<'tcx>(
    value: SegmentValue<'tcx>,
) -> Result<BinarySegmentTy, DefaultSegmentTyError<'tcx>> {
    match value {
        SegmentValue::Binding => Ok(BinarySegmentTy::Int),
        SegmentValue::Lit(lit, _) => match lit.node {
            LitKind::Int(..) | LitKind::Byte(_) => Ok(BinarySegmentTy::Int),
            LitKind::Float(..) => Ok(BinarySegmentTy::Float),
            LitKind::Char(_) => Ok(BinarySegmentTy::Char),
            LitKind::Str(..) => Ok(BinarySegmentTy::Str),
            LitKind::ByteStr(..) => Ok(BinarySegmentTy::Bin),
            LitKind::Err(guar) => Err(DefaultSegmentTyError::Reported(guar)),
            LitKind::CStr(..) => Err(DefaultSegmentTyError::Literal("C string")),
            LitKind::Bool(_) => Err(DefaultSegmentTyError::Literal("boolean")),
            LitKind::Atom(..) => Err(DefaultSegmentTyError::Literal("atom")),
        },
        SegmentValue::Value { ty, .. } => match *ty.kind() {
            ty::Int => Ok(BinarySegmentTy::Int),
            ty::Float => Ok(BinarySegmentTy::Float),
            ty::Char => Ok(BinarySegmentTy::Char),
            ty::Str => Ok(BinarySegmentTy::Str),
            ty::Bin => Ok(BinarySegmentTy::Bin),
            ty::Bits => Ok(BinarySegmentTy::Bits),
            ty::Error(guar) => Err(DefaultSegmentTyError::Reported(guar)),
            _ => Err(DefaultSegmentTyError::Type(ty)),
        },
    }
}

enum DefaultSegmentTyError<'tcx> {
    Reported(ErrorGuaranteed),
    Literal(&'static str),
    Type(Ty<'tcx>),
}

fn lit_bytes(lit: &hir::Lit) -> Option<&[u8]> {
    match &lit.node {
        LitKind::Str(sym, _) => Some(sym.as_str().as_bytes()),
        LitKind::ByteStr(sym, _) => Some(sym.as_byte_str()),
        _ => None,
    }
}

fn int_lit(expr: &hir::Expr<'_>) -> Option<u128> {
    match expr.kind {
        hir::ExprKind::Lit(hir::Lit { node: LitKind::Int(value), .. }) => Some(value.get()),
        _ => None,
    }
}

/// Are the bytes of a segment with this value and type valid UTF-8 on their own, and so end
/// (and start) on a UTF-8 character boundary of any valid UTF-8 string containing them?
pub(crate) fn is_str_like(value: SegmentValue<'_>, layout: &BinarySegmentLayout<HirId>) -> bool {
    let is_utf8 = layout.encoding() == Some(BinaryEncoding::Utf8);
    match value {
        SegmentValue::Binding => is_utf8,
        SegmentValue::Lit(lit, negated) => match lit.node {
            LitKind::Char(_) => is_utf8,
            // A string literal encoded as UTF-8, or taken as raw bytes by `bin` or `bits`.
            LitKind::Str(..) | LitKind::ByteStr(..) => {
                (is_utf8 || matches!(layout.ty, BinarySegmentTy::Bin | BinarySegmentTy::Bits))
                    && lit_bytes(lit).is_some_and(|bytes| str::from_utf8(bytes).is_ok())
            }
            LitKind::Int(value) => {
                layout.ty == BinarySegmentTy::Int
                    && !negated
                    && layout.size == BinarySegmentSize::Fixed(8)
                    && layout.unit == 1
                    && value.get() < 0x80
            }
            LitKind::Byte(value) => {
                layout.ty == BinarySegmentTy::Int
                    && layout.size == BinarySegmentSize::Fixed(8)
                    && layout.unit == 1
                    && value < 0x80
            }
            _ => false,
        },
        SegmentValue::Value { ty, in_pattern } => match (*ty.kind(), layout.ty) {
            (ty::Char, BinarySegmentTy::Char) => is_utf8,
            (ty::Str, BinarySegmentTy::Str | BinarySegmentTy::Bin | BinarySegmentTy::Bits) => {
                in_pattern || layout.size == BinarySegmentSize::OfValue
            }
            _ => false,
        },
    }
}

impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
    fn str_size_sugg(
        &self,
        spec: &hir::BinarySegmentSpec<'_>,
        index: usize,
        value: SegmentValue<'_>,
    ) -> errors::BinarySegmentStrSizeSugg {
        if !matches!(value, SegmentValue::Lit(..)) {
            return errors::BinarySegmentStrSizeSugg::Bin(spec.ty_span);
        }

        let arg = spec.args[index];
        let span = if spec.args.len() == 1 {
            // `str(2)` becomes `str`.
            spec.span.with_lo(spec.ty_span.hi())
        } else if index > 0 {
            // `str(utf16, 2)` becomes `str(utf16)`.
            arg.span.with_lo(spec.args[index - 1].span.hi())
        } else {
            // `str(2, utf16)` becomes `str(utf16)`.
            arg.span.until(spec.args[1].span)
        };
        errors::BinarySegmentStrSizeSugg::Remove(span)
    }

    pub(crate) fn resolve_binary_segment_layout(
        &self,
        segment_span: Span,
        spec: Option<&'tcx hir::BinarySegmentSpec<'tcx>>,
        value: SegmentValue<'tcx>,
    ) -> Result<BinarySegmentLayout<HirId>, ErrorGuaranteed> {
        let tcx = self.tcx;
        let ty = match spec {
            Some(spec) => spec.ty,
            None => match default_binary_segment_ty(value) {
                Ok(ty) => ty,
                Err(DefaultSegmentTyError::Reported(guar)) => return Err(guar),
                Err(DefaultSegmentTyError::Literal(kind)) => {
                    return Err(self.dcx().emit_err(errors::BinarySegmentUnsupportedLiteral {
                        span: segment_span,
                        kind,
                    }));
                }
                Err(DefaultSegmentTyError::Type(ty)) => {
                    return Err(self.dcx().emit_err(errors::BinarySegmentUnsupportedType {
                        span: segment_span,
                        ty,
                    }));
                }
            },
        };

        let mut result = Ok(());
        let mut size_expr: Option<&'tcx hir::Expr<'tcx>> = None;
        let mut unit_expr: Option<&'tcx hir::Expr<'tcx>> = None;
        let mut signed: Option<(bool, Span)> = None;
        let mut endianness: Option<(BinaryEndianness, Span)> = None;
        let mut encoding: Option<(BinaryEncoding, Span)> = None;

        let not_allowed = |span: Span, arg: &str| {
            self.dcx().emit_err(errors::BinarySegmentArgNotAllowed {
                span,
                arg: arg.to_string(),
                ty: ty.name(),
            })
        };
        let repeated = |span: Span, prev_span: Span, arg: &'static str| {
            self.dcx().emit_err(errors::BinarySegmentRepeatedArg { span, prev_span, arg })
        };

        let args = spec.into_iter().flat_map(|spec| {
            spec.args.iter().enumerate().map(move |(index, arg)| (spec, index, arg))
        });
        for (spec, index, arg) in args {
            let span = arg.span;
            match arg.kind {
                hir::BinarySegmentArgKind::Size(expr) => {
                    self.check_expr_has_type_or_error(expr, tcx.types.int, |_| {});
                    if let Some(prev) = size_expr {
                        result = Err(repeated(expr.span, prev.span, "size"));
                    } else if ty == BinarySegmentTy::Str {
                        let sugg = self.str_size_sugg(spec, index, value);
                        result = Err(self
                            .dcx()
                            .emit_err(errors::BinarySegmentStrSize { span: expr.span, sugg }));
                    } else if ty == BinarySegmentTy::Char {
                        result = Err(not_allowed(expr.span, "size"));
                    } else {
                        size_expr = Some(expr);
                    }
                }
                hir::BinarySegmentArgKind::Unit(expr) => {
                    self.check_expr_has_type_or_error(expr, tcx.types.int, |_| {});
                    if let Some(prev) = unit_expr {
                        result = Err(repeated(expr.span, prev.span, "unit"));
                    } else if matches!(
                        ty,
                        BinarySegmentTy::Float | BinarySegmentTy::Str | BinarySegmentTy::Char
                    ) {
                        result = Err(not_allowed(expr.span, "unit"));
                    } else {
                        unit_expr = Some(expr);
                    }
                }
                hir::BinarySegmentArgKind::Flag(flag) => match flag {
                    BinarySegmentFlag::Signed | BinarySegmentFlag::Unsigned => {
                        if ty != BinarySegmentTy::Int {
                            result = Err(not_allowed(span, flag.name().as_str()));
                        } else if let Some((_, prev)) = signed {
                            result = Err(repeated(span, prev, "signedness"));
                        } else {
                            signed = Some((flag == BinarySegmentFlag::Signed, span));
                        }
                    }
                    BinarySegmentFlag::Big
                    | BinarySegmentFlag::Little
                    | BinarySegmentFlag::Native => {
                        if !matches!(
                            ty,
                            BinarySegmentTy::Int
                                | BinarySegmentTy::Float
                                | BinarySegmentTy::Char
                                | BinarySegmentTy::Str
                        ) {
                            result = Err(not_allowed(span, flag.name().as_str()));
                        } else if let Some((_, prev)) = endianness {
                            result = Err(repeated(span, prev, "endianness"));
                        } else {
                            let value = match flag {
                                BinarySegmentFlag::Big => BinaryEndianness::Big,
                                BinarySegmentFlag::Little => BinaryEndianness::Little,
                                _ => BinaryEndianness::Native,
                            };
                            endianness = Some((value, span));
                        }
                    }
                    BinarySegmentFlag::Utf8
                    | BinarySegmentFlag::Utf16
                    | BinarySegmentFlag::Utf32 => {
                        if !matches!(ty, BinarySegmentTy::Char | BinarySegmentTy::Str) {
                            result = Err(not_allowed(span, flag.name().as_str()));
                        } else if ty == BinarySegmentTy::Str
                            && flag != BinarySegmentFlag::Utf8
                            && !matches!(
                                value,
                                SegmentValue::Lit(hir::Lit { node: LitKind::Str(..), .. }, _)
                            )
                        {
                            result =
                                Err(self.dcx().emit_err(errors::BinarySegmentStrEncoding { span }));
                        } else if let Some((_, prev)) = encoding {
                            result = Err(repeated(span, prev, "encoding"));
                        } else {
                            let value = match flag {
                                BinarySegmentFlag::Utf8 => BinaryEncoding::Utf8,
                                BinarySegmentFlag::Utf16 => BinaryEncoding::Utf16,
                                _ => BinaryEncoding::Utf32,
                            };
                            encoding = Some((value, span));
                        }
                    }
                },
            }
        }

        let encoding = encoding.map_or(BinaryEncoding::Utf8, |(encoding, _)| encoding);
        if let (BinaryEncoding::Utf8, Some((_, span))) = (encoding, endianness)
            && matches!(ty, BinarySegmentTy::Char | BinarySegmentTy::Str)
        {
            result = Err(self.dcx().emit_err(errors::BinarySegmentUtf8Endianness { span }));
        }

        let unit = match unit_expr {
            Some(expr) => match int_lit(expr) {
                Some(unit @ 1..=256) => {
                    if ty == BinarySegmentTy::Bin && unit % 8 != 0 {
                        result = Err(self
                            .dcx()
                            .emit_err(errors::BinarySegmentBinUnit { span: expr.span }));
                    }
                    unit as u16
                }
                _ => {
                    result =
                        Err(self.dcx().emit_err(errors::BinarySegmentUnit { span: expr.span }));
                    1
                }
            },
            None => match ty {
                BinarySegmentTy::Str | BinarySegmentTy::Bin => 8,
                BinarySegmentTy::Int
                | BinarySegmentTy::Float
                | BinarySegmentTy::Char
                | BinarySegmentTy::Bits => 1,
            },
        };

        let literal_bytes = match value {
            SegmentValue::Lit(lit, _) => lit_bytes(lit),
            SegmentValue::Binding | SegmentValue::Value { .. } => None,
        };
        let size = if let Some(expr) = size_expr {
            if literal_bytes.is_some() {
                result =
                    Err(self.dcx().emit_err(errors::BinarySegmentLiteralSize { span: expr.span }));
            }
            // A literal size too large for a `u64` is left for Erlang to evaluate.
            match (ty, int_lit(expr).and_then(|size| u64::try_from(size).ok())) {
                (BinarySegmentTy::Float, Some(size @ (16 | 32 | 64))) => {
                    BinarySegmentSize::Fixed(size)
                }
                (BinarySegmentTy::Float, _) => {
                    result = Err(self
                        .dcx()
                        .emit_err(errors::BinarySegmentFloatSize { span: expr.span }));
                    BinarySegmentSize::Fixed(64)
                }
                (_, Some(size)) => BinarySegmentSize::Fixed(size),
                (_, None) => BinarySegmentSize::Dynamic(expr.hir_id),
            }
        } else {
            match (ty, value) {
                (BinarySegmentTy::Int, _) => BinarySegmentSize::Fixed(8),
                (BinarySegmentTy::Float, _) => BinarySegmentSize::Fixed(64),
                (BinarySegmentTy::Char, _) => BinarySegmentSize::Encoded(encoding),
                // A string literal is text in a `str` segment, and bytes elsewhere.
                (
                    BinarySegmentTy::Str,
                    SegmentValue::Lit(hir::Lit { node: LitKind::Str(..), .. }, _),
                ) => BinarySegmentSize::Encoded(encoding),
                (_, SegmentValue::Lit(..)) if let Some(bytes) = literal_bytes => {
                    let bits = bytes.len() as u64 * 8;
                    if bits % u64::from(unit) != 0 {
                        result = Err(self
                            .dcx()
                            .emit_err(errors::BinarySegmentLiteralUnit { span: segment_span }));
                    }
                    BinarySegmentSize::Fixed(bits / u64::from(unit))
                }
                (
                    BinarySegmentTy::Str | BinarySegmentTy::Bin | BinarySegmentTy::Bits,
                    SegmentValue::Binding,
                ) => BinarySegmentSize::Unsized,
                (BinarySegmentTy::Str | BinarySegmentTy::Bin | BinarySegmentTy::Bits, _) => {
                    BinarySegmentSize::OfValue
                }
            }
        };

        result.map(|()| BinarySegmentLayout {
            ty,
            size,
            unit,
            signed: signed.is_some_and(|(signed, _)| signed),
            endianness: endianness.map_or(BinaryEndianness::Big, |(endianness, _)| endianness),
        })
    }
}
