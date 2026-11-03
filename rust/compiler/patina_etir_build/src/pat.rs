use rustc_abi::FIRST_VARIANT;
use rustc_data_structures::packed::Pu128;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::TermClass;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::etir::visit::{Visitor, walk_pat};
use rustc_middle::erl::etir::{self, ExprId, PatId, PatternInput, VarId};
use rustc_middle::span_bug;
use rustc_middle::thir::{self, LocalVarId, PatKind};
use rustc_middle::ty::{self, GenericArgsRef, Ty};
use rustc_span::{ByteSymbol, Span, Symbol};

use super::{ThirBodyCx, not_implemented_kind};

impl<'a, 'tcx> ThirBodyCx<'a, 'tcx> {
    pub(super) fn lower_pat(&mut self, pat: &thir::Pat<'tcx>, input: PatternInput) -> PatId {
        let tcx = self.cx.tcx;
        match &pat.kind {
            PatKind::Or { pats } => self.lower_pat_or(pat, pats, input),

            PatKind::Range(range) => self.lower_pat_range(pat, range, input),

            PatKind::Wild => self.lower_pat_wild(pat, input),

            PatKind::Binding { .. } | PatKind::Pin { .. } | PatKind::Assignee { .. }
                if pat.ty.is_phantom_data() =>
            {
                self.lower_pat_phantom(pat, input)
            }

            PatKind::Binding { var, subpattern, is_primary, .. } => {
                self.lower_pat_binding(pat, *var, subpattern.as_deref(), *is_primary, input)
            }

            PatKind::Pin { var } => self.lower_pat_pin(pat, *var),

            PatKind::Assignee { var } => self.lower_pat_assignee(pat, *var),

            PatKind::TermClass { class, subpattern } => {
                self.lower_pat_term_class(pat, *class, subpattern)
            }

            PatKind::Exception { .. } => {
                span_bug!(pat.span, "exception pattern outside a catch arm")
            }

            PatKind::TermPat { subpattern, adt_repr_match } => {
                self.lower_pat_term(subpattern, *adt_repr_match)
            }

            PatKind::Deref { subpattern, .. } => self.lower_pat(subpattern, input),

            PatKind::Leaf { subpatterns } => self.lower_pat_leaf(pat, subpatterns, input),

            PatKind::Variant { adt_def, variant_index, subpatterns, .. } => {
                self.lower_pat_adt(pat, *adt_def, *variant_index, subpatterns, input)
            }

            PatKind::Constant { value } => self.lower_pat_constant(pat, *value),

            PatKind::NamedConst { def_id, args } => self.lower_pat_named_const(pat, *def_id, args),

            PatKind::Map { fields } => self.lower_pat_map(pat, fields, input),

            PatKind::BinaryPattern { segments } => self.lower_pat_binary_pattern(pat, segments),

            PatKind::Slice { prefix, slice, suffix } => {
                self.lower_pat_slice(pat, prefix, slice.as_deref(), suffix, input)
            }

            _ => not_implemented_kind(tcx, pat.span, "THIR pattern kind", &pat.kind),
        }
    }

    fn lower_pat_or(
        &mut self,
        pat: &thir::Pat<'tcx>,
        pats: &[thir::Pat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        let pats = pats.iter().map(|pat| self.lower_pat(pat, input)).collect();
        self.cx.push_pat(etir::PatKind::Or { pats }, pat.ty, pat.span)
    }

    fn lower_pat_range(
        &mut self,
        pat: &thir::Pat<'tcx>,
        range: &thir::PatRange<'tcx>,
        input: PatternInput,
    ) -> PatId {
        let span = pat.span;
        let lo = range.lo.as_finite().map(|value| self.range_bound(span, range.ty, value));
        let hi = range.hi.as_finite().map(|value| self.range_bound(span, range.ty, value));
        let range = etir::PatRange { lo, hi, end: range.end, ty: range.ty, input };
        self.cx.push_pat(etir::PatKind::Range(Box::new(range)), pat.ty, span)
    }

    fn range_bound(&mut self, span: Span, ty: Ty<'tcx>, value: ty::ValTree<'tcx>) -> ExprId {
        let tcx = self.cx.tcx;
        let lit = match ty.kind() {
            ty::Int => {
                let value = value.to_leaf().to_i128();
                etir::Lit::Int { value: Pu128(value.unsigned_abs()), neg: value < 0 }
            }
            ty::Char => {
                let value = char::try_from(value.to_leaf())
                    .unwrap_or_else(|_| span_bug!(span, "invalid range character"));
                etir::Lit::Char(value)
            }
            ty::Float => etir::Lit::Float(value.to_leaf().to_u64()),
            _ => not_implemented_kind(tcx, span, "range pattern type", ty),
        };
        self.cx.push_expr(etir::ExprKind::Literal(lit, span), ty, span)
    }

    fn lower_pat_wild(&mut self, pat: &thir::Pat<'tcx>, input: PatternInput) -> PatId {
        self.cx.push_pat(etir::PatKind::Wild { input }, pat.ty, pat.span)
    }

    fn lower_pat_phantom(&mut self, pat: &thir::Pat<'tcx>, input: PatternInput) -> PatId {
        match &pat.kind {
            PatKind::Binding { subpattern: Some(subpattern), .. } => {
                self.lower_pat(subpattern, input)
            }
            _ => self.lower_pat_wild(pat, input),
        }
    }

    fn lower_pat_binding(
        &mut self,
        pat: &thir::Pat<'tcx>,
        var: LocalVarId,
        subpattern: Option<&thir::Pat<'tcx>>,
        is_primary: bool,
        input: PatternInput,
    ) -> PatId {
        let var = self.var(var);
        let subpattern = subpattern.map(|sub| self.lower_pat(sub, input));
        let kind = etir::PatKind::Binding { var, is_primary, subpattern, input };
        self.cx.push_pat(kind, pat.ty, pat.span)
    }

    fn lower_pat_pin(&mut self, pat: &thir::Pat<'tcx>, var: LocalVarId) -> PatId {
        let var = self.var(var);
        self.cx.push_pat(etir::PatKind::Pin { var }, pat.ty, pat.span)
    }

    fn lower_pat_assignee(&mut self, pat: &thir::Pat<'tcx>, var: LocalVarId) -> PatId {
        let var = self.var(var);
        self.cx.push_pat(etir::PatKind::Assignee { var }, pat.ty, pat.span)
    }

    fn lower_pat_term_class(
        &mut self,
        pat: &thir::Pat<'tcx>,
        class: TermClass,
        subpattern: &thir::Pat<'tcx>,
    ) -> PatId {
        let subpattern = self.lower_pat(subpattern, PatternInput::Typed);
        self.cx.push_pat(etir::PatKind::TermClass { class, subpattern }, pat.ty, pat.span)
    }

    fn lower_pat_term(&mut self, subpattern: &thir::Pat<'tcx>, adt_repr_match: bool) -> PatId {
        let input = if adt_repr_match {
            PatternInput::UntrustedRepresentation
        } else {
            PatternInput::Typed
        };
        self.lower_pat(subpattern, input)
    }

    fn lower_pat_leaf(
        &mut self,
        pat: &thir::Pat<'tcx>,
        subpatterns: &[thir::FieldPat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        let tcx = self.cx.tcx;
        match pat.ty.kind() {
            ty::Tuple(field_tys) => self.lower_pat_tuple(pat, field_tys, subpatterns, input),

            ty::Adt(adt_def, _) => {
                self.lower_pat_adt(pat, *adt_def, FIRST_VARIANT, subpatterns, input)
            }

            _ => not_implemented_kind(tcx, pat.span, "leaf pattern type", pat.ty),
        }
    }

    fn lower_pat_tuple(
        &mut self,
        pat: &thir::Pat<'tcx>,
        field_tys: &'tcx ty::List<Ty<'tcx>>,
        subpatterns: &[thir::FieldPat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        let fields = field_tys
            .iter()
            .enumerate()
            .map(|(idx, field_ty)| {
                match subpatterns.iter().find(|field| field.field.index() == idx) {
                    Some(field) => self.lower_pat(&field.pattern, input),
                    None => self.cx.push_pat(etir::PatKind::Wild { input }, field_ty, pat.span),
                }
            })
            .collect();

        self.cx.push_pat(etir::PatKind::Tuple { fields }, pat.ty, pat.span)
    }

    fn lower_pat_adt(
        &mut self,
        pat: &thir::Pat<'tcx>,
        adt_def: ty::AdtDef<'tcx>,
        variant_index: rustc_abi::VariantIdx,
        subpatterns: &[thir::FieldPat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        let tcx = self.cx.tcx;
        let span = pat.span;
        let variant = adt_def.variant(variant_index);
        let ty::Adt(_, args) = *pat.ty.kind() else {
            span_bug!(span, "ADT pattern has non-ADT type {}", pat.ty)
        };

        let repr = self.lower_adt_repr(span, adt_def, variant);

        // In declaration order.
        let fields = tcx
            .erl_runtime_fields(variant)
            .map(|(idx, field_def)| {
                let ty = field_def.ty(tcx, args);
                let pattern = match subpatterns.iter().find(|field| field.field == idx) {
                    Some(field) => self.lower_pat(&field.pattern, input),
                    None => self.cx.push_pat(etir::PatKind::Wild { input }, ty, span),
                };
                etir::FieldPat { name: field_def.name, ty, pattern }
            })
            .collect();

        let adt = etir::AdtPat { repr, fields, input };
        self.cx.push_pat(etir::PatKind::Adt(Box::new(adt)), pat.ty, span)
    }

    fn lower_pat_constant(&mut self, pat: &thir::Pat<'tcx>, value: ty::Value<'tcx>) -> PatId {
        let tcx = self.cx.tcx;
        let span = pat.span;
        let lit = match *value.ty.kind() {
            ty::Bool => {
                let value = value
                    .try_to_bool()
                    .unwrap_or_else(|| span_bug!(span, "invalid boolean pattern constant"));
                etir::Lit::Bool(value)
            }
            ty::Char => {
                let value = char::try_from(value.to_leaf())
                    .unwrap_or_else(|_| span_bug!(span, "invalid character pattern constant"));
                etir::Lit::Char(value)
            }
            ty::Int => {
                let value = value.to_leaf().to_i128();
                etir::Lit::Int { value: Pu128(value.unsigned_abs()), neg: value < 0 }
            }
            ty::Float => etir::Lit::Float(value.to_leaf().to_u64()),
            ty::Bin | ty::Bits => {
                let bytes = value.try_to_raw_bytes(tcx).unwrap_or_else(|| {
                    span_bug!(span, "binary pattern did not lower to raw bytes")
                });
                etir::Lit::ByteStr(ByteSymbol::intern(bytes))
            }
            kind @ (ty::Str | ty::Atom) => {
                let bytes = value.try_to_raw_bytes(tcx).unwrap_or_else(|| {
                    span_bug!(span, "{kind:?} pattern did not lower to raw bytes")
                });
                let str = str::from_utf8(bytes)
                    .unwrap_or_else(|_| span_bug!(span, "{kind:?} pattern is not valid UTF-8"));

                let sym = Symbol::intern(str);
                if kind == ty::Str { etir::Lit::Str(sym) } else { etir::Lit::Atom(sym) }
            }
            _ => not_implemented_kind(tcx, span, "constant pattern type", value.ty),
        };

        self.cx.push_pat(etir::PatKind::Literal(lit), pat.ty, span)
    }

    fn lower_pat_named_const(
        &mut self,
        pat: &thir::Pat<'tcx>,
        def_id: DefId,
        args: GenericArgsRef<'tcx>,
    ) -> PatId {
        // Reports a cycle between constants as a query cycle, for consistency with expr side.
        self.cx.tcx.ensure_ok().etir_template(def_id);
        self.cx.push_pat(etir::PatKind::Const { def_id, args }, pat.ty, pat.span)
    }

    fn lower_pat_map(
        &mut self,
        pat: &thir::Pat<'tcx>,
        fields: &[thir::MapFieldPat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        let fields = fields
            .iter()
            .map(|field| (self.lower_expr(field.key), self.lower_pat(&field.pattern, input)))
            .collect();

        self.cx.push_pat(etir::PatKind::Map { fields }, pat.ty, pat.span)
    }

    fn lower_pat_binary_pattern(
        &mut self,
        pat: &thir::Pat<'tcx>,
        segments: &[thir::BinarySegmentPat<'tcx>],
    ) -> PatId {
        let segments =
            segments.iter().map(|segment| self.lower_pat_binary_segment(segment)).collect();
        self.cx.push_pat(etir::PatKind::BinaryPattern { segments }, pat.ty, pat.span)
    }

    fn lower_pat_binary_segment(
        &mut self,
        segment: &thir::BinarySegmentPat<'tcx>,
    ) -> etir::BinarySegmentPat {
        let layout = segment.layout.with_size_expr(|size| self.lower_expr(size));
        let pattern = self.lower_pat(&segment.pattern, PatternInput::Typed);
        etir::BinarySegmentPat { pattern, layout, span: segment.span }
    }

    fn lower_pat_slice(
        &mut self,
        pat: &thir::Pat<'tcx>,
        prefix: &[thir::Pat<'tcx>],
        slice: Option<&thir::Pat<'tcx>>,
        suffix: &[thir::Pat<'tcx>],
        input: PatternInput,
    ) -> PatId {
        if !suffix.is_empty() {
            span_bug!(pat.span, "suffix not supported at this stage")
        }
        let items = prefix.iter().map(|pat| self.lower_pat(pat, input)).collect();
        let tail = slice.map(|tail| self.lower_pat(tail, input));
        self.cx.push_pat(etir::PatKind::List { items, tail }, pat.ty, pat.span)
    }

    /// Walks `pattern` and list all variables it binds.
    pub(super) fn bound_vars(&self, pattern: PatId) -> SmallVec<[VarId; 1]> {
        struct BoundVars<'a, 'tcx> {
            body: &'a etir::Body<'tcx>,
            vars: SmallVec<[VarId; 1]>,
        }

        impl<'a, 'tcx: 'a> Visitor<'a, 'tcx> for BoundVars<'a, 'tcx> {
            fn etir(&self) -> &'a etir::Body<'tcx> {
                self.body
            }

            fn visit_expr(&mut self, _id: ExprId, _expr: &'a etir::Expr<'tcx>) {
                // No (externally visible) binding happens within pattern's expressions.
            }

            fn visit_pat(&mut self, _id: PatId, pat: &'a etir::Pat<'tcx>) {
                if let etir::PatKind::Binding { var, is_primary: true, .. } = pat.kind {
                    self.vars.push(var);
                }
                walk_pat(self, pat);
            }
        }

        let mut visitor = BoundVars { body: &self.cx.body, vars: SmallVec::new() };
        visitor.visit_pat_id(&pattern);
        visitor.vars
    }

    pub(super) fn bool_pat(&mut self, span: Span, value: bool) -> PatId {
        let kind = etir::PatKind::Literal(etir::Lit::Bool(value));
        self.cx.push_pat(kind, self.cx.tcx.types.bool, span)
    }

    pub(super) fn is_literal_bool_pat(&self, pattern: PatId, value: bool) -> bool {
        matches!(self.cx.body[pattern].kind, etir::PatKind::Literal(etir::Lit::Bool(v)) if v == value)
    }
}
