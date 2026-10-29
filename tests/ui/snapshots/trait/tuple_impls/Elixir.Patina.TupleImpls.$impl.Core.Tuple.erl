-file("tests/ui/src/trait/tuple_impls.pt", 17).

-module('Elixir.Patina.TupleImpls.$impl.Core.Tuple').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TupleImpls::Show::show<()>'/2,
         'TupleImpls::Show::$patina_witness<()>'/0,
         'TupleImpls::Show::show<(_,)>'/2,
         'TupleImpls::Show::$patina_witness<(_,)>'/1,
         'TupleImpls::Show::show<(_, _)>'/2,
         'TupleImpls::Show::$patina_witness<(_, _)>'/1,
         'TupleImpls::Show::show<(_, _, _)>'/2,
         'TupleImpls::Show::$patina_witness<(_, _, _)>'/1]).

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

'TupleImpls::Show::show<()>'(_witness_Show@0_0,
                             _self@1_0) ->
    <<"()"/utf8>>.

'TupleImpls::Show::$patina_witness<()>'() ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<()>'/2},
     {},
     {}}.

'TupleImpls::Show::show<(_,)>'(_witness_Show@0_0,
                               _self@1_0) ->
    _where_Show@2_0 = erlang:element(3, _witness_Show@0_0),
    _witness_Show@3_0 = erlang:element(1, _where_Show@2_0),
    _vtable_Show@4_0 = erlang:element(1, _witness_Show@3_0),
    _fn_Show_show@5_0 = erlang:element(1, _vtable_Show@4_0),
    {_A@6_0} = _self@1_0,
    <<<<<<"("/utf8,
          (_fn_Show_show@5_0(_witness_Show@3_0,
                             _A@6_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_,)>'(_witnesses_where_Show@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_,)>'/2},
     {},
     _witnesses_where_Show@0_0}.

'TupleImpls::Show::show<(_, _)>'(_witness_Show@0_0,
                                 _self@1_0) ->
    _where_Show@2_0 = erlang:element(3, _witness_Show@0_0),
    _witness_Show@3_0 = erlang:element(2, _where_Show@2_0),
    _witness_Show@4_0 = erlang:element(1, _where_Show@2_0),
    _vtable_Show@5_0 = erlang:element(1, _witness_Show@3_0),
    _fn_Show_show@6_0 = erlang:element(1, _vtable_Show@5_0),
    _vtable_Show@7_0 = erlang:element(1, _witness_Show@4_0),
    _fn_Show_show@8_0 = erlang:element(1, _vtable_Show@7_0),
    {_A@9_0, _B@10_0} = _self@1_0,
    <<<<<<<<<<"("/utf8,
              (_fn_Show_show@8_0(_witness_Show@4_0,
                                 _A@9_0))/binary>>/binary,
            ","/utf8>>/binary,
          (_fn_Show_show@6_0(_witness_Show@3_0,
                             _B@10_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_, _)>'(_witnesses_where_Show@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_, _)>'/2},
     {},
     _witnesses_where_Show@0_0}.

'TupleImpls::Show::show<(_, _, _)>'(_witness_Show@0_0,
                                    _self@1_0) ->
    _where_Show@2_0 = erlang:element(3, _witness_Show@0_0),
    _witness_Show@3_0 = erlang:element(3, _where_Show@2_0),
    _witness_Show@4_0 = erlang:element(2, _where_Show@2_0),
    _witness_Show@5_0 = erlang:element(1, _where_Show@2_0),
    _vtable_Show@6_0 = erlang:element(1, _witness_Show@3_0),
    _fn_Show_show@7_0 = erlang:element(1, _vtable_Show@6_0),
    _vtable_Show@8_0 = erlang:element(1, _witness_Show@4_0),
    _fn_Show_show@9_0 = erlang:element(1, _vtable_Show@8_0),
    _vtable_Show@10_0 = erlang:element(1,
                                       _witness_Show@5_0),
    _fn_Show_show@11_0 = erlang:element(1,
                                        _vtable_Show@10_0),
    {_A@12_0, _B@13_0, _C@14_0} = _self@1_0,
    <<<<<<<<<<<<<<"("/utf8,
                  (_fn_Show_show@11_0(_witness_Show@5_0,
                                      _A@12_0))/binary>>/binary,
                ","/utf8>>/binary,
              (_fn_Show_show@9_0(_witness_Show@4_0,
                                 _B@13_0))/binary>>/binary,
            ","/utf8>>/binary,
          (_fn_Show_show@7_0(_witness_Show@3_0,
                             _C@14_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_, _, _)>'(_witnesses_where_Show@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_, _, _)>'/2},
     {},
     _witnesses_where_Show@0_0}.
