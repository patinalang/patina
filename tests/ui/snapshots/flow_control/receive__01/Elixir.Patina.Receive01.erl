-file("tests/ui/src/flow_control/receive__01.pt", 4).

-module('Elixir.Patina.Receive01').

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
    receive
        _n@0_0 when erlang:is_integer(_n@0_0) ->
            io:put_chars([[<<"received integer ">>,
                           io_lib:format("~ktp", [_n@0_0])]]),
            io:nl(),
            {};
        ping ->
            io:put_chars([[<<"received ping">>]]),
            io:nl(),
            {};
        _a@1_0 when erlang:is_atom(_a@1_0) ->
            io:put_chars([[<<"received atom ">>,
                           io_lib:format("~ktp", [_a@1_0])]]),
            io:nl(),
            {}
        after 'Elixir.Patina.Core.Timeout':from_seconds(5) ->
                  begin
                      io:put_chars([[<<"timeout (5 seconds with no matching "
                                       "message)">>]]),
                      io:nl(),
                      {}
                  end
    end,
    receive
        _val@2_0 ->
            io:put_chars([[<<"received any message ">>,
                           io_lib:format("~ktp", [_val@2_0])]]),
            io:nl(),
            {}
    end.
