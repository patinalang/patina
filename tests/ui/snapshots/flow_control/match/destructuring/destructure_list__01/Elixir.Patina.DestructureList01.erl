-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_list__01.pt",
      4).

-module('Elixir.Patina.DestructureList01').

-compile([no_auto_import]).

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
    _list@0_0 = [1, -2, 6],
    case _list@0_0 of
        [0, _second@1_0, _third@2_0] ->
            io:put_chars([[<<"list[0] = 0, list[1] = ">>,
                           io_lib:format("~ktp", [_second@1_0]),
                           <<", list[2] = ">>,
                           io_lib:format("~ktp", [_third@2_0])]]),
            io:nl();
        [1, _, _third@3_0] ->
            io:put_chars([[<<"list[0] = 1, list[2] = ">>,
                           io_lib:format("~ktp", [_third@3_0]),
                           <<" and list[1] was ignored">>]]),
            io:nl();
        [-1, _second@4_0 | _] ->
            io:put_chars([[<<"list[0] = -1, list[1] = ">>,
                           io_lib:format("~ktp", [_second@4_0]),
                           <<" and all the other ones were ignored">>]]),
            io:nl();
        [3, _second@6_0 | _tail@5_0 = _] ->
            io:put_chars([[<<"list[0] = 3, list[1] = ">>,
                           io_lib:format("~ktp", [_second@6_0]),
                           <<" and the other elements were ">>,
                           io_lib:format("~ktp", [_tail@5_0])]]),
            io:nl();
        _ ->
            io:put_chars([[<<"no list pattern matched">>]]),
            io:nl()
    end.
