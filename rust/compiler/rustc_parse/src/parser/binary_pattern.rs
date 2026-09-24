//! Parsing of binary patterns, in pattern and expression position: `<<len:8, payload: bin(len), rest @ ..>>`.

use rustc_ast::token::{self, Token};
use rustc_ast::tokenstream::Spacing;
use rustc_ast::{
    BinarySegment, BinarySegmentArg, BinarySegmentArgKind, BinarySegmentFlag, BinarySegmentSpec,
    BinarySegmentTy, BindingMode, Expr, ExprKind, Pat, PatBinarySegment, PatBinarySegmentKind,
    PatKind,
};
use rustc_errors::{Applicability, PResult};
use rustc_span::{BytePos, Span, sym};
use thin_vec::{ThinVec, thin_vec};

use super::attr_wrapper::AttrWrapper;
use super::{Parser, Restrictions};
use crate::{errors, exp};

impl<'a> Parser<'a> {
    /// Is the current token the `<<` opening a binary pattern?
    ///
    /// Only meaningful where an operand starts: in infix position `<<` is a left shift.
    pub(super) fn is_binary_pattern(&self) -> bool {
        self.token == token::Shl
    }

    /// Parses a binary pattern: `<<kind:8, len, payload: bin(len), rest @ ..>>`.
    pub(super) fn parse_pat_binary_pattern(&mut self) -> PResult<'a, PatKind> {
        let mut segments = self.parse_binary_segments(|this| {
            let lo = this.token.span;
            if this.eat(exp!(DotDot)) {
                let pat = Box::new(this.mk_pat(lo, PatKind::Wild));
                return Ok(PatBinarySegment { pat, kind: PatBinarySegmentKind::Rest, span: lo });
            }
            let pat = this.parse_binary_segment_pat()?;
            if let PatKind::Ident(..) = pat.kind
                && this.token == token::At
                && this.look_ahead(1, |t| *t == token::DotDot)
            {
                this.bump(); // `@`
                this.bump(); // `..`
                let span = lo.to(this.prev_token.span);
                return Ok(PatBinarySegment { pat, kind: PatBinarySegmentKind::Rest, span });
            }
            let kind = match this.parse_binary_segment_spec()? {
                Some(spec) => PatBinarySegmentKind::Spec(spec),
                None => PatBinarySegmentKind::Default,
            };
            Ok(PatBinarySegment { pat, kind, span: lo.to(this.prev_token.span) })
        })?;
        if let Some((_, init)) = segments.split_last_mut() {
            for segment in init {
                if let PatBinarySegmentKind::Rest = segment.kind {
                    let guar = self
                        .dcx()
                        .emit_err(errors::BinarySegmentRestNotLast { span: segment.span });
                    segment.kind = PatBinarySegmentKind::Recovered(guar);
                }
            }
        }
        Ok(PatKind::BinaryPattern(segments))
    }

    /// Parses a binary pattern expression: `<<kind:8, payload: bin, 1.5: float(32)>>`.
    pub(super) fn parse_expr_binary_pattern(&mut self) -> PResult<'a, Box<Expr>> {
        let lo = self.token.span;
        let segments = self.parse_binary_segments(|this| {
            let lo = this.token.span;
            // Segment values are unary-level expressions, so that `>>`, `,` and `:` end them.
            // Binary operators and casts need parentheses: `<<(a >> 2):8>>`.
            let expr = this.with_res(Restrictions::BINARY_SEGMENT, |this| {
                this.parse_expr_prefix(AttrWrapper::empty())
            })?;
            let spec = this.parse_binary_segment_spec()?;
            Ok(BinarySegment { expr, spec, span: lo.to(this.prev_token.span) })
        })?;
        Ok(self.mk_expr(lo.to(self.prev_token.span), ExprKind::BinaryPattern(segments)))
    }

    /// Parses `<< segment, ... >>`, where the current token is `<<`.
    fn parse_binary_segments<T>(
        &mut self,
        mut parse_segment: impl FnMut(&mut Self) -> PResult<'a, T>,
    ) -> PResult<'a, ThinVec<T>> {
        self.bump(); // `<<`
        self.with_res(Restrictions::empty(), |this| {
            let mut segments = ThinVec::new();
            if this.eat_binary_pattern_close() {
                return Ok(segments);
            }
            loop {
                segments.push(parse_segment(this)?);
                if this.eat_binary_pattern_close() {
                    return Ok(segments);
                }
                this.expect(exp!(Comma))?;
                // Allow a trailing comma.
                if this.eat_binary_pattern_close() {
                    return Ok(segments);
                }
            }
        })
    }

    /// Eats the closing `>>`, splitting it off glued tokens like `>>=`.
    fn eat_binary_pattern_close(&mut self) -> bool {
        if self.token != token::ShrEq {
            return self.break_and_eat(exp!(Shr));
        }

        let span = self.token.span;
        let spacing = self.token_spacing;
        let close = Token::new(token::Shr, span.with_hi(span.lo() + BytePos(2)));
        let rest = Token::new(token::Eq, span.with_lo(close.span.hi()));
        let glued = (spacing == Spacing::Joint)
            .then(|| self.look_ahead(1, |next| rest.glue(next).map(|t| (t, next.span))))
            .flatten();

        self.token = close;
        self.bump_with((rest, spacing));
        if let Some((glued, next_span)) = glued {
            // Consume the leftover `=` and the token after it as one glued token, so that
            // `<<x>>=> x` and `<<1>>==<<1>>` parse. This bumps the parser past the token after
            // `>>=`, so the `>>` is no longer the current token for token capturing.
            self.bump();
            self.prev_token = close;
            self.token = Token::new(glued.kind, rest.span.to(next_span));
            self.break_last_token_glued = true;
        }
        // Both bytes of the `>>` were split off the `>>=`.
        self.break_last_token = 2;
        true
    }

    /// Parses the value part of a binary pattern segment: a binding (`_`, `x`, `mut x`), a
    /// literal (`1`, `-1`, `1.5`, `'c'`, `"str"`, `b"bin"`) or a pinned variable (`^x`).
    fn parse_binary_segment_pat(&mut self) -> PResult<'a, Box<Pat>> {
        let lo = self.token.span;
        let kind = if self.eat_keyword(exp!(Underscore)) {
            PatKind::Wild
        } else if self.eat_noexpect(&token::Caret) {
            self.parse_pat_pin()?
        } else if self.eat_keyword(exp!(Mut)) {
            PatKind::Ident(BindingMode::MUT, self.parse_ident()?, None)
        } else if self.check_ident() && !self.token.is_bool_lit() {
            PatKind::Ident(BindingMode::NONE, self.parse_ident()?, None)
        } else if matches!(self.token.kind, token::Literal(_) | token::Minus)
            || self.token.is_bool_lit()
        {
            PatKind::Expr(self.parse_literal_maybe_minus()?)
        } else {
            return Err(self.dcx().create_err(errors::BinarySegmentExpectedPattern {
                span: self.token.span,
                token: super::token_descr(&self.token),
            }));
        };
        Ok(Box::new(self.mk_pat(lo.to(self.prev_token.span), kind)))
    }

    /// Parses the optional type of a binary pattern segment: `: ty(args)`, `: ty` without
    /// arguments, or `:n` with an integer literal `n`, short for `: int(n)`.
    fn parse_binary_segment_spec(&mut self) -> PResult<'a, Option<BinarySegmentSpec>> {
        let lo = self.token.span;
        if !self.eat(exp!(Colon)) {
            return Ok(None);
        } else if let token::Literal(token::Lit { kind: token::LitKind::Integer, .. }) =
            self.token.kind
        {
            let ty_span = self.prev_token.span.shrink_to_hi();
            let size = self.parse_literal_maybe_minus()?;
            return Ok(Some(self.short_binary_segment_spec(lo, ty_span, size)));
        } else if self.token.ident().is_some_and(|(ident, _)| {
            BinarySegmentTy::from_symbol(ident.name).is_some()
                || self.look_ahead(1, |t| *t == token::OpenParen)
        }) {
            let ty_ident = self.parse_ident()?;
            let ty = match BinarySegmentTy::from_symbol(ty_ident.name) {
                Some(ty) => ty,
                None => {
                    return Err(self.dcx().create_err(errors::BinarySegmentUnknownType {
                        span: ty_ident.span,
                        ty: ty_ident,
                    }));
                }
            };
            let args = if self.check(exp!(OpenParen)) {
                self.parse_paren_comma_seq(|this| this.parse_binary_segment_arg())?.0
            } else {
                ThinVec::new()
            };
            return Ok(Some(BinarySegmentSpec {
                ty,
                ty_span: ty_ident.span,
                args,
                span: lo.to(self.prev_token.span),
            }));
        }

        let colon_span = self.prev_token.span;
        let size = self.parse_expr_prefix(AttrWrapper::empty())?;
        let is_name = matches!(
            &size.kind,
            ExprKind::Path(None, path)
                if path.segments.len() == 1 && path.segments[0].args.is_none()
        );
        let suggestion = errors::BinarySegmentSizeSugg {
            open: colon_span.between(size.span),
            close: size.span.shrink_to_hi(),
            applicability: if is_name {
                Applicability::MaybeIncorrect
            } else {
                Applicability::MachineApplicable
            },
        };
        let span = size.span;
        let guar =
            self.dcx().emit_err(errors::BinarySegmentExpectedSizeOrType { span, suggestion });
        // The name might be a mistyped type rather than a size, so don't resolve it.
        let size = if is_name { self.mk_expr_err(span, guar) } else { size };
        let ty_span = colon_span.shrink_to_hi();
        Ok(Some(self.short_binary_segment_spec(lo, ty_span, size)))
    }

    /// Builds the type of the short form `x:n`, which is `x: int(n)`.
    fn short_binary_segment_spec(
        &self,
        lo: Span,
        ty_span: Span,
        size: Box<Expr>,
    ) -> BinarySegmentSpec {
        let span = size.span;
        BinarySegmentSpec {
            ty: BinarySegmentTy::Int,
            ty_span,
            args: thin_vec![BinarySegmentArg { kind: BinarySegmentArgKind::Size(size), span }],
            span: lo.to(self.prev_token.span),
        }
    }

    /// Parses an argument of a binary pattern segment type: `size = expr`, `unit = expr`, a flag
    /// like `little`, or a bare size expression.
    fn parse_binary_segment_arg(&mut self) -> PResult<'a, BinarySegmentArg> {
        let lo = self.token.span;
        let kind = self.parse_binary_segment_arg_kind()?;
        Ok(BinarySegmentArg { kind, span: lo.to(self.prev_token.span) })
    }

    fn parse_binary_segment_arg_kind(&mut self) -> PResult<'a, BinarySegmentArgKind> {
        if let Some((ident, _)) = self.token.ident() {
            let next_is = |this: &Self, kind: token::TokenKind| this.look_ahead(1, |t| *t == kind);
            if matches!(ident.name, sym::size | sym::unit) && next_is(self, token::Eq) {
                self.bump(); // `size` or `unit`
                self.bump(); // `=`
                let expr = self.parse_expr()?;
                return Ok(if ident.name == sym::size {
                    BinarySegmentArgKind::Size(expr)
                } else {
                    BinarySegmentArgKind::Unit(expr)
                });
            }
            if let Some(flag) = BinarySegmentFlag::from_symbol(ident.name)
                && (next_is(self, token::Comma) || next_is(self, token::CloseParen))
            {
                self.bump();
                return Ok(BinarySegmentArgKind::Flag(flag));
            }
        }
        Ok(BinarySegmentArgKind::Size(self.parse_expr()?))
    }
}
