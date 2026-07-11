-file("tests/ui/src/custom_types/enum__01.pt", 8).
-module('Elixir.Patina.Enum01').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,main/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
inspect(_var_0) ->
    case _var_0 of
        'Elixir.Patina.Enum01.WebEvent.PageLoad' ->
            io:put_chars([[<<"page loaded">>]]),
            io:nl();
        'Elixir.Patina.Enum01.WebEvent.PageUnload' ->
            io:put_chars([[<<"page unloaded">>]]),
            io:nl();
        #'Elixir.Patina.Enum01.WebEvent':'KeyPress'{'0' = _var_1} ->
            io:put_chars([[<<"pressed '">>,
                           io_lib:format("~tp", [_var_1]),
                           <<"'.">>]]),
            io:nl();
        #'Elixir.Patina.Enum01.WebEvent':'Paste'{'0' = _var_2} ->
            io:put_chars([[<<"pasted \"">>, _var_2, <<"\".">>]]),
            io:nl();
        #'Elixir.Patina.Enum01.WebEvent':'Click'{x = _var_3, y = _var_4} ->
            begin
                io:put_chars([[<<"clicked at x=">>,
                               io_lib:format("~tp", [_var_3]),
                               <<", y=">>,
                               io_lib:format("~tp", [_var_4]),
                               <<".">>]]),
                io:nl()
            end
    end.
main() ->
    _var_0 = #'Elixir.Patina.Enum01.WebEvent':'KeyPress'{'0' = $x},
    _var_1 =
        #'Elixir.Patina.Enum01.WebEvent':'Paste'{'0' = <<"my text">>},
    _var_2 = #'Elixir.Patina.Enum01.WebEvent':'Click'{x = 20, y = 80},
    _var_3 = 'Elixir.Patina.Enum01.WebEvent.PageLoad',
    _var_4 = 'Elixir.Patina.Enum01.WebEvent.PageUnload',
    inspect(_var_0),
    inspect(_var_1),
    inspect(_var_2),
    inspect(_var_3),
    inspect(_var_4).
