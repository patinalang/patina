-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_list__01.pt",
      4).

-module('Elixir.Patina.DestructureList01').

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
    _list@0_0 = [1, -2, 6],
    case _list@0_0 of
        [0, _second@1_0, _third@2_0] ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_second@1_0]),
                                                _element@4_0 =
                                                    io_lib:format("~ktp",
                                                                  [_third@2_0]),
                                                [<<"list[0] = 0, list[1] = "/utf8>>,
                                                 _element@3_0,
                                                 <<", list[2] = "/utf8>>,
                                                 _element@4_0]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        [1, _, _third@6_0] ->
            io:put_chars(begin
                             _element@8_0 = begin
                                                _element@7_0 =
                                                    io_lib:format("~ktp",
                                                                  [_third@6_0]),
                                                [<<"list[0] = 1, list[2] = "/utf8>>,
                                                 _element@7_0,
                                                 <<" and list[1] was ignored"/utf8>>]
                                            end,
                             [_element@8_0]
                         end),
            io:nl(),
            {};
        [-1, _second@9_0 | _] ->
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_second@9_0]),
                                                 [<<"list[0] = -1, list[1] = "/utf8>>,
                                                  _element@10_0,
                                                  <<" and all the other ones were ignored"/utf8>>]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            {};
        [3, _second@13_0 | _tail@12_0 = _] ->
            io:put_chars(begin
                             _element@16_0 = begin
                                                 _element@14_0 =
                                                     io_lib:format("~ktp",
                                                                   [_second@13_0]),
                                                 _element@15_0 =
                                                     io_lib:format("~ktp",
                                                                   [_tail@12_0]),
                                                 [<<"list[0] = 3, list[1] = "/utf8>>,
                                                  _element@14_0,
                                                  <<" and the other elements were "/utf8>>,
                                                  _element@15_0]
                                             end,
                             [_element@16_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"no list pattern matched"/utf8>>]]),
            io:nl(),
            {}
    end.
