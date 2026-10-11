-file("tests/ui/src/trait/tuple_impls.pt", 17).

-module('Elixir.Patina.TupleImpls.$impl.Core.Tuple').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'TupleImpls::Show::show'/2]).

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

'TupleImpls::Show::show'(_witness@0_0 =
                             {'impl TupleImpls::Show for ()', _},
                         _self@1_0) ->
    <<"()"/utf8>>;
'TupleImpls::Show::show'(_witness@0_0 =
                             {'impl TupleImpls::Show for (_,)',
                              _,
                              _witness@1_0},
                         _self@2_0) ->
    {_A@3_0} = _self@2_0,
    <<<<<<"("/utf8,
          ((erlang:element(2,
                           _witness@1_0)):'TupleImpls::Show::show'(_witness@1_0,
                                                                   _A@3_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>;
'TupleImpls::Show::show'(_witness@0_0 =
                             {'impl TupleImpls::Show for (_, _)',
                              _,
                              _witness@1_0,
                              _witness@2_0},
                         _self@3_0) ->
    {_A@4_0, _B@5_0} = _self@3_0,
    <<<<<<<<<<"("/utf8,
              ((erlang:element(2,
                               _witness@1_0)):'TupleImpls::Show::show'(_witness@1_0,
                                                                       _A@4_0))/binary>>/binary,
            ","/utf8>>/binary,
          ((erlang:element(2,
                           _witness@2_0)):'TupleImpls::Show::show'(_witness@2_0,
                                                                   _B@5_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>;
'TupleImpls::Show::show'(_witness@0_0 =
                             {'impl TupleImpls::Show for (_, _, _)',
                              _,
                              _witness@1_0,
                              _witness@2_0,
                              _witness@3_0},
                         _self@4_0) ->
    {_A@5_0, _B@6_0, _C@7_0} = _self@4_0,
    <<<<<<<<<<<<<<"("/utf8,
                  ((erlang:element(2,
                                   _witness@1_0)):'TupleImpls::Show::show'(_witness@1_0,
                                                                           _A@5_0))/binary>>/binary,
                ","/utf8>>/binary,
              ((erlang:element(2,
                               _witness@2_0)):'TupleImpls::Show::show'(_witness@2_0,
                                                                       _B@6_0))/binary>>/binary,
            ","/utf8>>/binary,
          ((erlang:element(2,
                           _witness@3_0)):'TupleImpls::Show::show'(_witness@3_0,
                                                                   _C@7_0))/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.
