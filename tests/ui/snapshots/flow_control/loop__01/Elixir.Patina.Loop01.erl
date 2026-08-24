-file("tests/ui/src/flow_control/loop__01.pt", 4).
-module('Elixir.Patina.Loop01').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,'$patina_loop_10_109'/1,main/0]).
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
'$patina_loop_10_109'(_count@0_0) ->
    _ = begin
            _count@0_1 = _count@0_0 + 1,
            case _count@0_1 =:= 3 of
                true ->
                    begin
                        io:put_chars([[<<"three">>]]),
                        io:nl()
                    end,
                    '$patina_marker':continue(_count@0_1);
                false ->
                    {}
            end,
            begin
                io:put_chars([[io_lib:format("~ktp", [_count@0_1])]]),
                io:nl()
            end,
            case _count@0_1 =:= 5 of
                true ->
                    begin
                        io:put_chars([[<<"OK, that's enough">>]]),
                        io:nl()
                    end,
                    '$patina_marker':break({break, 0, {}, _count@0_1});
                false ->
                    {}
            end
        end,
    '$patina_marker':continue(_count@0_1).
main() ->
    _count@0_0 = 0,
    begin
        io:put_chars([[<<"Let's count until infinity!">>]]),
        io:nl()
    end,
    case '$patina_loop_10_109'(_count@0_0) of
        {break, 0, _loop_value@1_0, _count@0_1} ->
            _loop_value@1_0
    end.
