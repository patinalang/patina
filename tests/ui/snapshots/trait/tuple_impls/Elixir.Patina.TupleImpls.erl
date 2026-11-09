-file("tests/ui/src/trait/tuple_impls.pt", 1).

-module('Elixir.Patina.TupleImpls').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         show_unit/0,
         show_pair/2,
         show_generic_all/2]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) -> nil;
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

show_generic(_witnesses_where_show_generic@0_0,
             _t@1_0) ->
    _witness_Show@2_0 = erlang:element(1,
                                       _witnesses_where_show_generic@0_0),
    _vtable_Show@3_0 = erlang:element(1, _witness_Show@2_0),
    _fn_Show_show@4_0 = erlang:element(1, _vtable_Show@3_0),
    _fn_Show_show@4_0(_witness_Show@2_0, _t@1_0).

show_unit() ->
    _witness_Show@0_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<()>'(),
    'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::show<()>'(_witness_Show@0_0,
                                                                              {}).

show_pair(_n@0_0, _s@1_0) ->
    _witness_Show@2_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Int':'tuple_impls::Show::$patina_witness'(),
    _witness_Show@3_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Str':'tuple_impls::Show::$patina_witness'(),
    _witnesses_where_Show@4_0 = {_witness_Show@2_0,
                                 _witness_Show@3_0},
    _witness_Show@5_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<(_, _)>'(_witnesses_where_Show@4_0),
    'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::show<(_, _)>'(_witness_Show@5_0,
                                                                                  {_n@0_0,
                                                                                   _s@1_0}).

show_generic_all(_n@0_0, _s@1_0) ->
    _witness_Show@2_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Int':'tuple_impls::Show::$patina_witness'(),
    _witnesses_where_Show@3_0 = {_witness_Show@2_0},
    _witness_Show@4_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<(_,)>'(_witnesses_where_Show@3_0),
    _witnesses_where_show_generic@5_0 = {_witness_Show@4_0},
    _witness_Show@6_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Str':'tuple_impls::Show::$patina_witness'(),
    _witnesses_where_Show@7_0 = {_witness_Show@2_0,
                                 _witness_Show@6_0},
    _witness_Show@8_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<(_, _)>'(_witnesses_where_Show@7_0),
    _witnesses_where_show_generic@9_0 = {_witness_Show@8_0},
    _witness_Show@10_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<()>'(),
    _witnesses_where_Show@11_0 = {_witness_Show@2_0,
                                  _witness_Show@10_0},
    _witness_Show@12_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<(_, _)>'(_witnesses_where_Show@11_0),
    _witnesses_where_Show@13_0 = {_witness_Show@2_0,
                                  _witness_Show@6_0,
                                  _witness_Show@12_0},
    _witness_Show@14_0 =
        'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'tuple_impls::Show::$patina_witness<(_, _, _)>'(_witnesses_where_Show@13_0),
    _witnesses_where_show_generic@15_0 =
        {_witness_Show@14_0},
    <<<<(show_generic(_witnesses_where_show_generic@5_0,
                      {_n@0_0}))/binary,
        (show_generic(_witnesses_where_show_generic@9_0,
                      {_n@0_0, _s@1_0}))/binary>>/binary,
      (show_generic(_witnesses_where_show_generic@15_0,
                    begin
                        _element@16_0 = {_n@0_0, {}},
                        {_n@0_0, _s@1_0, _element@16_0}
                    end))/binary>>.
