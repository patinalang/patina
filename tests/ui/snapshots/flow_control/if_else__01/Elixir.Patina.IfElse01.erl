-file("tests/ui/src/flow_control/if_else__01.pt", 4).
-module('Elixir.Patina.IfElse01').
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
    _var_0 = 5,
    case _var_0 < 0 of
        true ->
            begin
                io:put_chars([io_lib:format("~tp", [_var_0]),
                              <<" is negative">>])
            end;
        false ->
            case _var_0 > 0 of
                true ->
                    begin
                        io:put_chars([io_lib:format("~tp", [_var_0]),
                                      <<" is positive">>])
                    end;
                false ->
                    begin
                        io:put_chars([io_lib:format("~tp", [_var_0]),
                                      <<" is zero">>])
                    end
            end
    end,
    _var_1 =
        case
            _var_0 < 10
            andalso
            _var_0 > -10
        of
            true ->
                begin
                    io:put_chars([[<<", and is a small number, increase"
                                     " ten-fold">>]]),
                    io:nl()
                end,
                10 * _var_0;
            false ->
                begin
                    io:put_chars([[<<", and is a big number, halve the "
                                     "number">>]]),
                    io:nl()
                end,
                _var_0 div 2
        end,
    begin
        io:put_chars([[io_lib:format("~tp", [_var_0]),
                       <<" -> ">>,
                       io_lib:format("~tp", [_var_1])]]),
        io:nl()
    end.
