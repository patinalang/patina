//! Translation of binary patterns, in pattern and expression position, to Erlang's bit syntax.

use rustc_data_structures::smallvec::{SmallVec, smallvec};
use rustc_hir::BinarySegmentTy;
use rustc_middle::erl::binary_pattern::{
    BinaryEncoding, BinaryEndianness, BinarySegmentLayout, BinarySegmentSize,
};
use rustc_middle::erl::etir::{self, ExprId, Lit};
use rustc_middle::{bug, span_bug};
use rustc_span::{Span, Symbol, sym};

use crate::compile::translate::body::{BodyCx, Patterns};
use crate::erlang::abstract_format as eaf;

type Layout = BinarySegmentLayout<ExprId>;

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    /// Translates `<<len:8, payload: bin(len), rest: bin>>` to
    /// `<<Len:8/integer, Payload:Len/binary, Rest/binary>>`.
    pub(super) fn translate_pat_binary_pattern(
        &mut self,
        span: Span,
        segments: &[etir::BinarySegmentPat],
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let mut elements = SmallVec::<[_; 8]>::new();
        for segment in segments {
            let layout = &segment.layout;
            let pattern_kind = &self.etir[segment.pattern].kind;

            if let etir::PatKind::Literal(lit) = pattern_kind
                && let Some(splice) = literal_splice(lit, layout)
            {
                match splice {
                    LiteralSplice::Text { text, tsl } => {
                        let value = afb.pat_string(segment.span, text.as_str());
                        elements.push(afb.pat_binary_element(segment.span, value, None, Some(tsl)));
                    }
                    LiteralSplice::Bytes(bytes) => elements.extend(bytes.iter().map(|&byte| {
                        let byte = afb.pat_int(segment.span, byte.into());
                        afb.pat_binary_element(segment.span, byte, None, None::<[eaf::Tsl; 0]>)
                    })),
                }
                continue;
            }

            // Translate the size first: it can only refer to bindings of earlier segments.
            let size = if let BinarySegmentSize::OfValue = layout.size {
                // A pinned bitstring with no explicit size matches exactly the pinned value. Its
                // size is always explicit, since Erlang only allows an unsized last segment.
                let etir::PatKind::Pin { var } = pattern_kind else {
                    span_bug!(segment.span, "sized-by-value segment isn't a pin")
                };
                let value = self.translate_var_ref(segment.span, *var);
                Some(self.size_of_value_expr(segment.span, value, layout))
            } else {
                self.segment_size(segment.span, layout)
            };
            let value = self.translate_single_pattern(segment.pattern);
            let tsl = segment_tsl(layout);
            elements.push(self.translator.afb.pat_binary_element(
                segment.span,
                value,
                size,
                Some(tsl),
            ));
        }
        self.translator.afb.pat_binary(span, elements).into()
    }

    /// Translates `<<len:8, payload: bin>>` to `<<Len:8/integer, Payload/binary>>`.
    pub(super) fn translate_expr_binary_pattern(
        &mut self,
        span: Span,
        segments: &[etir::BinarySegmentExpr],
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let mut elements = SmallVec::<[_; 8]>::new();
        for segment in segments {
            let layout = &segment.layout;

            if let etir::ExprKind::Literal(lit, _) = &self.etir[segment.value].kind
                && let Some(splice) = literal_splice(lit, layout)
            {
                match splice {
                    LiteralSplice::Text { text, tsl } => {
                        let value = afb.expr_string(segment.span, text);
                        elements.push(afb.expr_binary_element(
                            segment.span,
                            value,
                            None,
                            Some(tsl),
                        ));
                    }
                    LiteralSplice::Bytes(bytes) => elements.extend(bytes.iter().map(|&byte| {
                        let byte = afb.expr_int(segment.span, byte.into());
                        afb.expr_binary_element(segment.span, byte, None, None::<[eaf::Tsl; 0]>)
                    })),
                }
                continue;
            }

            let value = self.translate_expr(segment.value);
            let size = self.segment_size(segment.span, layout);
            let tsl = segment_tsl(layout);
            elements.push(afb.expr_binary_element(segment.span, value, size, Some(tsl)));
        }
        afb.expr_binary(span, elements)
    }

    fn segment_size(&mut self, span: Span, layout: &Layout) -> Option<eaf::BinElemSize<'arena>> {
        let afb = &self.translator.afb;

        let size = match layout.size {
            BinarySegmentSize::Fixed(size) => afb.expr_int(span, size.into()),
            BinarySegmentSize::Dynamic(size) => self.translate_expr(size),
            BinarySegmentSize::OfValue
            | BinarySegmentSize::Unsized
            | BinarySegmentSize::Encoded(_) => return None,
        };
        Some(afb.bin_elem_size(size))
    }

    fn size_of_value_expr(
        &self,
        span: Span,
        value: eaf::Expr<'arena>,
        layout: &Layout,
    ) -> eaf::BinElemSize<'arena> {
        let afb = &self.translator.afb;
        let erlang_call = |name| afb.erlang_bif(span, name, [value]);
        let size = match layout.unit {
            1 => erlang_call(sym::bit_size),
            8 => erlang_call(sym::byte_size),
            unit => afb.expr_binary_op(
                span,
                eaf::BinaryOp::DivInt,
                erlang_call(sym::bit_size),
                afb.expr_int(span, unit.into()),
            ),
        };
        afb.bin_elem_size(size)
    }
}

enum LiteralSplice<'l> {
    Text { text: Symbol, tsl: SmallVec<[eaf::Tsl; 4]> },
    Bytes(&'l [u8]),
}

/// A string literal segment must be written as a charlist (or as ints) to use /utf* suffixes.
/// Separately, in patterns, a binary literal is not a valid binary pattern segment.
/// For these reasons, this function decides whether the literal should be spliced into the binary,
/// e.g.  << "png"  >> in Patina becomes << "png"/utf8 >> in Erlang,
/// while << b"png" >> in Patina becomes << 80, 78, 71 >>.
fn literal_splice<'lit>(lit: &'lit Lit, layout: &Layout) -> Option<LiteralSplice<'lit>> {
    match lit {
        &Lit::Str(text) => {
            let tsl = match layout.size {
                BinarySegmentSize::Encoded(_) => segment_tsl(layout),
                _ => smallvec![eaf::Tsl::Utf8],
            };
            Some(LiteralSplice::Text { text, tsl })
        }
        Lit::ByteStr(bytes) => Some(LiteralSplice::Bytes(bytes.as_byte_str())),
        _ => None,
    }
}

/// The Erlang type specifier list of a segment, like `[integer, signed, little]`.
fn segment_tsl(layout: &Layout) -> SmallVec<[eaf::Tsl; 4]> {
    let mut tsl = SmallVec::new();
    let default_unit = match (layout.ty, layout.size) {
        (_, BinarySegmentSize::Encoded(encoding)) => {
            tsl.push(match encoding {
                BinaryEncoding::Utf8 => eaf::Tsl::Utf8,
                BinaryEncoding::Utf16 => eaf::Tsl::Utf16,
                BinaryEncoding::Utf32 => eaf::Tsl::Utf32,
            });
            None
        }
        (BinarySegmentTy::Char, _) => bug!("`char` segment without an encoding"),
        (BinarySegmentTy::Int, _) => {
            tsl.push(eaf::Tsl::Integer);
            if layout.signed {
                tsl.push(eaf::Tsl::Signed);
            }
            Some(1)
        }
        (BinarySegmentTy::Float, _) => {
            tsl.push(eaf::Tsl::Float);
            None
        }
        (BinarySegmentTy::Str | BinarySegmentTy::Bin, _) => {
            tsl.push(eaf::Tsl::Binary);
            Some(8)
        }
        // Erlang's `bitstring` fixes the unit at 1; `binary-unit:N` is a bitstring of any unit.
        (BinarySegmentTy::Bits, _) if layout.unit != 1 => {
            tsl.push(eaf::Tsl::Binary);
            Some(8)
        }
        (BinarySegmentTy::Bits, _) => {
            tsl.push(eaf::Tsl::Bitstring);
            Some(1)
        }
    };
    match layout.endianness {
        BinaryEndianness::Big => {}
        BinaryEndianness::Little => tsl.push(eaf::Tsl::Little),
        BinaryEndianness::Native => tsl.push(eaf::Tsl::Native),
    }
    if let Some(default_unit) = default_unit
        && layout.unit != default_unit
    {
        tsl.push(eaf::Tsl::Unit { size: layout.unit.into() });
    }
    tsl
}
