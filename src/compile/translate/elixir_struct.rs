use rustc_hir::VariantData;
use rustc_span::{Span, Symbol, sym};

use crate::compile::eaf_builder::EafBuilder;
use crate::erlang::abstract_format as eaf;

pub(super) fn struct0_function<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
    mod_name: Symbol,
    variant_data: VariantData<'tcx>,
) -> eaf::ModuleForm<'arena> {
    let base_map = base_struct_map(afb, span, mod_name, variant_data);
    let clause0 = afb.clause(span, [], afb.clause_body(base_map));
    afb.function(span, sym::__struct__, 0, [clause0])
}

pub(super) fn struct1_function<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::ModuleForm<'arena> {
    // {Key, Value}
    let pair_pat =
        afb.pat_tuple(span, [afb.pat_var(span, sym::Key), afb.pat_var(span, sym::Value)]);
    // Acc#{Key := Value}
    let update = afb.expr_map_update_exact(
        span,
        afb.expr_var(span, sym::Acc),
        [(afb.expr_var(span, sym::Key), afb.expr_var(span, sym::Value))],
    );
    // fun({Key, Value}, Acc) -> Acc#{Key := Value} end
    let fun_clause =
        afb.clause(span, [pair_pat, afb.pat_var(span, sym::Acc)], afb.clause_body(update));
    let reducer = afb.expr_fun(span, [fun_clause]);

    // 'Elixir.Enum':reduce(Fields, '__struct__'(), $reducer)
    let reduce_call = afb.expr_remote_call(
        span,
        afb.expr_atom(span, sym::ElixirEnum),
        afb.expr_atom(span, sym::reduce),
        [
            afb.expr_var(span, sym::Fields),
            afb.expr_local_call(span, afb.expr_atom(span, sym::__struct__), []),
            reducer,
        ],
    );

    let clause = afb.clause(span, [afb.pat_var(span, sym::Fields)], afb.clause_body(reduce_call));
    afb.function(span, sym::__struct__, 1, [clause])
}

fn base_struct_map<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
    mod_name: Symbol,
    variant_data: VariantData<'tcx>,
) -> eaf::Expr<'arena> {
    let struct_key = afb.expr_atom(span, sym::__struct__);
    let struct_val = afb.expr_atom(span, mod_name);

    let field_assocs = variant_data.fields().iter().map(|field| {
        let key = afb.expr_atom(field.ident.span, field.ident.name);
        let val = afb.expr_atom(field.span, sym::nil);
        (key, val)
    });

    afb.expr_map(span, std::iter::once((struct_key, struct_val)).chain(field_assocs))
}
