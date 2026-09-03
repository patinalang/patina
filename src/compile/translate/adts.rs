use rustc_abi::FieldIdx;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution, ErlRepr, ErlVariantRepr};
use rustc_middle::span_bug;
use rustc_middle::thir::{AdtExpr, AdtExprBase, Expr, FieldPat};
use rustc_middle::ty::{AdtDef, VariantDef};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::body::{BodyCx, Patterns};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_adt_pattern(
        &mut self,
        span: Span,
        adt_def: AdtDef<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
    ) -> Patterns<'arena> {
        let repr = self.tcx().erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlRepr::Atom => self.translator.afb.pat_atom(span, repr.tag).into(),
            ErlRepr::Tagged => self.translate_tagged_pattern(span, variant, subpatterns, repr),
            ErlRepr::Elixir => {
                self.translate_elixir_struct_pattern(span, adt_def, variant, subpatterns)
            }
            ErlRepr::NativeRecord => {
                self.translate_native_record_pattern(span, variant, subpatterns, repr)
            }
            ErlRepr::Transparent => {
                not_implemented!(self.tcx(), span, "ADT pattern representation", repr.kind)
            }
        }
    }

    pub(super) fn translate_adt(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let adt_def = adt_expr.adt_def;
        let variant = adt_def.variant(adt_expr.variant_index);

        let repr = self.tcx().erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlRepr::Atom => afb.expr_atom(expr.span, repr.tag),
            ErlRepr::Tagged => self.translate_tagged(expr, adt_expr, variant, repr),
            ErlRepr::Elixir => self.translate_elixir_struct(expr, adt_expr, variant),
            ErlRepr::NativeRecord => self.translate_native_record(expr, adt_expr, variant, repr),
            ErlRepr::Transparent => {
                not_implemented!(self.tcx(), expr.span, "ADT representation", repr.kind)
            }
        }
    }

    fn translate_tagged(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        match &adt_expr.base {
            AdtExprBase::None => {
                let tag = afb.expr_atom(expr.span, repr.tag);
                let fields = (0..variant.fields.len()).map(|index| {
                    let field = adt_expr
                        .fields
                        .iter()
                        .find(|field| field.name.index() == index)
                        .unwrap_or_else(|| {
                            span_bug!(
                                expr.span,
                                "tagged tuple construction is missing field {index}"
                            )
                        });
                    self.translate_expr(field.expr)
                });

                afb.expr_tuple(expr.span, std::iter::once(tag).chain(fields))
            }
            AdtExprBase::Base(base) => {
                let mut value = self.translate_expr(base.base);

                for field in &adt_expr.fields {
                    let field_span = self.thir[field.expr].span;
                    let field_value = self.translate_expr(field.expr);
                    value = afb.expr_remote_call(
                        field_span,
                        afb.expr_atom(field_span, sym::erlang),
                        afb.expr_atom(field_span, sym::setelement),
                        [
                            afb.expr_int(field_span, field.name.index() as u128 + 2),
                            value,
                            field_value,
                        ],
                    );
                }

                value
            }
            AdtExprBase::DefaultFields(_) => {
                not_implemented!(self.tcx(), expr.span, "tagged tuple with default fields")
            }
        }
    }

    fn translate_elixir_struct(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
        variant: &VariantDef,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let tcx = self.tcx();

        let base = match &adt_expr.base {
            AdtExprBase::None => {
                let fn_atom = afb.expr_atom(expr.span, sym::__struct__);

                match self.resolve_extern_module_name(
                    expr.span,
                    adt_expr.adt_def.did(),
                    variant.def_id,
                ) {
                    Some(module) => {
                        let mod_expr = afb.expr_atom(expr.span, module);
                        afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
                    }
                    None => afb.expr_local_call(expr.span, fn_atom, []),
                }
            }
            AdtExprBase::Base(base) => self.translate_expr(base.base),
            AdtExprBase::DefaultFields(_) => {
                not_implemented!(tcx, expr.span, "ADT default-field base", adt_expr.base);
            }
        };

        let updates = adt_expr.fields.iter().map(|field| {
            let field_def = &variant.fields[field.name];
            let value = self.translate_expr(field.expr);
            let key = afb.expr_atom(self.thir[field.expr].span, field_def.name);
            (key, value)
        });

        afb.expr_map_update_exact(expr.span, base, updates)
    }

    fn translate_native_record(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx();
        let afb = &self.translator.afb;

        let base = match &adt_expr.base {
            AdtExprBase::None => None,
            AdtExprBase::Base(base) => Some(self.translate_expr(base.base)),
            AdtExprBase::DefaultFields(_) => {
                not_implemented!(
                    tcx,
                    expr.span,
                    "native record with default fields",
                    adt_expr.base
                );
            }
        };

        let record_name = variant.name;
        let module = self.record_module_for_access(repr.record_module);

        let fields = adt_expr.fields.iter().map(|field| {
            let field_def = &variant.fields[field.name];
            let value = self.translate_expr(field.expr);
            let ident = Ident::new(field_def.name, self.thir[field.expr].span);
            (ident, value)
        });

        match base {
            None => afb.expr_record_create(expr.span, module, record_name, fields),
            Some(base) => afb.expr_record_update(expr.span, base, module, record_name, fields),
        }
    }

    fn translate_elixir_struct_pattern(
        &mut self,
        span: Span,
        adt_def: AdtDef<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let struct_module = self.resolve_module_name(span, adt_def.did(), variant.def_id);
        let struct_key = afb.pat_atom(span, sym::__struct__);
        let struct_val = afb.pat_atom(span, struct_module);

        let (keys, groups) = subpatterns
            .iter()
            .map(|field| {
                let field_def = &variant.fields[field.field];
                let key = afb.pat_atom(field.pattern.span, field_def.name);
                let alternatives = self.translate_pattern(&field.pattern);
                (key, alternatives)
            })
            .collect::<(SmallVec<[_; 4]>, SmallVec<[_; 4]>)>();

        self.pattern_cartesian_product(span, &groups, |values| {
            let fields = std::iter::once((struct_key, struct_val))
                .chain(keys.iter().copied().zip(values.iter().copied()));
            afb.pat_map(span, fields)
        })
    }

    fn translate_native_record_pattern(
        &mut self,
        span: Span,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
    ) -> Patterns<'arena> {
        let module = self.record_module_for_access(repr.record_module);
        let field_count = variant.fields.len();

        let (groups, fields) = (0..field_count)
            .map(|idx| {
                let alternatives = match subpatterns.iter().find(|field| field.field.index() == idx)
                {
                    Some(field) => self.translate_pattern(&field.pattern),
                    None => self.translator.afb.pat_var(span, kw::Underscore).into(),
                };

                let field = &variant.fields[FieldIdx::from_u32(idx as u32)];
                (alternatives, Ident::new(field.name, span))
            })
            .collect::<(SmallVec<[_; 4]>, SmallVec<[_; 4]>)>();

        self.pattern_cartesian_product(span, &groups, |values| {
            self.translator.afb.pat_record(
                span,
                module,
                variant.name,
                fields.iter().copied().zip(values.iter().copied()),
            )
        })
    }

    fn translate_tagged_pattern(
        &mut self,
        span: Span,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
    ) -> Patterns<'arena> {
        let field_count = variant.fields.len();
        let groups = (0..field_count)
            .map(|idx| match subpatterns.iter().find(|field| field.field.index() == idx) {
                Some(field) => self.translate_pattern(&field.pattern),
                None => self.translator.afb.pat_var(span, kw::Underscore).into(),
            })
            .collect::<SmallVec<[_; 4]>>();

        self.pattern_cartesian_product(span, &groups, |fields| {
            let tag = self.translator.afb.pat_atom(span, repr.tag);
            self.translator.afb.pat_tuple(span, std::iter::once(tag).chain(fields.iter().copied()))
        })
    }

    fn resolve_module_name(
        &mut self,
        span: Span,
        adt_def_id: DefId,
        variant_def_id: DefId,
    ) -> Symbol {
        let tcx = self.tcx();

        let target =
            ErlCallTarget::Constructor { nominal_type: adt_def_id, variant: variant_def_id };

        let module_resolution = tcx
            .erl_module_for_target(self.owner.to_def_id(), target)
            .unwrap_or_else(|| span_bug!(span, "adt constructor must resolve to a module"));

        let ErlModuleResolution::Static(module_id) = module_resolution else {
            span_bug!(span, "adt constructor has dynamic module");
        };

        tcx.erl_module_name(module_id)
    }

    pub(super) fn record_module_for_access(&self, record_module: Symbol) -> Option<Symbol> {
        let tcx = self.tcx();
        let current_module = tcx
            .erl_definition_module(self.owner.to_def_id())
            .map(|module_id| tcx.erl_module_name(module_id));

        (current_module != Some(record_module)).then_some(record_module)
    }

    fn resolve_extern_module_name(
        &mut self,
        span: Span,
        adt_def_id: DefId,
        variant_def_id: DefId,
    ) -> Option<Symbol> {
        let tcx = self.tcx();

        let target =
            ErlCallTarget::Constructor { nominal_type: adt_def_id, variant: variant_def_id };

        let call_kind = tcx
            .erl_call_kind(self.owner.to_def_id(), target, None)
            .unwrap_or_else(|| span_bug!(span, "adt constructor didn't classify"));

        // adt construction is always local if possible
        (call_kind == ErlCallKind::Extern)
            .then(|| self.resolve_module_name(span, adt_def_id, variant_def_id))
    }
}
