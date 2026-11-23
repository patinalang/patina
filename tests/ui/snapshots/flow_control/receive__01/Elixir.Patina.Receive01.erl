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
        _n@2_0 when erlang:is_integer(_n@2_0) ->
            io:put_chars(begin
                             _element@4_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_n@2_0]),
                                                [<<"received integer "/utf8>>,
                                                 _element@3_0]
                                            end,
                             [_element@4_0]
                         end),
            io:nl(),
            {};
        ping ->
            io:put_chars([[<<"received ping"/utf8>>]]),
            io:nl(),
            {};
        _a@5_0 when erlang:is_atom(_a@5_0) ->
            io:put_chars(begin
                             _element@7_0 = begin
                                                _element@6_0 =
                                                    io_lib:format("~ktp",
                                                                  [_a@5_0]),
                                                [<<"received atom "/utf8>>,
                                                 _element@6_0]
                                            end,
                             [_element@7_0]
                         end),
            io:nl(),
            {}
        after begin
                  _s@0_0 = 5, _0@1_0 = _s@0_0 * 1000, _0@1_0
              end ->
                  io:put_chars([[<<"timeout (5 seconds with no matching "
                                   "message)"/utf8>>]]),
                  io:nl(),
                  {}
    end,
    receive
        _val@8_0 ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_val@8_0]),
                                                 [<<"received any message "/utf8>>,
                                                  _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {}
    end.
