use rustc_abi::FieldIdx;
use rustc_hir::def::CtorKind;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::{ErlAdtRepr, ErlCallKind, ErlCallTarget, ErlModuleResolution};
use rustc_middle::span_bug;
use rustc_middle::thir::{AdtExpr, AdtExprBase, Expr, FieldPat};
use rustc_middle::ty::{AdtDef, VariantDef};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::body::BodyCx;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_adt_pattern(
        &mut self,
        span: Span,
        adt_def: AdtDef<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
    ) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;

        // Unit structs always compile down to atom
        if variant.ctor_kind() == Some(CtorKind::Const) {
            let name = self.translator.modules.constructor_name(variant.def_id);
            return afb.pat_atom(span, name);
        }

        match self.tcx().erl_adt_repr(adt_def.did()) {
            ErlAdtRepr::Elixir => {
                self.translate_elixir_struct_pattern(span, adt_def, variant, subpatterns)
            }
            ErlAdtRepr::NativeRecord => {
                self.translate_native_record_pattern(span, adt_def, variant, subpatterns)
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

        // Unit structs always compile down to atom
        if variant.ctor_kind() == Some(CtorKind::Const) {
            let name = self.translator.modules.constructor_name(variant.def_id);
            return afb.expr_atom(expr.span, name);
        }

        match self.tcx().erl_adt_repr(adt_def.did()) {
            ErlAdtRepr::Elixir => self.translate_elixir_struct(expr, adt_expr, variant),
            ErlAdtRepr::NativeRecord => self.translate_native_record(expr, adt_expr, variant),
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
    ) -> eaf::Expr<'arena> {
        let tcx = self.tcx();
        let afb = &self.translator.afb;

        let base = match &adt_expr.base {
            AdtExprBase::None => None,
            AdtExprBase::Base(base) => Some(self.translate_expr(base.base)),
            AdtExprBase::DefaultFields(_) => {
                not_implemented!(tcx, expr.span, "native record with base", adt_expr.base);
            }
        };

        let record_name = variant.name;
        let module =
            self.resolve_extern_module_name(expr.span, adt_expr.adt_def.did(), variant.def_id);

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
    ) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;

        // Always include the `__struct__` discriminator.
        let struct_module = self.resolve_module_name(span, adt_def.did(), variant.def_id);
        let struct_key = afb.pat_atom(span, sym::__struct__);
        let struct_val = afb.pat_atom(span, struct_module);

        let fields =
            std::iter::once((struct_key, struct_val)).chain(subpatterns.iter().map(|field_pat| {
                let field_def = &variant.fields[field_pat.field];
                let key = afb.pat_atom(field_pat.pattern.span, field_def.name);
                let value = self.translate_pattern(&field_pat.pattern);
                (key, value)
            }));

        afb.pat_map(span, fields)
    }

    fn translate_native_record_pattern(
        &mut self,
        span: Span,
        adt_def: AdtDef<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
    ) -> eaf::Pattern<'arena> {
        let afb = &self.translator.afb;

        let module = self.resolve_extern_module_name(span, adt_def.did(), variant.def_id);

        let field_count = variant.fields.len();

        let fields = (0..field_count).map(|idx| {
            let val = match subpatterns.iter().find(|field_pat| field_pat.field.index() == idx) {
                Some(field_pat) => self.translate_pattern(&field_pat.pattern),
                None => afb.pat_var(span, kw::Underscore),
            };

            let field = &variant.fields[FieldIdx::from_u32(idx as u32)];
            (Ident::new(field.name, span), val)
        });

        afb.pat_record(span, module, variant.name, fields)
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

        self.translator.modules.module_name(module_id)
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
