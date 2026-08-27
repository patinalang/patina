-file("tests/ui/src/flow_control/match__01.pt", 4).
-module('Elixir.Patina.Match01').
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
    _number@0_0 = 13,
    begin
        io:put_chars([[<<"Tell me about ">>,
                       io_lib:format("~ktp", [_number@0_0])]]),
        io:nl()
    end,
    case _number@0_0 of
        1 ->
            io:put_chars([[<<"One!">>]]),
            io:nl();
        2 ->
            io:put_chars([[<<"This is a prime">>]]),
            io:nl();
        3 ->
            io:put_chars([[<<"This is a prime">>]]),
            io:nl();
        5 ->
            io:put_chars([[<<"This is a prime">>]]),
            io:nl();
        7 ->
            io:put_chars([[<<"This is a prime">>]]),
            io:nl();
        11 ->
            io:put_chars([[<<"This is a prime">>]]),
            io:nl();
        _range@1_0 when _range@1_0 >= 13, _range@1_0 =< 19 ->
            io:put_chars([[<<"A teen">>]]),
            io:nl();
        _ ->
            io:put_chars([[<<"Ain't special">>]]),
            io:nl()
    end,
    _boolean@2_0 = true,
    _binary@3_0 =
        case _boolean@2_0 of
            false ->
                0;
            true ->
                1
        end,
    begin
        io:put_chars([[io_lib:format("~ktp", [_boolean@2_0]),
                       <<" -> ">>,
                       io_lib:format("~ktp", [_binary@3_0])]]),
        io:nl()
    end,
    _expected_value@4_0 = 42,
    case 123 of
        _expected_value@4_0 ->
            io:put_chars([[<<"found expected value">>]]),
            io:nl();
        _value@5_0 ->
            io:put_chars([[<<"found unexpected value ">>,
                           io_lib:format("~ktp", [_value@5_0])]]),
            io:nl()
    end.
