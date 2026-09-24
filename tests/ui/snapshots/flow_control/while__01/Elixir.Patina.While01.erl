-file("tests/ui/src/flow_control/while__01.pt", 4).

-module('Elixir.Patina.While01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, '$patina_loop_5_143'/1, main/0]).

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

'$patina_loop_5_143'(_n@0_0) ->
    _ = begin
            case _n@0_0 < 101 of
                true ->
                    case _n@0_0 rem 15 =:= 0 of
                        true ->
                            begin
                                io:put_chars([[<<"fizzbuzz"/utf8>>]]),
                                io:nl(),
                                {}
                            end;
                        false ->
                            case _n@0_0 rem 3 =:= 0 of
                                true ->
                                    begin
                                        io:put_chars([[<<"fizz"/utf8>>]]),
                                        io:nl(),
                                        {}
                                    end;
                                false ->
                                    case _n@0_0 rem 5 =:= 0 of
                                        true ->
                                            begin
                                                io:put_chars([[<<"buzz"/utf8>>]]),
                                                io:nl(),
                                                {}
                                            end;
                                        false ->
                                            begin
                                                io:put_chars([[io_lib:format("~ktp",
                                                                             [_n@0_0])]]),
                                                io:nl(),
                                                {}
                                            end
                                    end
                            end
                    end,
                    _n@0_1 = _n@0_0 + 1,
                    {};
                false ->
                    _branch_value@1_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@0_0})
                                        end,
                    _n@0_1 = _n@0_0,
                    _branch_value@1_0
            end
        end,
    '$patina_marker':continue(_n@0_1).

main() ->
    _n@0_0 = 1,
    case '$patina_loop_5_143'(_n@0_0) of
        {break, 0, _loop_value@2_0, _n@0_1} -> _loop_value@2_0
    end.
