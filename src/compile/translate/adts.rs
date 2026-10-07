use rustc_abi::{FieldIdx, VariantIdx};
use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::shape::ErlShape;
use rustc_middle::erl::{
    ErlCallKind, ErlCallTarget, ErlModuleResolution, ErlReprKind, ErlVariantRepr,
};
use rustc_middle::span_bug;
use rustc_middle::thir::{AdtExpr, AdtExprBase, Expr, FieldExpr, FieldPat, Pat};
use rustc_middle::ty::{self, AdtDef, VariantDef};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::body::{BodyCx, GuardedPattern, PatternInput, Patterns};
use crate::compile::translate::record_constructor_name;
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_adt_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant_index: VariantIdx,
        subpatterns: &[FieldPat<'tcx>],
        input: PatternInput,
    ) -> Patterns<'arena> {
        let variant = adt_def.variant(variant_index);
        let repr = self.tcx().erl_variant_repr(variant.def_id);
        match repr.kind {
            ErlReprKind::Atom => self.translator.afb.pat_atom(pat.span, repr.tag).into(),
            ErlReprKind::Int => not_implemented!(self.tcx(), pat.span, "int-represented ADT"),
            ErlReprKind::Tagged => {
                self.translate_tagged_pattern(pat, adt_def, variant_index, subpatterns, repr, input)
            }
            ErlReprKind::Elixir => self.translate_elixir_struct_pattern(
                pat,
                adt_def,
                variant_index,
                subpatterns,
                input,
            ),
            ErlReprKind::NativeRecord => self.translate_native_record_pattern(
                pat,
                adt_def,
                variant_index,
                subpatterns,
                repr,
                input,
            ),
            ErlReprKind::Transparent { .. } => self.translate_transparent_pattern(
                pat,
                adt_def,
                variant_index,
                subpatterns,
                repr,
                input,
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

        let mut fields = SmallVec::<[&FieldExpr; 4]>::new();
        let mut effects = Vec::new();
        for field in &adt_expr.fields {
            if self.tcx().is_erl_phantom_field(variant.fields[field.name].did) {
                let span = self.thir[field.expr].span;
                let value = self.translate_expr(field.expr);
                effects.push(afb.expr_match(span, afb.pat_var(span, kw::Underscore), value));
            } else {
                fields.push(field);
            }
        }

        let repr = self.tcx().erl_variant_repr(variant.def_id);
        let value = match repr.kind {
            ErlReprKind::Atom => afb.expr_atom(expr.span, repr.tag),
            ErlReprKind::Int => not_implemented!(self.tcx(), expr.span, "int-represented ADT"),
            ErlReprKind::Tagged => self.translate_tagged(expr, adt_expr, &fields, variant, repr),
            ErlReprKind::Elixir => self.translate_elixir_struct(expr, adt_expr, &fields, variant),
            ErlReprKind::NativeRecord => {
                self.translate_native_record(expr, adt_expr, &fields, variant, repr)
            }
            ErlReprKind::Transparent { .. } => {
                self.translate_transparent(expr, adt_expr, &fields, variant, repr)
            }
        };

        if effects.is_empty() {
            value
        } else {
            afb.expr_block(expr.span, effects.into_iter().chain([value]))
        }
    }

    fn translate_transparent(
        &mut self,
        expr: &Expr<'tcx>,
        adt_expr: &AdtExpr<'tcx>,
        fields: &[&FieldExpr],
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let value = match &adt_expr.base {
            AdtExprBase::None => {
                let [field] = fields else {
                    span_bug!(
                        expr.span,
                        "transparent ADT construction has num of fields other than 1: {fields:?}",
                    )
                };
                self.translate_expr(field.expr)
            }

            AdtExprBase::Base(base) => match fields {
                [] => self.translate_expr(base.base),
                [field] => self.translate_expr(field.expr),
                _ => {
                    span_bug!(
                        expr.span,
                        "transparent ADT update has more than one field: {fields:?}",
                    )
                }
            },

            AdtExprBase::DefaultFields(_) => {
                not_implemented!(self.tcx(), expr.span, "transparent ADT with default fields")
            }
        };

        let guard_shapes = self.tcx().erl_variant_wrap_guards((variant.def_id, adt_expr.args));
        if guard_shapes.is_empty() {
            // The variant is either not open-shape, or can't be mistaken with any other variant.
            return value;
        }

        let value_var = self.scopes.fresh_var(sym::value);
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
        pat: &Pat<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant_index: VariantIdx,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
        input: PatternInput,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let span = pat.span;
        let adt_ty = pat.ty;
        let variant = adt_def.variant(variant_index);

        let ty::Adt(_, args) = adt_ty.kind() else {
            span_bug!(span, "transparent ADT pattern has non-ADT type {adt_ty}")
        };
        let Some((payload_idx, payload_field)) = self.tcx().erl_single_runtime_field(variant)
        else {
            span_bug!(span, "transparent ADT pattern without a single runtime field")
        };
        let payload_ty = payload_field.ty(self.tcx(), args);

        let payload_patterns = match subpatterns.iter().find(|field| field.field == payload_idx) {
            None => self.translate_wild_pattern(span, payload_ty, input),
            Some(field) => self.translate_pattern(&field.pattern, input),
        };

        // A variant with no siblings is never wrapped.
        if adt_def.variants().len() == 1 {
            return payload_patterns;
        }

        let guard_shapes = self.tcx().erl_variant_wrap_guards((variant.def_id, args));
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

        let shape_var = self.scopes.fresh_var(sym::value);
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
        } else if matches!(input, PatternInput::UntrustedRepresentation) {
            // Translating the payload under `UntrustedRepresentation` already establishes its
            // shape. Another guard is redundant.
            SmallVec::new()
        } else {
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
        fields: &[&FieldExpr],
        variant: &VariantDef,
        repr: ErlVariantRepr,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        match &adt_expr.base {
            AdtExprBase::None => {
                let tag = afb.expr_atom(expr.span, repr.tag);
                let values = self.tcx().erl_runtime_fields(variant).map(|(idx, _)| {
                    let field =
                        fields.iter().find(|field| field.name == idx).unwrap_or_else(|| {
                            span_bug!(
                                expr.span,
                                "tagged tuple construction is missing field {idx:?}"
                            )
                        });
                    self.translate_expr(field.expr)
                });

                afb.expr_tuple(expr.span, std::iter::once(tag).chain(values))
            }
            AdtExprBase::Base(base) => {
                let mut value = self.translate_expr(base.base);

                for field in fields {
                    let field_span = self.thir[field.expr].span;
                    let field_value = self.translate_expr(field.expr);
                    let position =
                        self.tcx().erl_runtime_field_position(variant, field.name).unwrap_or_else(
                            || span_bug!(field_span, "update of a phantom field in a tagged tuple"),
                        );
                    value = afb.expr_remote_call(
                        field_span,
                        afb.expr_atom(field_span, sym::erlang),
                        afb.expr_atom(field_span, sym::setelement),
                        [afb.expr_int(field_span, position as u128 + 2), value, field_value],
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
        fields: &[&FieldExpr],
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

        let updates = fields.iter().map(|field| {
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
        fields: &[&FieldExpr],
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

        let fields = fields.iter().map(|field| {
            let field_def = &variant.fields[field.name];
            let value = self.translate_expr(field.expr);
            let ident = Ident::new(field_def.name, self.thir[field.expr].span);
            (field.name, ident, value)
        });

        match (base, module) {
            (None, Some(module)) => {
                // Use a generated remote constructor function.
                // We do this because crating a record requires the defining module to be loaded,
                // and unlike functions, it does not autoload on use.
                // The constructor takes every field, in declaration order.
                let mut fields = fields.into_iter().collect::<SmallVec<[_; 8]>>();
                fields.sort_by_key(|&(index, ..)| index);
                afb.expr_remote_call(
                    expr.span,
                    afb.expr_atom(expr.span, module),
                    afb.expr_atom(expr.span, record_constructor_name(record_name)),
                    fields.into_iter().map(|(_, _, value)| value),
                )
            }
            (None, None) => afb.expr_record_create(
                expr.span,
                None,
                record_name,
                fields.map(|(_, ident, value)| (ident, value)),
            ),
            (Some(base), module) => afb.expr_record_update(
                expr.span,
                base,
                module,
                record_name,
                fields.map(|(_, ident, value)| (ident, value)),
            ),
        }
    }

    fn translate_adt_fields(
        &mut self,
        pat: &Pat<'tcx>,
        variant: &VariantDef,
        subpatterns: &[FieldPat<'tcx>],
        input: PatternInput,
        fields: impl IntoIterator<Item = FieldIdx>,
    ) -> SmallVec<[(Ident, Patterns<'arena>); 4]> {
        let ty::Adt(_, args) = pat.ty.kind() else {
            span_bug!(pat.span, "ADT pattern has non-ADT type {}", pat.ty)
        };

        fields
            .into_iter()
            .map(|idx| {
                let field_def = &variant.fields[idx];
                let field = subpatterns.iter().find(|field| field.field == idx);
                let span = field.map_or(pat.span, |field| field.pattern.span);
                let patterns = if let Some(field) = field {
                    self.translate_pattern(&field.pattern, input)
                } else {
                    self.translate_wild_pattern(pat.span, field_def.ty(self.tcx(), args), input)
                };
                (Ident::new(field_def.name, span), patterns)
            })
            .collect()
    }

    fn translate_elixir_struct_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant_index: VariantIdx,
        subpatterns: &[FieldPat<'tcx>],
        input: PatternInput,
    ) -> Patterns<'arena> {
        let tcx = self.tcx();
        let afb = &self.translator.afb;
        let span = pat.span;
        let variant = adt_def.variant(variant_index);

        let struct_module = self.resolve_module_name(span, adt_def.did(), variant.def_id);
        let struct_key = afb.expr_atom(span, sym::__struct__);
        let struct_val = afb.pat_atom(span, struct_module);

        let fields = match input {
            PatternInput::UntrustedRepresentation => self.translate_adt_fields(
                pat,
                variant,
                subpatterns,
                input,
                tcx.erl_runtime_fields(variant).map(|(idx, _)| idx),
            ),
            PatternInput::Typed => self.translate_adt_fields(
                pat,
                variant,
                subpatterns,
                input,
                subpatterns
                    .iter()
                    .map(|field| field.field)
                    .filter(|&idx| !tcx.is_erl_phantom_field(variant.fields[idx].did)),
            ),
        };
        let (keys, groups): (SmallVec<[_; 4]>, SmallVec<[_; 4]>) = fields
            .into_iter()
            .map(|(field, patterns)| (afb.expr_atom(field.span, field.name), patterns))
            .unzip();

        self.pattern_cartesian_product(span, &groups, |values| {
            let fields = std::iter::once((struct_key, struct_val))
                .chain(keys.iter().copied().zip(values.iter().copied()));
            afb.pat_map(span, fields)
        })
    }

    fn translate_native_record_pattern(
        &mut self,
        pat: &Pat<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant_index: VariantIdx,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
        input: PatternInput,
    ) -> Patterns<'arena> {
        let span = pat.span;
        let variant = adt_def.variant(variant_index);
        let module = self.record_module_for_access(repr.record_module);

        let (fields, groups): (SmallVec<[_; 4]>, SmallVec<[_; 4]>) = self
            .translate_adt_fields(
                pat,
                variant,
                subpatterns,
                input,
                self.tcx().erl_runtime_fields(variant).map(|(idx, _)| idx),
            )
            .into_iter()
            .map(|(field, patterns)| (Ident::new(field.name, span), patterns))
            .unzip();

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
        pat: &Pat<'tcx>,
        adt_def: AdtDef<'tcx>,
        variant_index: VariantIdx,
        subpatterns: &[FieldPat<'tcx>],
        repr: ErlVariantRepr,
        input: PatternInput,
    ) -> Patterns<'arena> {
        let span = pat.span;
        let variant = adt_def.variant(variant_index);
        let groups = self
            .translate_adt_fields(
                pat,
                variant,
                subpatterns,
                input,
                self.tcx().erl_runtime_fields(variant).map(|(idx, _)| idx),
            )
            .into_iter()
            .map(|(_, patterns)| patterns)
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

        if matches!(tcx.def_kind(self.owner), DefKind::Const { .. } | DefKind::AssocConst { .. }) {
            return Some(record_module);
        }

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
    /// `None` represents an always-true guard (`ErlShape::Any`).
    pub(super) fn guards_for_shape(
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
            ErlShape::Bitstring => bif(sym::is_bitstring, &[value]),
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
