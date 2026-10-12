-file("tests/ui/src/custom_types/enum__01.pt", 8).

-module('Elixir.Patina.Enum01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

inspect(_event@0_0) ->
    case _event@0_0 of
        'Elixir.Patina.Enum01.WebEvent.PageLoad' ->
            io:put_chars(begin
                             _element@1_0 = [<<"page loaded"/utf8>>],
                             [_element@1_0]
                         end),
            io:nl(),
            {};
        'Elixir.Patina.Enum01.WebEvent.PageUnload' ->
            io:put_chars(begin
                             _element@2_0 = [<<"page unloaded"/utf8>>],
                             [_element@2_0]
                         end),
            io:nl(),
            {};
        _value@4_0 = _c@3_0
            when erlang:is_integer(_value@4_0) ->
            io:put_chars(begin
                             _element@8_0 = begin
                                                _element@5_0 =
                                                    <<"pressed '"/utf8>>,
                                                _element@6_0 =
                                                    io_lib:format("~ktp",
                                                                  [_c@3_0]),
                                                _element@7_0 = <<"'."/utf8>>,
                                                [_element@5_0,
                                                 _element@6_0,
                                                 _element@7_0]
                                            end,
                             [_element@8_0]
                         end),
            io:nl(),
            {};
        _value@10_0 = _s@9_0
            when erlang:is_binary(_value@10_0) ->
            io:put_chars(begin
                             _element@14_0 = begin
                                                 _element@11_0 =
                                                     <<"pasted \""/utf8>>,
                                                 _element@12_0 = _s@9_0,
                                                 _element@13_0 = <<"\"."/utf8>>,
                                                 [_element@11_0,
                                                  _element@12_0,
                                                  _element@13_0]
                                             end,
                             [_element@14_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.Enum01.WebEvent':'Click'{x = _x@15_0,
                                                 y = _y@16_0} ->
            begin
                io:put_chars(begin
                                 _element@22_0 = begin
                                                     _element@17_0 =
                                                         <<"clicked at x="/utf8>>,
                                                     _element@18_0 =
                                                         io_lib:format("~ktp",
                                                                       [_x@15_0]),
                                                     _element@19_0 =
                                                         <<", y="/utf8>>,
                                                     _element@20_0 =
                                                         io_lib:format("~ktp",
                                                                       [_y@16_0]),
                                                     _element@21_0 =
                                                         <<"."/utf8>>,
                                                     [_element@17_0,
                                                      _element@18_0,
                                                      _element@19_0,
                                                      _element@20_0,
                                                      _element@21_0]
                                                 end,
                                 [_element@22_0]
                             end),
                io:nl(),
                {}
            end
    end.

main() ->
    _pressed@0_0 = $x,
    _pasted@1_0 = <<"my text"/utf8>>,
    _click@2_0 =
        'Elixir.Patina.Enum01.WebEvent':'$patina_constructor__Click'(20,
                                                                     80),
    _load@3_0 = 'Elixir.Patina.Enum01.WebEvent.PageLoad',
    _unload@4_0 =
        'Elixir.Patina.Enum01.WebEvent.PageUnload',
    inspect(_pressed@0_0),
    inspect(_pasted@1_0),
    inspect(_click@2_0),
    inspect(_load@3_0),
    inspect(_unload@4_0).
