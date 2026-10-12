-file("tests/ui/src/flow_control/match/binary_matchi"
      "ng__04.pt",
      4).

-module('Elixir.Patina.BinaryMatching04').

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
    _text@0_0 = <<"Patina!"/utf8>>,
    _bytes@1_0 = <<80, 78, 71, $!/utf8, "rest"/utf8>>,
    _packet@2_0 = <<80, 84, $!/utf8, "ok"/utf8,
                    5:3/integer>>,
    case _text@0_0 of
        <<"Pa"/utf8, $t/utf8, _rest@3_0/binary>> ->
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@4_0 =
                                                    <<"text rest "/utf8>>,
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  [_rest@3_0]),
                                                [_element@4_0, _element@5_0]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@7_0 = [<<"unknown text"/utf8>>],
                             [_element@7_0]
                         end),
            io:nl(),
            {}
    end,
    case _bytes@1_0 of
        <<80, 78, 71, $!/utf8, _rest@8_0/binary>> ->
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@9_0 =
                                                     <<"png data "/utf8>>,
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@8_0]),
                                                 [_element@9_0, _element@10_0]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@12_0 = [<<"unknown binary"/utf8>>],
                             [_element@12_0]
                         end),
            io:nl(),
            {}
    end,
    case _packet@2_0 of
        <<80, 84, $!/utf8, "ok"/utf8, 5:3/integer,
          _rest@13_0/bitstring>> ->
            io:put_chars(begin
                             _element@16_0 = begin
                                                 _element@14_0 =
                                                     <<"packet tail "/utf8>>,
                                                 _element@15_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@13_0]),
                                                 [_element@14_0, _element@15_0]
                                             end,
                             [_element@16_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@17_0 = [<<"unknown packet"/utf8>>],
                             [_element@17_0]
                         end),
            io:nl(),
            {}
    end.
