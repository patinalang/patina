-file("tests/ui/src/flow_control/loop__01.pt", 4).

-module('Elixir.Patina.Loop01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'main$patina_loop_1'/1, main/0]).

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

'main$patina_loop_1'(_count@0_0) ->
    _ = begin
            _count@0_1 = _count@0_0 + 1,
            case _count@0_1 =:= 3 of
                true ->
                    begin
                        io:put_chars(begin
                                         _element@2_0 = [<<"three"/utf8>>],
                                         [_element@2_0]
                                     end),
                        io:nl(),
                        {}
                    end,
                    '$patina_marker':continue(_count@0_1);
                _ -> {}
            end,
            begin
                io:put_chars(begin
                                 _element@4_0 = begin
                                                    _element@3_0 =
                                                        io_lib:format("~ktp",
                                                                      [_count@0_1]),
                                                    [_element@3_0]
                                                end,
                                 [_element@4_0]
                             end),
                io:nl(),
                {}
            end,
            case _count@0_1 =:= 5 of
                true ->
                    begin
                        io:put_chars(begin
                                         _element@5_0 =
                                             [<<"OK, that's enough"/utf8>>],
                                         [_element@5_0]
                                     end),
                        io:nl(),
                        {}
                    end,
                    '$patina_marker':break({break, 0, {}, _count@0_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_count@0_1).

main() ->
    _count@0_0 = 0,
    begin
        io:put_chars(begin
                         _element@1_0 =
                             [<<"Let's count until infinity!"/utf8>>],
                         [_element@1_0]
                     end),
        io:nl(),
        {}
    end,
    case 'main$patina_loop_1'(_count@0_0) of
        {break, 0, _loop_value@6_0, _count@0_1} ->
            _loop_value@6_0
    end.
