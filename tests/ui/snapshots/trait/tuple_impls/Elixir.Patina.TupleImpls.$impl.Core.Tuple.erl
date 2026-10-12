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

'TupleImpls::Show::show<()>'(_witness@0_0, _self@1_0) ->
    <<"()"/utf8>>.

'TupleImpls::Show::$patina_witness<()>'() ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<()>'/2},
     {},
     {}}.

'TupleImpls::Show::show<(_,)>'(_witness@0_0,
                               _self@1_0) ->
    {_A@2_0} = _self@1_0,
    <<<<<<"("/utf8,
          begin
              _witness@3_0 = erlang:element(1,
                                            erlang:element(3, _witness@0_0)),
              (erlang:element(1,
                              erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                                _A@2_0)
          end/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_,)>'(_witness@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_,)>'/2},
     {},
     _witness@0_0}.

'TupleImpls::Show::show<(_, _)>'(_witness@0_0,
                                 _self@1_0) ->
    {_A@2_0, _B@3_0} = _self@1_0,
    <<<<<<<<<<"("/utf8,
              begin
                  _witness@4_0 = erlang:element(1,
                                                erlang:element(3,
                                                               _witness@0_0)),
                  (erlang:element(1,
                                  erlang:element(1,
                                                 _witness@4_0)))(_witness@4_0,
                                                                 _A@2_0)
              end/binary>>/binary,
            ","/utf8>>/binary,
          begin
              _witness@5_0 = erlang:element(2,
                                            erlang:element(3, _witness@0_0)),
              (erlang:element(1,
                              erlang:element(1, _witness@5_0)))(_witness@5_0,
                                                                _B@3_0)
          end/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_, _)>'(_witness@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_, _)>'/2},
     {},
     _witness@0_0}.

'TupleImpls::Show::show<(_, _, _)>'(_witness@0_0,
                                    _self@1_0) ->
    {_A@2_0, _B@3_0, _C@4_0} = _self@1_0,
    <<<<<<<<<<<<<<"("/utf8,
                  begin
                      _witness@5_0 = erlang:element(1,
                                                    erlang:element(3,
                                                                   _witness@0_0)),
                      (erlang:element(1,
                                      erlang:element(1,
                                                     _witness@5_0)))(_witness@5_0,
                                                                     _A@2_0)
                  end/binary>>/binary,
                ","/utf8>>/binary,
              begin
                  _witness@6_0 = erlang:element(2,
                                                erlang:element(3,
                                                               _witness@0_0)),
                  (erlang:element(1,
                                  erlang:element(1,
                                                 _witness@6_0)))(_witness@6_0,
                                                                 _B@3_0)
              end/binary>>/binary,
            ","/utf8>>/binary,
          begin
              _witness@7_0 = erlang:element(3,
                                            erlang:element(3, _witness@0_0)),
              (erlang:element(1,
                              erlang:element(1, _witness@7_0)))(_witness@7_0,
                                                                _C@4_0)
          end/binary>>/binary,
        ","/utf8>>/binary,
      ")"/utf8>>.

'TupleImpls::Show::$patina_witness<(_, _, _)>'(_witness@0_0) ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_, _, _)>'/2},
     {},
     _witness@0_0}.
