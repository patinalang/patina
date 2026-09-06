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
            io:put_chars([[<<"page loaded">>]]),
            io:nl(),
            {};
        'Elixir.Patina.Enum01.WebEvent.PageUnload' ->
            io:put_chars([[<<"page unloaded">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.Enum01.WebEvent':'KeyPress'{'0' =
                                                        _c@1_0} ->
            io:put_chars([[<<"pressed '">>,
                           io_lib:format("~ktp", [_c@1_0]),
                           <<"'.">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.Enum01.WebEvent':'Paste'{'0' =
                                                     _s@2_0} ->
            io:put_chars([[<<"pasted \"">>, _s@2_0, <<"\".">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.Enum01.WebEvent':'Click'{x = _x@3_0,
                                                 y = _y@4_0} ->
            begin
                io:put_chars([[<<"clicked at x=">>,
                               io_lib:format("~ktp", [_x@3_0]),
                               <<", y=">>,
                               io_lib:format("~ktp", [_y@4_0]),
                               <<".">>]]),
                io:nl(),
                {}
            end
    end.

main() ->
    _pressed@0_0 =
        #'Elixir.Patina.Enum01.WebEvent':'KeyPress'{'0' = $x},
    _pasted@1_0 =
        #'Elixir.Patina.Enum01.WebEvent':'Paste'{'0' =
                                                     <<"my text">>},
    _click@2_0 = #'Elixir.Patina.Enum01.WebEvent':'Click'{x
                                                              = 20,
                                                          y = 80},
    _load@3_0 = 'Elixir.Patina.Enum01.WebEvent.PageLoad',
    _unload@4_0 =
        'Elixir.Patina.Enum01.WebEvent.PageUnload',
    inspect(_pressed@0_0),
    inspect(_pasted@1_0),
    inspect(_click@2_0),
    inspect(_load@3_0),
    inspect(_unload@4_0).
