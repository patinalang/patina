use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::def_id::DefId;
use rustc_middle::erl::etir::{self, AdtRepr, PatternInput};
use rustc_middle::erl::shape::ErlShape;
use rustc_middle::erl::{ErlCallKind, ErlCallTarget, ErlModuleResolution};
use rustc_middle::{span_bug, ty};
use rustc_span::{Ident, Span, Symbol, kw, sym};

use crate::compile::translate::body::{BodyCx, GuardedPattern, Patterns};
use crate::compile::translate::record_constructor_name;
use crate::erlang::abstract_format as eaf;

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn translate_adt_pattern(
        &mut self,
        pat: &etir::Pat<'tcx>,
        adt: &etir::AdtPat<'tcx>,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;

        if adt
            .fields
            .iter()
            .any(|field| field.ty.is_privately_uninhabited(self.tcx(), self.typing_env))
        {
            // An uninhibited variant (such as when matching `Option<!>`) can never match,
            // so it's cleaner to never emit a match clause for it.
            for field in &adt.fields {
                self.translate_pattern(field.pattern);
            }
            return Patterns::new();
        }

        match adt.repr {
            AdtRepr::Atom { tag } => afb.pat_atom(pat.span, tag).into(),
            AdtRepr::Tagged { tag } => self.translate_tagged_pattern(pat, adt, tag),
            AdtRepr::Elixir { adt: adt_def_id, variant } => {
                self.translate_elixir_struct_pattern(pat, adt, adt_def_id, variant)
            }
            AdtRepr::NativeRecord { record_module, tag } => {
                self.translate_native_record_pattern(pat, adt, record_module, tag)
            }
            AdtRepr::Transparent { tag, variant, has_siblings } => {
                self.translate_transparent_pattern(pat, adt, tag, variant, has_siblings)
            }
        }
    }

    pub(super) fn translate_adt(
        &mut self,
        expr: &etir::Expr<'tcx>,
        adt: &etir::AdtExpr,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        match adt.repr {
            AdtRepr::Atom { tag } => afb.expr_atom(expr.span, tag),
            AdtRepr::Tagged { tag } => self.translate_tagged(expr, adt, tag),
            AdtRepr::Elixir { adt: adt_def_id, variant } => {
                self.translate_elixir_struct(expr, adt, adt_def_id, variant)
            }
            AdtRepr::NativeRecord { record_module, tag } => {
                self.translate_native_record(expr, adt, record_module, tag)
            }
            AdtRepr::Transparent { tag, variant, .. } => {
                self.translate_transparent(expr, adt, variant, tag)
            }
        }
    }

    fn translate_transparent(
        &mut self,
        expr: &etir::Expr<'tcx>,
        adt: &etir::AdtExpr,
        variant: DefId,
        tag: Symbol,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let fields = &adt.fields[..];

        let value = match adt.base {
            None => {
                let [field] = fields else {
                    span_bug!(
                        expr.span,
                        "transparent ADT construction has num of fields other than 1: {fields:?}",
                    )
                };
                self.translate_expr(field.value)
            }

            Some(base) => match fields {
                [] => self.translate_expr(base),
                [field] => self.translate_expr(field.value),
                fields => {
                    span_bug!(
                        expr.span,
                        "transparent ADT update has more than one field: {fields:?}",
                    )
                }
            },
        };

        let ty::Adt(_, args) = *expr.ty.kind() else {
            span_bug!(expr.span, "transparent ADT construction has non-ADT type {}", expr.ty)
        };
        let guard_shapes = self.tcx().erl_variant_wrap_guards((variant, args));
        if guard_shapes.is_empty() {
            // The variant is either not open-shape, or can't be mistaken with any other variant.
            return value;
        }

        let value_var = self.scopes.fresh_var(sym::value);
        let value_var_pat = afb.pat_var(expr.span, value_var.erl_name);
        let value_var_expr = afb.expr_var(expr.span, value_var.erl_name);
        let wrapped = afb.expr_tuple(expr.span, [afb.expr_atom(expr.span, tag), value_var_expr]);

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
        pat: &etir::Pat<'tcx>,
        adt: &etir::AdtPat<'tcx>,
        tag: Symbol,
        variant: DefId,
        has_siblings: bool,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let span = pat.span;
        let adt_ty = pat.ty;

        let ty::Adt(_, args) = adt_ty.kind() else {
            span_bug!(span, "transparent ADT pattern has non-ADT type {adt_ty}")
        };

        let [payload] = &adt.fields.raw[..] else {
            span_bug!(span, "transparent ADT pattern without a single runtime field")
        };

        let payload_patterns = self.translate_pattern(payload.pattern);

        // A variant with no siblings is never wrapped.
        if !has_siblings {
            return payload_patterns;
        }

        let guard_shapes = self.tcx().erl_variant_wrap_guards((variant, args));
        let wraps = !guard_shapes.is_empty();

        // If wrapping is a possibility, we need to guard against the wrapped shape as well.
        let wrapped = wraps
            .then(|| {
                payload_patterns.clone().into_iter().map(|GuardedPattern(payload, guards)| {
                    let tag = afb.pat_atom(span, tag);
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
        } else if matches!(adt.input, PatternInput::UntrustedRepresentation) {
            // Translating the payload under `UntrustedRepresentation` already establishes its
            // shape. Another guard is redundant.
            SmallVec::new()
        } else {
            let shape = self.tcx().erl_variant_discriminating_shape(variant, args, payload.ty);
            self.guards_for_shape(span, shape_expr, &shape).into_iter().collect()
        };

        let unwrapped = payload_patterns.into_iter().map(|GuardedPattern(payload, mut guards)| {
            guards.extend(shape_guards.iter().copied());
            GuardedPattern(afb.pat_match(span, shape_pat, payload), guards)
        });

        wrapped.chain(unwrapped).collect()
    }

    fn translate_tagged(
        &mut self,
        expr: &etir::Expr<'tcx>,
        adt: &etir::AdtExpr,
        tag: Symbol,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        match adt.base {
            None => {
                let tag = afb.expr_atom(expr.span, tag);
                let values = adt.fields.iter().map(|field| self.translate_expr(field.value));

                afb.expr_tuple(expr.span, std::iter::once(tag).chain(values))
            }
            Some(base) => {
                let mut value = self.translate_expr(base);

                for field in &adt.fields {
                    let field_span = self.etir[field.value].span;
                    let field_value = self.translate_expr(field.value);
                    let position = field.position.as_u32();
                    value = afb.erlang_bif(
                        field_span,
                        sym::setelement,
                        [afb.expr_int(field_span, position as u128 + 2), value, field_value],
                    );
                }

                value
            }
        }
    }

    fn translate_elixir_struct(
        &mut self,
        expr: &etir::Expr<'tcx>,
        adt: &etir::AdtExpr,
        adt_def_id: DefId,
        variant: DefId,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let base = match adt.base {
            None => {
                let fn_atom = afb.expr_atom(expr.span, sym::__struct__);

                match self.resolve_extern_module_name(expr.span, adt_def_id, variant) {
                    Some(module) => {
                        let mod_expr = afb.expr_atom(expr.span, module);
                        afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
                    }
                    None => afb.expr_local_call(expr.span, fn_atom, []),
                }
            }
            Some(base) => self.translate_expr(base),
        };

        let updates = adt.fields.iter().map(|field| {
            let value = self.translate_expr(field.value);
            let key = afb.expr_atom(self.etir[field.value].span, field.name);
            (key, value)
        });

        afb.expr_map_update_exact(expr.span, base, updates)
    }

    fn translate_native_record(
        &mut self,
        expr: &etir::Expr<'tcx>,
        adt: &etir::AdtExpr,
        record_module: Symbol,
        record_name: Symbol,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;

        let base = adt.base.map(|base| self.translate_expr(base));
        let module = self.record_module_for_access(record_module);

        let fields = adt.fields.iter().map(|field| {
            let value = self.translate_expr(field.value);
            let ident = Ident::new(field.name, self.etir[field.value].span);
            (ident, value)
        });

        match (base, module) {
            (None, Some(module)) => {
                // Use a generated remote constructor function.
                // We do this because crating a record requires the defining module to be loaded,
                // and unlike functions, it does not autoload on use.
                // The constructor takes every field, in declaration order.
                afb.expr_remote_call(
                    expr.span,
                    afb.expr_atom(expr.span, module),
                    afb.expr_atom(expr.span, record_constructor_name(record_name)),
                    fields.map(|(_, value)| value),
                )
            }
            (None, None) => afb.expr_record_create(expr.span, None, record_name, fields),
            (Some(base), module) => {
                afb.expr_record_update(expr.span, base, module, record_name, fields)
            }
        }
    }

    fn translate_adt_fields(
        &mut self,
        adt: &etir::AdtPat<'tcx>,
        only_mentioned: bool,
    ) -> SmallVec<[(Ident, Patterns<'arena>); 4]> {
        adt.fields
            .iter()
            .filter(|field| !only_mentioned || !self.etir[field.pattern].kind.is_wild())
            .map(|field| {
                let span = self.etir[field.pattern].span;
                let patterns = self.translate_pattern(field.pattern);
                (Ident::new(field.name, span), patterns)
            })
            .collect()
    }

    fn translate_elixir_struct_pattern(
        &mut self,
        pat: &etir::Pat<'tcx>,
        adt: &etir::AdtPat<'tcx>,
        adt_def_id: DefId,
        variant: DefId,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let span = pat.span;

        let struct_module = self.resolve_module_name(span, adt_def_id, variant);
        let struct_key = afb.expr_atom(span, sym::__struct__);
        let struct_val = afb.pat_atom(span, struct_module);

        let fields = self.translate_adt_fields(adt, matches!(adt.input, PatternInput::Typed));
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
        pat: &etir::Pat<'tcx>,
        adt: &etir::AdtPat<'tcx>,
        record_module: Symbol,
        tag: Symbol,
    ) -> Patterns<'arena> {
        let span = pat.span;
        let module = self.record_module_for_access(record_module);

        let (fields, groups): (SmallVec<[_; 4]>, SmallVec<[_; 4]>) =
            self.translate_adt_fields(adt, false).into_iter().unzip();

        self.pattern_cartesian_product(span, &groups, |values| {
            self.translator.afb.pat_record(
                span,
                module,
                tag,
                fields.iter().copied().zip(values.iter().copied()),
            )
        })
    }

    fn translate_tagged_pattern(
        &mut self,
        pat: &etir::Pat<'tcx>,
        adt: &etir::AdtPat<'tcx>,
        tag: Symbol,
    ) -> Patterns<'arena> {
        let afb = &self.translator.afb;
        let span = pat.span;
        let groups = self
            .translate_adt_fields(adt, false)
            .into_iter()
            .map(|(_, patterns)| patterns)
            .collect::<SmallVec<[_; 4]>>();

        self.pattern_cartesian_product(span, &groups, |fields| {
            let tag = afb.pat_atom(span, tag);
            self.translator.afb.pat_tuple(span, std::iter::once(tag).chain(fields.iter().copied()))
        })
    }

    pub(super) fn translate_field(
        &mut self,
        span: Span,
        lhs: etir::ExprId,
        repr: AdtRepr,
        position: etir::FieldIdx,
        name: Symbol,
    ) -> eaf::Expr<'arena> {
        let afb = &self.translator.afb;
        let value = self.translate_expr(lhs);

        match repr {
            AdtRepr::Atom { .. } => {
                span_bug!(span, "field access on atom-represented ADT (unit variant)")
            }
            AdtRepr::Tagged { .. } => {
                let index = afb.expr_int(span, (position.as_u32() + 2).into());
                afb.erlang_bif(span, sym::element, [index, value])
            }
            AdtRepr::Elixir { .. } => {
                let lhs_span = self.etir[lhs].span;
                afb.erlang_bif(span, sym::map_get, [afb.expr_atom(lhs_span, name), value])
            }
            AdtRepr::NativeRecord { record_module, tag } => {
                let module = self.record_module_for_access(record_module);
                afb.expr_record_field(span, value, module, tag, name)
            }
            AdtRepr::Transparent { .. } => value,
        }
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
            .erl_module_for_target(self.owner, target)
            .unwrap_or_else(|| span_bug!(span, "adt constructor must resolve to a module"));

        let ErlModuleResolution::Static(module_id) = module_resolution else {
            span_bug!(span, "adt constructor has dynamic module");
        };

        tcx.erl_module_name(module_id)
    }

    fn record_module_for_access(&self, record_module: Symbol) -> Option<Symbol> {
        let tcx = self.tcx();

        let current_module =
            tcx.erl_definition_module(self.owner).map(|module_id| tcx.erl_module_name(module_id));

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
            .erl_module_for_target(self.owner, target)
            .and_then(|module| tcx.erl_call_kind(self.owner, module, false))
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
            afb.erlang_bif(span, name, args.iter().copied())
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
            ErlShape::Tuple { arity, first } => {
                let tuple = bif(sym::is_tuple, &[value]);
                let size = eq(bif(sym::tuple_size, &[value]), afb.expr_int(span, *arity as u128));
                let tuple_and_size = and(tuple, size);
                let head = bif(sym::element, &[afb.expr_int(span, 1), value]);
                match self.guards_for_shape(span, head, first) {
                    Some(head_match) if *arity > 0 => and(tuple_and_size, head_match),
                    _ => tuple_and_size,
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
