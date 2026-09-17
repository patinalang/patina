-file("tests/ui/src/flow_control/try_catch_after__01.pt",
      4).

-module('Elixir.Patina.TryCatchAfter01').

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

open_resource() -> ok.

main() ->
    try open_resource(), {} catch
        error:badarg ->
            io:put_chars([[<<"badarg!">>]]),
            io:nl(),
            {};
        error:(#{'__struct__' := 'Elixir.ArithmeticError'}) ->
            io:put_chars([[<<"Elixir ArithmeticError">>]]),
            io:nl(),
            {};
        error:(_reason@0_0 = #{'__struct__' :=
                                   'Elixir.ArgumentError'}) ->
            io:put_chars([[<<"Elixir ArgumentError ">>,
                           io_lib:format("~ktp", [_reason@0_0])]]),
            io:nl(),
            {};
        error:_reason@2_0:_stacktrace@1_0 ->
            io:put_chars([[<<"other error: ">>,
                           io_lib:format("~ktp", [_reason@2_0]),
                           <<" from ">>,
                           io_lib:format("~ktp", [_stacktrace@1_0])]]),
            io:nl(),
            {};
        _value@3_0 ->
            io:put_chars([[<<"thrown: ">>,
                           io_lib:format("~ktp", [_value@3_0])]]),
            io:nl(),
            {};
        exit:_reason@4_0 ->
            io:put_chars([[<<"exit: ">>,
                           io_lib:format("~ktp", [_reason@4_0])]]),
            io:nl(),
            {}
    after
        begin
            io:put_chars([[<<"cleaning up">>]]), io:nl(), {}
        end
    end.
