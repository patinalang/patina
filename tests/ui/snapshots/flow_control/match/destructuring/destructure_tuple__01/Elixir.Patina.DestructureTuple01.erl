-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_tuple__01.pt",
      4).

-module('Elixir.Patina.DestructureTuple01').

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
    _triple@0_0 = {0, -2, 3},
    begin
        io:put_chars([[<<"Tell me about ">>,
                       io_lib:format("~ktp", [_triple@0_0])]]),
        io:nl(),
        {}
    end,
    case _triple@0_0 of
        {0, _y@1_0, _z@2_0} ->
            io:put_chars([[<<"First is `0`, `y` is ">>,
                           io_lib:format("~ktp", [_y@1_0]),
                           <<", and `z` is ">>,
                           io_lib:format("~ktp", [_z@2_0])]]),
            io:nl(),
            {};
        {1, _, _} ->
            io:put_chars([[<<"First is `1` and the rest doesn't matter">>]]),
            io:nl(),
            {};
        {_, _, 2} ->
            io:put_chars([[<<"last is `2` and the rest doesn't matter">>]]),
            io:nl(),
            {};
        {3, _, 4} ->
            io:put_chars([[<<"First is `3`, last is `4`, and the rest "
                             "doesn't matter">>]]),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"It doesn't matter what they are">>]]),
            io:nl(),
            {}
    end.
