use rustc_data_structures::smallvec::SmallVec;
use rustc_hir::{Expr, ExprField, QPath, StructTailExpr};
use rustc_middle::erl::{ErlAdtRepr, ErlCallKind, ErlCallTarget};
use rustc_middle::span_bug;
use rustc_span::{Ident, Symbol, sym};

use crate::compile::translate::{Scope, Translator};
use crate::erlang::abstract_format as eaf;
use crate::{not_implemented, span_fatal};

impl<'tcx: 'arena, 'arena> Translator<'tcx, 'arena> {
    pub(super) fn struct_tuple_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        callee: &'tcx Expr<'tcx>,
        args: &'tcx [Expr<'tcx>],
        args_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
        erl_call_kind: ErlCallKind,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let Some(target) = self.tcx.resolve_erl_call_target(expr) else {
            span_bug!(expr.span, "call target not found");
        };

        let ErlCallTarget::Constructor { nominal_type, .. } = target else {
            span_bug!(expr.span, "call target is not a constructor");
        };

        match self.tcx.erl_adt_repr(nominal_type) {
            ErlAdtRepr::Elixir => {
                self.struct_elixir_tuple_expr(expr, callee, args_exprs, erl_call_kind, scope)
            }
            ErlAdtRepr::NativeRecord => {
                self.struct_native_record_tuple_expr(expr, args, args_exprs, erl_call_kind)
            }
        }
    }

    fn struct_elixir_tuple_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        callee: &'tcx Expr<'tcx>,
        args_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
        erl_call_kind: ErlCallKind,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let fn_atom = afb.expr_atom(callee.span, sym::__struct__);
        let base = if erl_call_kind.can_be_local() {
            afb.expr_local_call(expr.span, fn_atom, [])
        } else {
            let mod_expr = self.translate_expr(callee, scope);
            afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
        };
        let updates = args_exprs
            .into_iter()
            .enumerate()
            .map(|(i, arg)| (afb.expr_atom(expr.span, Symbol::intern(&i.to_string())), arg));

        afb.expr_map_update_exact(expr.span, base, updates)
    }

    fn struct_native_record_tuple_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        args: &'tcx [Expr<'tcx>],
        args_exprs: SmallVec<[eaf::Expr<'arena>; 8]>,
        erl_call_kind: ErlCallKind,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let Some(target) = self.tcx.resolve_erl_call_target(expr) else {
            span_bug!(expr.span, "call target not found");
        };

        let ErlCallTarget::Constructor { nominal_type, variant, .. } = target else {
            span_bug!(expr.span, "call target is not a constructor");
        };

        let fields = args.iter().zip(args_exprs.into_iter()).enumerate().map(|(i, arg)| {
            let field_name = Symbol::intern(&i.to_string());
            let ident = Ident::new(field_name, arg.0.span);
            (ident, arg.1)
        });

        let module = if erl_call_kind.can_be_local() {
            None
        } else {
            let Some(module) = self.tcx.erl_definition_module(nominal_type) else {
                span_bug!(expr.span, "struct has no resolved Erlang module");
            };
            Some(self.modules.module_name(module))
        };

        // TODO: this unwrap is how we can distinguish between struct-created-by-call
        // and struct-created-by-{}.
        let name = self.tcx.item_name(variant.unwrap());
        afb.expr_record_create(expr.span, module, name, fields)
    }

    pub(super) fn struct_struct_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        qpath: &'tcx QPath<'tcx>,
        fields: &'tcx [ExprField<'tcx>],
        tail: StructTailExpr<'tcx>,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        self.struct_elixir_struct_expr(expr, qpath, fields, tail, scope)
    }

    fn struct_elixir_struct_expr(
        &self,
        expr: &'tcx Expr<'tcx>,
        qpath: &'tcx QPath<'tcx>,
        fields: &'tcx [ExprField<'tcx>],
        tail: StructTailExpr<'tcx>,
        scope: &mut Scope,
    ) -> eaf::Expr<'arena> {
        let afb = &self.afb;

        let base = match tail {
            StructTailExpr::None => {
                let Some(def_id) = self.qpath_res(qpath, expr.hir_id).opt_def_id() else {
                    span_bug!(expr.span, "struct {qpath:?} not found");
                };

                let target = ErlCallTarget::Constructor {
                    nominal_type: def_id,
                    variant: None,
                    args: self.tcx.typeck(expr.hir_id.owner.def_id).node_args(expr.hir_id),
                };

                let Some(erl_call_kind) =
                    self.tcx.erl_call_kind_from(expr.hir_id.owner.to_def_id(), target)
                else {
                    span_bug!(expr.span, "call kind not found");
                };

                // A struct literal lowers to a synthetic `__struct__()` call to the
                // struct's own module - there is no call `Expr`, so classify by def id.
                let fn_atom = afb.expr_atom(expr.span, sym::__struct__);

                if erl_call_kind.can_be_local() {
                    afb.expr_local_call(expr.span, fn_atom, [])
                } else {
                    let mod_expr = self.translate_qpath(qpath, expr.hir_id, scope);
                    afb.expr_remote_call(expr.span, mod_expr, fn_atom, [])
                }
            }

            StructTailExpr::Base(base) => self.translate_expr(base, scope),

            StructTailExpr::DefaultFields(_) | StructTailExpr::NoneWithError(_) => {
                not_implemented!(self.tcx, expr.span, "struct with default field values", tail);
            }
        };

        let translated_fields_iter = fields.iter().map(|field| {
            let key = afb.expr_atom(field.ident.span, field.ident.name);
            (key, self.translate_expr(field.expr, scope))
        });

        afb.expr_map_update_exact(expr.span, base, translated_fields_iter)
    }
}
