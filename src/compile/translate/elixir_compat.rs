use rustc_hir::VariantData;
use rustc_span::{Span, Symbol, kw, sym};

use crate::compile::eaf_builder::EafBuilder;
use crate::erlang::abstract_format as eaf;

pub(super) fn info_function<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
    variant_data: Option<VariantData<'tcx>>,
) -> eaf::ModuleForm<'arena> {
    let clauses = [
        info_functions_clause(afb, span),
        info_macros_clause(afb, span),
        info_struct_clause(afb, span, variant_data),
        info_exports_md5_clause(afb, span),
        info_deprecated_clause(afb, span),
        info_default_clause(afb, span),
    ];

    afb.function(span, sym::__info__, 1, clauses)
}

fn info_functions_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::Clause<'arena> {
    // [{module_info, 0}, {module_info, 1}, {__info__, 1}]
    let unwanted_exports = afb.expr_list(
        span,
        [
            afb.expr_tuple(span, [afb.expr_atom(span, sym::__info__), afb.expr_int(span, 1)]),
            afb.expr_tuple(span, [afb.expr_atom(span, sym::module_info), afb.expr_int(span, 0)]),
            afb.expr_tuple(span, [afb.expr_atom(span, sym::module_info), afb.expr_int(span, 1)]),
        ],
        None,
    );
    // module_info(exports)
    let exports = afb.expr_local_call(
        span,
        afb.expr_atom(span, sym::module_info),
        [afb.expr_atom(span, sym::exports)],
    );
    // lists:sort(module_info(exports))
    let exports = afb.expr_remote_call(
        span,
        afb.expr_atom(span, sym::lists),
        afb.expr_atom(span, sym::sort),
        [exports],
    );
    // ordsets:subtract(lists:sort(module_info(exports)), [{__info__, 1}, {module_info, 0}, {module_info, 1}])
    let exports = afb.expr_remote_call(
        span,
        afb.expr_atom(span, sym::ordsets),
        afb.expr_atom(span, sym::subtract),
        [exports, unwanted_exports],
    );

    // (functions) ->
    //   ordsets:subtract(lists:sort(module_info(exports)), [{__info__, 1}, {module_info, 0}, {module_info, 1}]).
    let arg = afb.pat_atom(span, sym::functions);
    afb.clause(span, [arg], None, exports)
}

fn info_macros_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::Clause<'arena> {
    // (macros) -> []
    let body = afb.expr_nil(span);
    let arg = afb.pat_atom(span, sym::macros);
    afb.clause(span, [arg], None, body)
}

fn info_struct_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
    variant_data: Option<VariantData<'tcx>>,
) -> eaf::Clause<'arena> {
    let variant_data = match variant_data {
        Some(variant_data) => variant_data,
        None => {
            let body = afb.expr_atom(span, sym::nil);
            let arg = afb.pat_atom(span, kw::Struct);
            return afb.clause(span, [arg], None, body);
        }
    };

    let list_elems = variant_data.fields().iter().map(|field| {
        // field => $field_name
        let field_key = afb.expr_atom(field.ident.span, sym::field);
        let field_val = afb.expr_atom(field.ident.span, field.ident.name);

        // default => nil
        let default_key = afb.expr_atom(field.span, kw::Default);
        let default_val = afb.expr_atom(field.span, sym::nil);

        // required => false
        let required_key = afb.expr_atom(field.span, sym::required);
        let required_val = afb.expr_atom(field.span, kw::False);

        // #{field => $field_name, default => nil, required => false}
        afb.expr_map(
            span,
            [(field_key, field_val), (default_key, default_val), (required_key, required_val)],
        )
    });

    // (struct) -> [#{field => $field_name, default => nil, required => false}, ...]
    let body = afb.expr_list(span, list_elems, None);
    let arg = afb.pat_atom(span, kw::Struct);
    afb.clause(span, [arg], None, body)
}

fn info_exports_md5_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::Clause<'arena> {
    let s = span;

    // module_info(exports)
    let exports = afb.expr_local_call(
        s,
        afb.expr_atom(s, sym::module_info),
        [afb.expr_atom(s, sym::exports)],
    );
    //  lists:sort(module_info(exports))
    let exports = afb.expr_remote_call(
        s,
        afb.expr_atom(s, sym::lists),
        afb.expr_atom(s, sym::sort),
        [exports],
    );
    // '__info__'(struct)
    let strukt =
        afb.expr_local_call(s, afb.expr_atom(s, sym::__info__), [afb.expr_atom(s, kw::Struct)]);
    // {lists:sort(module_info(exports)), [], '__info__'(struct)}
    let tuple = afb.expr_tuple(s, [exports, afb.expr_nil(s), strukt]);
    // [deterministic]
    let deterministic_list = afb.expr_list(s, [afb.expr_atom(s, sym::deterministic)], None);
    // term_to_binary({lists:sort(module_info(exports)), [], '__info__'(struct)}, [deterministic])
    let term_to_binary = afb.expr_remote_call(
        s,
        afb.expr_atom(s, sym::erlang),
        afb.expr_atom(s, sym::term_to_binary),
        [tuple, deterministic_list],
    );
    // erlang:md5(term_to_binary({lists:sort(module_info(exports)), [], Struct}, [deterministic])).
    let erlang_md5 = afb.expr_remote_call(
        s,
        afb.expr_atom(s, sym::erlang),
        afb.expr_atom(s, sym::md5),
        [term_to_binary],
    );

    // (exports_md5) ->
    //     erlang:md5(term_to_binary({lists:sort(module_info(exports)), [], Struct}, [deterministic])).
    let arg = afb.pat_atom(span, sym::exports_md5);
    afb.clause(span, [arg], None, erlang_md5)
}

fn info_deprecated_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::Clause<'arena> {
    // (deprecated) -> []
    let body = afb.expr_nil(span);
    let arg = afb.pat_atom(span, sym::deprecated);
    afb.clause(span, [arg], None, body)
}

fn info_default_clause<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
) -> eaf::Clause<'arena> {
    let arg = afb.pat_var(span, sym::Key);

    let body = afb.expr_local_call(
        span,
        afb.expr_atom(span, sym::module_info),
        [afb.expr_var(span, sym::Key)],
    );

    // (Key) -> module_info(Key)
    afb.clause(span, [arg], None, body)
}

pub(super) fn struct0_function<'tcx, 'arena>(
    afb: &EafBuilder<'tcx, 'arena>,
    span: Span,
    mod_name: Symbol,
    variant_data: VariantData<'tcx>,
) -> eaf::ModuleForm<'arena> {
    let base_map = base_struct_map(afb, span, mod_name, variant_data);
    let clause0 = afb.clause(span, [], None, base_map);
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
    let fun_clause = afb.clause(span, [pair_pat, afb.pat_var(span, sym::Acc)], None, update);
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

    let clause = afb.clause(span, [afb.pat_var(span, sym::Fields)], None, reduce_call);
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
