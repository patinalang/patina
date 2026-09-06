use rustc_abi::FieldIdx;
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::shape::ErlShape;
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution, ErlRepr, ErlVariantRepr};
use rustc_middle::span_bug;
use rustc_middle::thir::{AdtExpr, AdtExprBase, Expr, FieldPat};
use rustc_middle::ty::{self, AdtDef, Ty, VariantDef};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::body::{BodyCx, GuardedPattern, Patterns};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_adt_pattern(
        &mut self,
        span: Span,
        adt_ty: Ty<'tcx>,
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
            ErlRepr::Transparent => self.translate_transparent_pattern(
                span,
                adt_ty,
                adt_def,
                variant,
                subpatterns,
                repr,
            ),
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
            ErlRepr::Transparent => self.translate_transparent(expr, adt_expr, variant, repr),
        }
    }

    fn translate_transparent(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let value = match &adt_expr.base {
            AdtExprBase::None => {
                let [field] = adt_expr.fields.as_ref() else {
                    span_bug!(
                        expr.span,
                        "transparent ADT construction has num of fields other than 1: {:?}",
                        adt_expr.fields
                    )
                };
                self.translate_expr(field.expr)
            }

            AdtExprBase::Base(base) => match adt_expr.fields.as_ref() {
                [] => self.translate_expr(base.base),
                [field] => self.translate_expr(field.expr),
                _ => {
                    span_bug!(
                        expr.span,
                        "transparent ADT update has more than one field: {:?}",
                        adt_expr.fields
                    )
                }
            },

            AdtExprBase::DefaultFields(_) => {
                not_implemented!(self.tcx(), expr.span, "transparent ADT with default fields")
            }
        };

        let guard_shapes = self.tcx().erl_open_transparent_guards((variant.def_id, adt_expr.args));
        if guard_shapes.is_empty() {
            // The variant is either not open-shape, or can't be mistaken with any other variant.
            return value;
        }

        let value_var = self.scopes.fresh_var(sym::value, true);
        let value_var_pat = afb.pat_var(expr.span, value_var.erl_name);
        let value_var_expr = afb.expr_var(expr.span, value_var.erl_name);
        let wrapped =
            afb.expr_tuple(expr.span, [afb.expr_atom(expr.span, repr.tag), value_var_expr]);

        let clauses = guard_shapes
            .iter()
            .map(|shape| {
                let guard = self.guards_for_shape(expr.span, value_var_expr, shape);
                if guard.is_none() {
                    span_bug!(expr.span, "empty guard clause in open transparent construction")
                }
                afb.clause(expr.span, [value_var_pat], guard, wrapped)
            })
            .chain(std::iter::once(afb.clause(expr.span, [value_var_pat], None, value_var_expr)));

        afb.expr_case(expr.span, value, clauses)
    }

    fn translate_transparent_pattern(
        &mut self,
        span: Span,
        adt_ty: Ty<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        let payload_patterns = match subpatterns {
            [] => afb.pat_var(span, kw::Underscore).into(),
            [field] => self.translate_pattern(&field.pattern),
            _ => span_bug!(span, "transparent ADT pattern with more than one field"),
        };

        // A variant with no siblings is never wrapped.
        if adt_def.variants().len() == 1 {
            return payload_patterns;
        }

        let ty::Adt(_, args) = adt_ty.kind() else {
            span_bug!(span, "transparent ADT pattern has non-ADT type {adt_ty}")
        };

        let guard_shapes = self.tcx().erl_open_transparent_guards((variant.def_id, args));
        let wraps = !guard_shapes.is_empty();

        // If wrapping is a possibility, we need to guard against the wrapped shape as well.
        let wrapped = wraps
            .then(|| {
                payload_patterns.clone().into_iter().map(|GuardedPattern(payload, guards)| {
                    let tag = afb.pat_atom(span, repr.tag);
                    GuardedPattern(afb.pat_tuple(span, [tag, payload]), guards)
                })
            })
            .into_iter()
            .flatten();

        let shape_var = self.scopes.fresh_var(sym::value, true);
        let shape_pat = afb.pat_var(span, shape_var.erl_name);
        let shape_expr = afb.expr_var(span, shape_var.erl_name);

        let shape_guards: SmallVec<[eaf::Expr<'arena>; 1]> = if wraps {
            guard_shapes
                .iter()
                .map(|shape| match self.guards_for_shape(span, shape_expr, shape) {
                    Some(guard) => afb.expr_unary_op(span, eaf::UnaryOp::Not, guard),
                    None => afb.expr_atom(span, kw::False),
                })
                .collect()
        } else {
            let payload_ty = variant.fields[FieldIdx::ZERO].ty(self.tcx(), args);
            let shape = self.tcx().erl_shape_of_ty(payload_ty);
            self.guards_for_shape(span, shape_expr, shape).into_iter().collect()
        };

        let unwrapped = payload_patterns.into_iter().map(|GuardedPattern(payload, mut guards)| {
            guards.extend(shape_guards.iter().copied());
            GuardedPattern(afb.pat_match(span, shape_pat, payload), guards)
        });

        wrapped.chain(unwrapped).collect()
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

        let record_name = repr.tag;
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
                repr.tag,
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

    /// Shallow Erlang guard tests which recognize `shape`.
    ///
    /// An empty vector represents an always-true guard (`ErlShape::Any`).
    fn guards_for_shape(
        &self,
        span: Span,
        value: eaf::Expr<'arena>,
        shape: &ErlShape,
    ) -> Option<eaf::Expr<'arena>> {
        let afb = &self.translator.afb;

        let bif = |name: Symbol, args: &[eaf::Expr<'arena>]| {
            afb.expr_remote_call(
                span,
                afb.expr_atom(span, sym::erlang),
                afb.expr_atom(span, name),
                args.iter().copied(),
            )
        };
        let and = |left, right| afb.expr_binary_op(span, eaf::BinaryOp::Andalso, left, right);
        let or = |left, right| afb.expr_binary_op(span, eaf::BinaryOp::Orelse, left, right);
        let eq = |left, right| afb.expr_binary_op(span, eaf::BinaryOp::EqExact, left, right);

        let guard_expr = match shape {
            ErlShape::Atom(atom) => eq(value, afb.expr_atom(span, *atom)),
            ErlShape::AnyAtom => bif(sym::is_atom, &[value]),
            ErlShape::Int => bif(sym::is_integer, &[value]),
            ErlShape::Float => bif(sym::is_float, &[value]),
            ErlShape::Binary => bif(sym::is_binary, &[value]),
            ErlShape::List => bif(sym::is_list, &[value]),
            ErlShape::Fun => bif(sym::is_function, &[value]),
            ErlShape::Tuple { arity, tag } => {
                let tuple = bif(sym::is_tuple, &[value]);
                let size = eq(bif(sym::tuple_size, &[value]), afb.expr_int(span, *arity as u128));
                let tuple_and_size = and(tuple, size);
                if let Some(tag) = tag {
                    let head = bif(sym::element, &[afb.expr_int(span, 1), value]);
                    let tag_match = eq(head, afb.expr_atom(span, *tag));
                    and(tuple_and_size, tag_match)
                } else {
                    tuple_and_size
                }
            }
            ErlShape::AnyTuple => bif(sym::is_tuple, &[value]),
            ErlShape::Map => bif(sym::is_map, &[value]),
            ErlShape::ElixirStruct(module) => and(
                bif(sym::is_map, &[value]),
                eq(
                    bif(sym::map_get, &[afb.expr_atom(span, sym::__struct__), value]),
                    afb.expr_atom(span, *module),
                ),
            ),
            ErlShape::NativeRecord { module, name } => bif(
                sym::is_record,
                &[value, afb.expr_atom(span, *module), afb.expr_atom(span, *name)],
            ),
            ErlShape::Union(shapes) => {
                let mut combined = None;
                for shape in shapes {
                    let test = self.guards_for_shape(span, value, shape)?;
                    combined = Some(match combined {
                        Some(previous) => or(previous, test),
                        None => test,
                    });
                }
                combined?
            }
            ErlShape::Any => return None,
        };

        Some(guard_expr)
    }
}
