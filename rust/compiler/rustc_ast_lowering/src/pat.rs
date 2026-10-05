use std::sync::Arc;

use rustc_ast::*;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{self as hir, LangItem, Target};
use rustc_middle::span_bug;
use rustc_span::{DesugaringKind, ErrorGuaranteed, Ident, Span, Spanned, respan};

use super::errors::{
    ArbitraryExpressionInPattern, ExtraDoubleDot, InvalidMapPatternKey, InvalidMapPatternKeyHelp,
    MisplacedDoubleDot, SubTupleBinding, UnpinnedLocalInMapPatternKey,
};
use super::{
    AllowReturnTypeNotation, ImplTraitContext, ImplTraitPosition, LoweringContext, ParamMode,
};

impl<'hir> LoweringContext<'_, 'hir> {
    pub(crate) fn lower_pat(&mut self, pattern: &Pat) -> &'hir hir::Pat<'hir> {
        self.arena.alloc(self.lower_pat_mut(pattern))
    }

    fn lower_pat_mut(&mut self, mut pattern: &Pat) -> hir::Pat<'hir> {
        ensure_sufficient_stack(|| {
            // loop here to avoid recursion
            let pat_hir_id = self.lower_node_id(pattern.id);
            let node = loop {
                match &pattern.kind {
                    PatKind::Missing => break hir::PatKind::Missing,
                    PatKind::Wild => break hir::PatKind::Wild,
                    PatKind::Never => break hir::PatKind::Never,
                    PatKind::Ident(binding_mode, ident, sub) => {
                        let lower_sub = |this: &mut Self| sub.as_ref().map(|s| this.lower_pat(s));
                        break self.lower_pat_ident(
                            pattern,
                            *binding_mode,
                            *ident,
                            pat_hir_id,
                            lower_sub,
                        );
                    }
                    PatKind::Expr(e) => {
                        break hir::PatKind::Expr(self.lower_expr_within_pat(e, false));
                    }
                    PatKind::Pin(e) => {
                        break hir::PatKind::Pin(self.lower_expr_within_pat(e, true));
                    }
                    PatKind::TermClass(subpattern, class) => {
                        break hir::PatKind::TermClass(self.lower_pat(subpattern), *class);
                    }
                    PatKind::ExceptionClass(args, class) => {
                        break self.lower_pat_exception(pattern.span, args, *class);
                    }
                    PatKind::TupleStruct(qself, path, pats) => {
                        let qpath = self.lower_qpath(
                            pattern.id,
                            qself,
                            path,
                            ParamMode::Optional,
                            AllowReturnTypeNotation::No,
                            ImplTraitContext::Disallowed(ImplTraitPosition::Path),
                            None,
                        );
                        let (pats, ddpos) = self.lower_pat_tuple(pats, "tuple struct");
                        break hir::PatKind::TupleStruct(qpath, pats, ddpos);
                    }
                    PatKind::Or(pats) => {
                        break hir::PatKind::Or(
                            self.arena.alloc_from_iter(pats.iter().map(|x| self.lower_pat_mut(x))),
                        );
                    }
                    PatKind::Path(qself, path) => {
                        let qpath = self.lower_qpath(
                            pattern.id,
                            qself,
                            path,
                            ParamMode::Optional,
                            AllowReturnTypeNotation::No,
                            ImplTraitContext::Disallowed(ImplTraitPosition::Path),
                            None,
                        );
                        let kind = hir::PatExprKind::Path(qpath);
                        let span = self.lower_span(pattern.span);
                        let expr = hir::PatExpr { hir_id: pat_hir_id, span, kind };
                        let expr = self.arena.alloc(expr);
                        return hir::Pat {
                            hir_id: self.next_id(),
                            kind: hir::PatKind::Expr(expr),
                            span,
                            default_binding_modes: true,
                        };
                    }
                    PatKind::Struct(qself, path, fields, etc) => {
                        let qpath = self.lower_qpath(
                            pattern.id,
                            qself,
                            path,
                            ParamMode::Optional,
                            AllowReturnTypeNotation::No,
                            ImplTraitContext::Disallowed(ImplTraitPosition::Path),
                            None,
                        );

                        let fs = self.arena.alloc_from_iter(fields.iter().map(|f| {
                            let hir_id = self.lower_node_id(f.id);
                            self.lower_attrs(hir_id, &f.attrs, f.span, Target::PatField);

                            hir::PatField {
                                hir_id,
                                ident: self.lower_ident(f.ident),
                                pat: self.lower_pat(&f.pat),
                                is_shorthand: f.is_shorthand,
                                span: self.lower_span(f.span),
                            }
                        }));
                        break hir::PatKind::Struct(
                            qpath,
                            fs,
                            match etc {
                                ast::PatFieldsRest::Rest(sp) => Some(self.lower_span(*sp)),
                                ast::PatFieldsRest::Recovered(_) => Some(Span::default()),
                                _ => None,
                            },
                        );
                    }
                    PatKind::Tuple(pats) => {
                        let (pats, ddpos) = self.lower_pat_tuple(pats, "tuple");
                        break hir::PatKind::Tuple(pats, ddpos);
                    }
                    PatKind::Box(inner) => {
                        break hir::PatKind::Box(self.lower_pat(inner));
                    }
                    PatKind::Deref(inner) => {
                        break hir::PatKind::Deref(self.lower_pat(inner));
                    }
                    PatKind::Ref(inner, pinned, mutbl) => {
                        break hir::PatKind::Ref(self.lower_pat(inner), *pinned, *mutbl);
                    }
                    PatKind::Range(e1, e2, Spanned { node: end, .. }) => {
                        break hir::PatKind::Range(
                            e1.as_deref().map(|e| self.lower_expr_within_pat(e, true)),
                            e2.as_deref().map(|e| self.lower_expr_within_pat(e, true)),
                            self.lower_range_end(end, e2.is_some()),
                        );
                    }
                    PatKind::Guard(inner, guard) => {
                        break hir::PatKind::Guard(
                            self.lower_pat(inner),
                            self.lower_expr(&guard.cond),
                        );
                    }
                    PatKind::Slice(pats) => break self.lower_pat_slice(pats),
                    PatKind::Map(map_fields, rest) => {
                        let fields = self.arena.alloc_from_iter(map_fields.iter().map(|field| {
                            hir::PatMapField {
                                key: self.lower_map_key(&field.key, MapKeyContext::Pattern),
                                pat: self.lower_pat(&field.pat),
                                span: self.lower_span(field.span),
                            }
                        }));
                        let rest = match rest {
                            ast::PatMapFieldsRest::Rest(span) => self.lower_span(*span),
                            ast::PatMapFieldsRest::Recovered(_) => Span::default(),
                            ast::PatMapFieldsRest::None => {
                                span_bug!(pattern.span, "map pattern without required `..`")
                            }
                        };
                        break hir::PatKind::Map(fields, rest);
                    }
                    PatKind::BinaryPattern(segments) => {
                        let segments = self.arena.alloc_from_iter(segments.iter().map(|segment| {
                            hir::PatBinarySegment {
                                pat: self.lower_pat(&segment.pat),
                                kind: match &segment.kind {
                                    PatBinarySegmentKind::Default => {
                                        hir::PatBinarySegmentKind::Default
                                    }
                                    PatBinarySegmentKind::Spec(spec) => {
                                        hir::PatBinarySegmentKind::Spec(
                                            self.lower_binary_segment_spec(spec),
                                        )
                                    }
                                    PatBinarySegmentKind::Rest => hir::PatBinarySegmentKind::Rest,
                                    PatBinarySegmentKind::Recovered(guar) => {
                                        hir::PatBinarySegmentKind::Recovered(*guar)
                                    }
                                },
                                span: self.lower_span(segment.span),
                            }
                        }));
                        break hir::PatKind::BinaryPattern(segments);
                    }
                    PatKind::Rest => {
                        // If we reach here the `..` pattern is not semantically allowed.
                        break self.ban_illegal_rest_pat(pattern.span);
                    }
                    // return inner to be processed in next loop
                    PatKind::Paren(inner) => pattern = inner,
                    PatKind::MacCall(_) => {
                        panic!("{pattern:#?} shouldn't exist here")
                    }
                    PatKind::Err(guar) => break hir::PatKind::Err(*guar),
                }
            };

            self.pat_with_node_id_of(pattern, node, pat_hir_id)
        })
    }

    fn lower_pat_exception(
        &mut self,
        span: Span,
        args: &[Pat],
        class: ExceptionClass,
    ) -> hir::PatKind<'hir> {
        let span = self.lower_span(span);
        let (class_pat, reason_pat) = match (class, args) {
            (ExceptionClass::Any, [class_pat, reason_pat]) => {
                (self.lower_pat(class_pat), self.lower_pat(reason_pat))
            }
            (
                ExceptionClass::Error | ExceptionClass::Throw | ExceptionClass::Exit,
                [reason_pat],
            ) => (self.pat_exception_class_atom(span, class), self.lower_pat(reason_pat)),
            // The parser rejects a mismatched arity before we get here.
            _ => span_bug!(span, "`{}` pattern with {} subpatterns", class.name(), args.len()),
        };
        hir::PatKind::Exception { class, class_pat, reason_pat }
    }

    /// The atom literal pattern matching the class of `class`, e.g. `a"error"` for `error(..)`.
    fn pat_exception_class_atom(
        &mut self,
        span: Span,
        class: ExceptionClass,
    ) -> &'hir hir::Pat<'hir> {
        let lit = respan(span, LitKind::Atom(class.name(), StrStyle::Cooked));
        let expr = self.arena.alloc(hir::PatExpr {
            hir_id: self.next_id(),
            span,
            kind: hir::PatExprKind::Lit { lit, negated: false },
        });
        self.arena.alloc(hir::Pat {
            hir_id: self.next_id(),
            kind: hir::PatKind::Expr(expr),
            span,
            default_binding_modes: true,
        })
    }

    fn lower_pat_tuple(
        &mut self,
        pats: &[Pat],
        ctx: &str,
    ) -> (&'hir [hir::Pat<'hir>], hir::DotDotPos) {
        let mut elems = Vec::with_capacity(pats.len());
        let mut rest = None;

        let mut iter = pats.iter().enumerate();
        for (idx, pat) in iter.by_ref() {
            // Interpret the first `..` pattern as a sub-tuple pattern.
            // Note that unlike for slice patterns,
            // where `xs @ ..` is a legal sub-slice pattern,
            // it is not a legal sub-tuple pattern.
            match &pat.kind {
                // Found a sub-tuple rest pattern
                PatKind::Rest => {
                    rest = Some((idx, pat.span));
                    break;
                }
                // Found a sub-tuple pattern `$binding_mode $ident @ ..`.
                // This is not allowed as a sub-tuple pattern
                PatKind::Ident(_, ident, Some(sub)) if sub.is_rest() => {
                    let sp = pat.span;
                    self.dcx().emit_err(SubTupleBinding {
                        span: sp,
                        ident_name: ident.name,
                        ident: *ident,
                        ctx,
                    });
                }
                _ => {}
            }

            // It was not a sub-tuple pattern so lower it normally.
            elems.push(self.lower_pat_mut(pat));
        }

        for (_, pat) in iter {
            // There was a previous sub-tuple pattern; make sure we don't allow more...
            if pat.is_rest() {
                // ...but there was one again, so error.
                self.ban_extra_rest_pat(pat.span, rest.unwrap().1, ctx);
            } else {
                elems.push(self.lower_pat_mut(pat));
            }
        }

        (self.arena.alloc_from_iter(elems), hir::DotDotPos::new(rest.map(|(ddpos, _)| ddpos)))
    }

    /// Lower a slice pattern of form `[pat_0, ..., pat_n]` into
    /// `hir::PatKind::Slice(before, slice, after)`.
    ///
    /// When encountering `($binding_mode $ident @)? ..` (`slice`),
    /// this is interpreted as a sub-slice pattern semantically.
    /// Patterns that follow, which are not like `slice` -- or an error occurs, are in `after`.
    fn lower_pat_slice(&mut self, pats: &[Pat]) -> hir::PatKind<'hir> {
        let mut before = Vec::new();
        let mut after = Vec::new();
        let mut slice = None;

        // Lowers `$bm $ident @ ..` to `$bm $ident @ _`.
        let lower_rest_sub = |this: &mut Self, pat: &Pat, &ann, &ident, sub: &Pat| {
            let sub_hir_id = this.lower_node_id(sub.id);
            let lower_sub = |this: &mut Self| Some(this.pat_wild_with_node_id_of(sub, sub_hir_id));
            let pat_hir_id = this.lower_node_id(pat.id);
            let node = this.lower_pat_ident(pat, ann, ident, pat_hir_id, lower_sub);
            this.pat_with_node_id_of(pat, node, pat_hir_id)
        };

        let mut iter = pats.iter();
        // Lower all the patterns until the first occurrence of a sub-slice pattern.
        for pat in iter.by_ref() {
            match &pat.kind {
                // Found a sub-slice pattern `..`. Record, lower it to `_`, and stop here.
                PatKind::Rest => {
                    let hir_id = self.lower_node_id(pat.id);
                    slice = Some(self.pat_wild_with_node_id_of(pat, hir_id));
                    break;
                }
                // Found a sub-slice pattern `$binding_mode $ident @ ..`.
                // Record, lower it to `$binding_mode $ident @ _`, and stop here.
                PatKind::Ident(ann, ident, Some(sub)) if sub.is_rest() => {
                    slice = Some(self.arena.alloc(lower_rest_sub(self, pat, ann, ident, sub)));
                    break;
                }
                // It was not a subslice pattern so lower it normally.
                _ => before.push(self.lower_pat_mut(pat)),
            }
        }

        // Lower all the patterns after the first sub-slice pattern.
        for pat in iter {
            // Parser has already rejected any rest before the final list element.
            // Continue lowering only to recover after that diagnostic.
            let rest_span = match &pat.kind {
                PatKind::Rest => Some(pat.span),
                PatKind::Ident(ann, ident, Some(sub)) if sub.is_rest() => {
                    // #69103: Lower into `binding @ _` as above to avoid ICEs.
                    after.push(lower_rest_sub(self, pat, ann, ident, sub));
                    Some(sub.span)
                }
                _ => None,
            };
            if rest_span.is_none() {
                // Lower the pattern normally.
                after.push(self.lower_pat_mut(pat));
            }
        }

        hir::PatKind::Slice(
            self.arena.alloc_from_iter(before),
            slice,
            self.arena.alloc_from_iter(after),
        )
    }

    fn lower_pat_ident(
        &mut self,
        p: &Pat,
        annotation: BindingMode,
        ident: Ident,
        hir_id: hir::HirId,
        lower_sub: impl FnOnce(&mut Self) -> Option<&'hir hir::Pat<'hir>>,
    ) -> hir::PatKind<'hir> {
        match self.get_partial_res(p.id).map(|d| d.expect_full_res()) {
            // `None` can occur in body-less function signatures
            res @ (None | Some(Res::Local(_))) => {
                let binding_id = match res {
                    Some(Res::Local(id)) => {
                        // In `Or` patterns like `VariantA(s) | VariantB(s, _)`, multiple identifier patterns
                        // will be resolved to the same `Res::Local`. Thus they just share a single
                        // `HirId`.
                        if id == p.id {
                            self.ident_and_label_to_local_id.insert(id, hir_id.local_id);
                            hir_id
                        } else {
                            hir::HirId {
                                owner: self.current_hir_id_owner,
                                local_id: self.ident_and_label_to_local_id[&id],
                            }
                        }
                    }
                    _ => {
                        self.ident_and_label_to_local_id.insert(p.id, hir_id.local_id);
                        hir_id
                    }
                };
                hir::PatKind::Binding(
                    annotation,
                    binding_id,
                    self.lower_ident(ident),
                    lower_sub(self),
                )
            }
            Some(res) => {
                let res = self.lower_res(res);
                let span = self.lower_span(ident.span);
                hir::PatKind::Expr(self.arena.alloc(hir::PatExpr {
                    kind: hir::PatExprKind::Path(hir::QPath::Resolved(
                        None,
                        self.arena.alloc(hir::Path {
                            span,
                            res,
                            segments: arena_vec![self; hir::PathSegment::new(self.lower_ident(ident), self.next_id(), res)],
                        }),
                    )),
                    hir_id: self.next_id(),
                    span,
                }))
            }
        }
    }

    fn pat_wild_with_node_id_of(&mut self, p: &Pat, hir_id: hir::HirId) -> &'hir hir::Pat<'hir> {
        self.arena.alloc(self.pat_with_node_id_of(p, hir::PatKind::Wild, hir_id))
    }

    /// Construct a `Pat` with the `HirId` of `p.id` already lowered.
    fn pat_with_node_id_of(
        &mut self,
        p: &Pat,
        kind: hir::PatKind<'hir>,
        hir_id: hir::HirId,
    ) -> hir::Pat<'hir> {
        hir::Pat { hir_id, kind, span: self.lower_span(p.span), default_binding_modes: true }
    }

    /// Emit a friendly error for extra `..` patterns in a tuple/tuple struct/slice pattern.
    pub(crate) fn ban_extra_rest_pat(&self, sp: Span, prev_sp: Span, ctx: &str) {
        self.dcx().emit_err(ExtraDoubleDot { span: sp, prev_span: prev_sp, ctx });
    }

    /// Used to ban the `..` pattern in places it shouldn't be semantically.
    fn ban_illegal_rest_pat(&self, sp: Span) -> hir::PatKind<'hir> {
        self.dcx().emit_err(MisplacedDoubleDot { span: sp });

        // We're not in a list context so `..` can be reasonably treated
        // as `_` because it should always be valid and roughly matches the
        // intent of `..` (notice that the rest of a single slot is that slot).
        hir::PatKind::Wild
    }

    fn lower_range_end(&mut self, e: &RangeEnd, has_end: bool) -> hir::RangeEnd {
        match *e {
            RangeEnd::Excluded if has_end => hir::RangeEnd::Excluded,
            // No end; so `X..` behaves like `RangeFrom`.
            RangeEnd::Excluded | RangeEnd::Included(_) => hir::RangeEnd::Included,
        }
    }

    /// Matches `'-' lit | lit (cf. parser::Parser::parse_literal_maybe_minus)`,
    /// or paths for ranges.
    //
    // FIXME: do we want to allow `expr -> pattern` conversion to create path expressions?
    // That means making this work:
    //
    // ```rust,ignore (FIXME)
    // struct S;
    // macro_rules! m {
    //     ($a:expr) => {
    //         let $a = S;
    //     }
    // }
    // m!(S);
    // ```
    pub(super) fn lower_expr_within_pat(
        &mut self,
        expr: &Expr,
        allow_paths: bool,
    ) -> &'hir hir::PatExpr<'hir> {
        let expr = self.do_lower_expr_within_pat(expr, allow_paths);
        self.arena.alloc(expr)
    }

    fn do_lower_expr_within_pat(&mut self, expr: &Expr, allow_paths: bool) -> hir::PatExpr<'hir> {
        let span = self.lower_span(expr.span);
        let err =
            |guar| hir::PatExprKind::Lit { lit: respan(span, LitKind::Err(guar)), negated: false };
        let kind = match &expr.kind {
            ExprKind::Lit(lit) => {
                hir::PatExprKind::Lit { lit: self.lower_lit(lit, span), negated: false }
            }
            ExprKind::IncludedBytes(byte_sym) => hir::PatExprKind::Lit {
                lit: respan(span, LitKind::ByteStr(*byte_sym, StrStyle::Cooked)),
                negated: false,
            },
            ExprKind::Err(guar) => err(*guar),
            ExprKind::Dummy => span_bug!(span, "lowered ExprKind::Dummy"),
            ExprKind::Path(qself, path) if allow_paths => hir::PatExprKind::Path(self.lower_qpath(
                expr.id,
                qself,
                path,
                ParamMode::Optional,
                AllowReturnTypeNotation::No,
                ImplTraitContext::Disallowed(ImplTraitPosition::Path),
                None,
            )),
            ExprKind::Unary(UnOp::Neg, inner) if let ExprKind::Lit(lit) = &inner.kind => {
                hir::PatExprKind::Lit { lit: self.lower_lit(lit, span), negated: true }
            }
            _ => {
                let is_const_block = matches!(expr.kind, ExprKind::ConstBlock(_));
                let pattern_from_macro = expr.is_approximately_pattern()
                    || matches!(
                        expr.peel_parens().kind,
                        ExprKind::Binary(Spanned { node: BinOpKind::BitOr, .. }, ..)
                    );
                let guar = self.dcx().emit_err(ArbitraryExpressionInPattern {
                    span,
                    pattern_from_macro_note: pattern_from_macro,
                    const_block_in_pattern_help: is_const_block,
                });
                err(guar)
            }
        };
        hir::PatExpr { hir_id: self.lower_node_id(expr.id), span, kind }
    }

    pub(super) fn lower_map_key(
        &mut self,
        key: &Expr,
        ctx: MapKeyContext,
    ) -> &'hir hir::Expr<'hir> {
        if let Err(guar) = self.check_map_key(key, ctx) {
            return self.arena.alloc(self.expr_err(self.lower_span(key.span), guar));
        }
        let was_in_map_key = std::mem::replace(&mut self.is_in_map_key, true);
        let key = self.lower_expr(key);
        self.is_in_map_key = was_in_map_key;
        key
    }

    fn check_map_key(&self, expr: &Expr, ctx: MapKeyContext) -> Result<(), ErrorGuaranteed> {
        let (span, help) = match &expr.kind {
            // `|` reads as an or-pattern here, even though it parses as an operator.
            _ if let ExprKind::Binary(Spanned { node: BinOpKind::BitOr, .. }, ..) =
                expr.peel_parens().kind =>
            {
                let help = match ctx {
                    MapKeyContext::Pattern => Some(InvalidMapPatternKeyHelp::OrPattern),
                    MapKeyContext::Assignment => None,
                };
                (expr.span, help)
            }
            ExprKind::Lit(_)
            | ExprKind::IncludedBytes(_)
            | ExprKind::Pin(_)
            | ExprKind::Err(_)
            | ExprKind::Unary(UnOp::Neg, box Expr { kind: ExprKind::Lit(_), .. }) => {
                return Ok(());
            }
            ExprKind::Paren(inner) => return self.check_map_key(inner, ctx),
            ExprKind::Path(_, path) => {
                if let Some(Res::Local(_)) =
                    self.get_partial_res(expr.id).and_then(|res| res.full_res())
                {
                    let ident = path.segments[0].ident;
                    let error = UnpinnedLocalInMapPatternKey { span: expr.span, ident };
                    return Err(self.dcx().emit_err(error));
                }
                return Ok(());
            }
            ExprKind::Tup(exprs) => {
                return self.check_map_keys(exprs.iter().map(|expr| &**expr), ctx);
            }
            ExprKind::Array(box ArrayExpr { tail: ArrayTail::Tail(tail), .. }) => {
                (self.span_with_dot_dot(tail.span), None)
            }
            ExprKind::Array(array) => {
                return self.check_map_keys(array.elements.iter().map(|expr| &**expr), ctx);
            }
            ExprKind::Map(box MapExpr { rest: MapRest::Base(base), .. }) => {
                (self.span_with_dot_dot(base.span), None)
            }
            ExprKind::Map(map) => {
                return self.check_map_keys(
                    map.fields.iter().flat_map(|field| [&*field.key, &*field.value]),
                    ctx,
                );
            }
            ExprKind::BinaryPattern(segments) => {
                return self.check_map_keys(segments.iter().map(|segment| &*segment.expr), ctx);
            }
            ExprKind::Call(callee, args) if self.is_ctor_path(callee) => {
                return self.check_map_keys(args.iter().map(|arg| &**arg), ctx);
            }
            ExprKind::Struct(box StructExpr { rest: StructRest::Base(base), .. }) => {
                (self.span_with_dot_dot(base.span), None)
            }
            ExprKind::Struct(struct_expr) => {
                return self
                    .check_map_keys(struct_expr.fields.iter().map(|field| &*field.expr), ctx);
            }
            ExprKind::Call(..)
            | ExprKind::MethodCall(..)
            | ExprKind::Binary(..)
            | ExprKind::Unary(..) => (expr.span, Some(InvalidMapPatternKeyHelp::Compute)),
            ExprKind::Underscore => (expr.span, Some(InvalidMapPatternKeyHelp::Wildcard)),
            _ => (expr.span, None),
        };
        Err(self.dcx().emit_err(InvalidMapPatternKey { span, help }))
    }

    fn check_map_keys<'a>(
        &self,
        exprs: impl IntoIterator<Item = &'a Expr>,
        ctx: MapKeyContext,
    ) -> Result<(), ErrorGuaranteed> {
        let mut result = Ok(());
        for expr in exprs {
            result = result.and(self.check_map_key(expr, ctx));
        }
        result
    }

    /// Extends the span of a list tail or an update base to cover the `..` before it.
    fn span_with_dot_dot(&self, span: Span) -> Span {
        let source_map = self.tcx.sess.source_map();
        source_map
            .span_extend_prev_while(span, char::is_whitespace)
            .and_then(|span| source_map.span_extend_prev_while(span, |c| c == '.'))
            .unwrap_or(span)
    }

    fn is_ctor_path(&self, expr: &Expr) -> bool {
        matches!(expr.kind, ExprKind::Path(..))
            && matches!(
                self.get_partial_res(expr.id).and_then(|res| res.full_res()),
                Some(Res::Def(DefKind::Ctor(..), _) | Res::Err)
            )
    }

    pub(crate) fn lower_ty_pat(
        &mut self,
        pattern: &TyPat,
        base_type: Span,
    ) -> &'hir hir::TyPat<'hir> {
        self.arena.alloc(self.lower_ty_pat_mut(pattern, base_type))
    }

    fn lower_ty_pat_mut(&mut self, pattern: &TyPat, base_type: Span) -> hir::TyPat<'hir> {
        // loop here to avoid recursion
        let pat_hir_id = self.lower_node_id(pattern.id);
        let node = match &pattern.kind {
            TyPatKind::Range(e1, e2, Spanned { node: end, span }) => hir::TyPatKind::Range(
                e1.as_deref()
                    .map(|e| self.lower_anon_const_to_const_arg_and_alloc(e))
                    .unwrap_or_else(|| {
                        self.lower_ty_pat_range_end(
                            hir::LangItem::RangeMin,
                            span.shrink_to_lo(),
                            base_type,
                        )
                    }),
                e2.as_deref()
                    .map(|e| match end {
                        RangeEnd::Included(..) => self.lower_anon_const_to_const_arg_and_alloc(e),
                        RangeEnd::Excluded => self.lower_excluded_range_end(e),
                    })
                    .unwrap_or_else(|| {
                        self.lower_ty_pat_range_end(
                            hir::LangItem::RangeMax,
                            span.shrink_to_hi(),
                            base_type,
                        )
                    }),
            ),
            TyPatKind::NotNull => hir::TyPatKind::NotNull,
            TyPatKind::Or(variants) => {
                hir::TyPatKind::Or(self.arena.alloc_from_iter(
                    variants.iter().map(|pat| self.lower_ty_pat_mut(pat, base_type)),
                ))
            }
            TyPatKind::Err(guar) => hir::TyPatKind::Err(*guar),
        };

        hir::TyPat { hir_id: pat_hir_id, kind: node, span: self.lower_span(pattern.span) }
    }

    /// Lowers the range end of an exclusive range (`2..5`) to an inclusive range 2..=(5 - 1).
    /// This way the type system doesn't have to handle the distinction between inclusive/exclusive ranges.
    fn lower_excluded_range_end(&mut self, e: &AnonConst) -> &'hir hir::ConstArg<'hir> {
        let span = self.lower_span(e.value.span);
        let unstable_span = self.mark_span_with_reason(
            DesugaringKind::PatTyRange,
            span,
            Some(Arc::clone(&self.allow_pattern_type)),
        );
        let anon_const = self.with_new_scopes(span, |this| {
            let def_id = this.local_def_id(e.id);
            let hir_id = this.lower_node_id(e.id);
            let body = this.lower_body(|this| {
                // Need to use a custom function as we can't just subtract `1` from a `char`.
                let kind = hir::ExprKind::Path(this.make_lang_item_qpath(
                    hir::LangItem::RangeSub,
                    unstable_span,
                    None,
                ));
                let fn_def = this.arena.alloc(hir::Expr { hir_id: this.next_id(), kind, span });
                let args = this.arena.alloc([this.lower_expr_mut(&e.value)]);
                (
                    &[],
                    hir::Expr {
                        hir_id: this.next_id(),
                        kind: hir::ExprKind::Call(fn_def, args),
                        span,
                    },
                )
            });
            hir::AnonConst { def_id, hir_id, body, span }
        });
        self.arena.alloc(hir::ConstArg {
            hir_id: self.next_id(),
            kind: hir::ConstArgKind::Anon(self.arena.alloc(anon_const)),
            span,
        })
    }

    /// When a range has no end specified (`1..` or `1..=`) or no start specified (`..5` or `..=5`),
    /// we instead use a constant of the MAX/MIN of the type.
    /// This way the type system does not have to handle the lack of a start/end.
    fn lower_ty_pat_range_end(
        &mut self,
        lang_item: LangItem,
        span: Span,
        base_type: Span,
    ) -> &'hir hir::ConstArg<'hir> {
        let node_id = self.next_node_id();

        // Add a definition for the in-band const def.
        // We're generating a range end that didn't exist in the AST,
        // so the def collector didn't create the def ahead of time. That's why we have to do
        // it here.
        let def_id = self.create_def(node_id, None, DefKind::AnonConst, span);
        let hir_id = self.lower_node_id(node_id);

        let unstable_span = self.mark_span_with_reason(
            DesugaringKind::PatTyRange,
            self.lower_span(span),
            Some(Arc::clone(&self.allow_pattern_type)),
        );
        let span = self.lower_span(base_type);

        let path_expr = hir::Expr {
            hir_id: self.next_id(),
            kind: hir::ExprKind::Path(self.make_lang_item_qpath(lang_item, unstable_span, None)),
            span,
        };

        let ct = self.with_new_scopes(span, |this| {
            self.arena.alloc(hir::AnonConst {
                def_id,
                hir_id,
                body: this.lower_body(|_this| (&[], path_expr)),
                span,
            })
        });
        let hir_id = self.next_id();
        self.arena.alloc(hir::ConstArg { kind: hir::ConstArgKind::Anon(ct), hir_id, span })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum MapKeyContext {
    Pattern,
    Assignment,
}
