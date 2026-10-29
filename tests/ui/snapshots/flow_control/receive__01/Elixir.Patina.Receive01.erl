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
            io:put_chars(begin
                             _element@2_0 = begin
                                                _element@1_0 =
                                                    io_lib:format("~ktp",
                                                                  [_n@0_0]),
                                                [<<"received integer "/utf8>>,
                                                 _element@1_0]
                                            end,
                             [_element@2_0]
                         end),
            io:nl(),
            {};
        ping ->
            io:put_chars([[<<"received ping"/utf8>>]]),
            io:nl(),
            {};
        _a@3_0 when erlang:is_atom(_a@3_0) ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@4_0 =
                                                    io_lib:format("~ktp",
                                                                  [_a@3_0]),
                                                [<<"received atom "/utf8>>,
                                                 _element@4_0]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            {}
        after 'Elixir.Patina.Core.Timeout':from_seconds(5) ->
                  io:put_chars([[<<"timeout (5 seconds with no matching "
                                   "message)"/utf8>>]]),
                  io:nl(),
                  {}
    end,
    receive
        _val@6_0 ->
            io:put_chars(begin
                             _element@8_0 = begin
                                                _element@7_0 =
                                                    io_lib:format("~ktp",
                                                                  [_val@6_0]),
                                                [<<"received any message "/utf8>>,
                                                 _element@7_0]
                                            end,
                             [_element@8_0]
                         end),
            io:nl(),
            {}
    end.
