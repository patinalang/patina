-file("tests/ui/src/flow_control/match/guard__01.pt",
      4).

-module('Elixir.Patina.Guard01').

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

main() ->
    _temperature@0_0 = 35,
    case _temperature@0_0 of
        _value@2_0 = _t@1_0
            when erlang:is_integer(_value@2_0), _t@1_0 > 30 ->
            io:put_chars([[io_lib:format("~ktp", [_t@1_0]),
                           <<"C is above 30 Celsius">>]]),
            io:nl(),
            {};
        _value@4_0 = _t@3_0
            when erlang:is_integer(_value@4_0) ->
            io:put_chars([[io_lib:format("~ktp", [_t@3_0]),
                           <<"C is equal to or below 30 Celsius">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' =
                                                              _t@5_0}
            when _t@5_0 > 86 ->
            io:put_chars([[io_lib:format("~ktp", [_t@5_0]),
                           <<"F is above 86 Fahrenheit">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' =
                                                              _t@6_0} ->
            io:put_chars([[io_lib:format("~ktp", [_t@6_0]),
                           <<"F is equal to or below 86 Fahrenheit">>]]),
            io:nl(),
            {}
    end.
