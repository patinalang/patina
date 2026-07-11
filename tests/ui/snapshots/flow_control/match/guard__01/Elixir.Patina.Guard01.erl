-file("tests/ui/src/flow_control/match/guard__01.pt", 4).
-module('Elixir.Patina.Guard01').
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
main() ->
    _var_0 = #'Elixir.Patina.Guard01.Temperature':'Celsius'{'0' = 35},
    case _var_0 of
        #'Elixir.Patina.Guard01.Temperature':'Celsius'{'0' = _var_1}
            when _var_1 > 30 ->
            io:put_chars([[io_lib:format("~tp", [_var_1]),
                           <<"C is above 30 Celsius">>]]),
            io:nl();
        #'Elixir.Patina.Guard01.Temperature':'Celsius'{'0' = _var_2} ->
            io:put_chars([[io_lib:format("~tp", [_var_2]),
                           <<"C is equal to or below 30 Celsius">>]]),
            io:nl();
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' = _var_3}
            when _var_3 > 86 ->
            io:put_chars([[io_lib:format("~tp", [_var_3]),
                           <<"F is above 86 Fahrenheit">>]]),
            io:nl();
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' = _var_4} ->
            io:put_chars([[io_lib:format("~tp", [_var_4]),
                           <<"F is equal to or below 86 Fahrenheit">>]]),
            io:nl()
    end.
