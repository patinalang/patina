-file("tests/ui/src/flow_control/while__01.pt", 4).

-module('Elixir.Patina.While01').

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

'main$patina_loop_1'(_n@0_0) ->
    _ = begin
            case _n@0_0 < 101 of
                true ->
                    case _n@0_0 rem 15 =:= 0 of
                        true ->
                            begin
                                io:put_chars(begin
                                                 _element@1_0 =
                                                     [<<"fizzbuzz"/utf8>>],
                                                 [_element@1_0]
                                             end),
                                io:nl(),
                                {}
                            end;
                        _ ->
                            case _n@0_0 rem 3 =:= 0 of
                                true ->
                                    begin
                                        io:put_chars(begin
                                                         _element@2_0 =
                                                             [<<"fizz"/utf8>>],
                                                         [_element@2_0]
                                                     end),
                                        io:nl(),
                                        {}
                                    end;
                                _ ->
                                    case _n@0_0 rem 5 =:= 0 of
                                        true ->
                                            begin
                                                io:put_chars(begin
                                                                 _element@3_0 =
                                                                     [<<"buzz"/utf8>>],
                                                                 [_element@3_0]
                                                             end),
                                                io:nl(),
                                                {}
                                            end;
                                        _ ->
                                            begin
                                                io:put_chars(begin
                                                                 _element@5_0 =
                                                                     begin
                                                                         _element@4_0 =
                                                                             io_lib:format("~ktp",
                                                                                           [_n@0_0]),
                                                                         [_element@4_0]
                                                                     end,
                                                                 [_element@5_0]
                                                             end),
                                                io:nl(),
                                                {}
                                            end
                                    end
                            end
                    end,
                    _n@0_1 = _n@0_0 + 1,
                    {};
                _ ->
                    _branch_value@6_0 = begin
                                            begin
                                                '$patina_marker':break({break,
                                                                        0,
                                                                        {},
                                                                        _n@0_0}),
                                                []
                                            end
                                        end,
                    _n@0_1 = _n@0_0,
                    _branch_value@6_0
            end
        end,
    begin '$patina_marker':continue(_n@0_1), [] end.

main() ->
    _n@0_0 = 1,
    case 'main$patina_loop_1'(_n@0_0) of
        {break, 0, _loop_value@7_0, _n@0_1} -> _loop_value@7_0
    end.
