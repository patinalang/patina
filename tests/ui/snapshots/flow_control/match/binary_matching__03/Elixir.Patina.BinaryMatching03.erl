-file("tests/ui/src/flow_control/match/binary_matchi"
      "ng__03.pt",
      4).

-module('Elixir.Patina.BinaryMatching03').

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
    _bin@0_0 = <<16:8/integer, 5:8/integer, 6:8/integer,
                 7:8/integer>>,
    case _bin@0_0 of
        <<_kind@1_0:8/integer, _len@2_0:8/integer,
          _payload@3_0:_len@2_0/binary, _rest@4_0/binary>> ->
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@5_0 =
                                                     <<"kind = "/utf8>>,
                                                 _element@6_0 =
                                                     io_lib:format("~ktp",
                                                                   [_kind@1_0]),
                                                 _element@7_0 =
                                                     <<", payload = "/utf8>>,
                                                 _element@8_0 =
                                                     io_lib:format("~ktp",
                                                                   [_payload@3_0]),
                                                 _element@9_0 =
                                                     <<", rest = "/utf8>>,
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@4_0]),
                                                 [_element@5_0,
                                                  _element@6_0,
                                                  _element@7_0,
                                                  _element@8_0,
                                                  _element@9_0,
                                                  _element@10_0]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            {};
        <<_kind@12_0:8/integer, _len@13_0:8/integer,
          _payload@14_0:(_len@13_0 div 2)/binary,
          _rest@15_0/binary>> ->
            io:put_chars(begin
                             _element@22_0 = begin
                                                 _element@16_0 =
                                                     <<"kind = "/utf8>>,
                                                 _element@17_0 =
                                                     io_lib:format("~ktp",
                                                                   [_kind@12_0]),
                                                 _element@18_0 =
                                                     <<", payload = "/utf8>>,
                                                 _element@19_0 =
                                                     io_lib:format("~ktp",
                                                                   [_payload@14_0]),
                                                 _element@20_0 =
                                                     <<", rest = "/utf8>>,
                                                 _element@21_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@15_0]),
                                                 [_element@16_0,
                                                  _element@17_0,
                                                  _element@18_0,
                                                  _element@19_0,
                                                  _element@20_0,
                                                  _element@21_0]
                                             end,
                             [_element@22_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@23_0 = [<<"invalid packet"/utf8>>],
                             [_element@23_0]
                         end),
            io:nl(),
            {}
    end,
    _packet@24_0 = <<3:4/integer, 5:4/integer,
                     7:3/integer>>,
    case _packet@24_0 of
        <<_header@25_0:8/integer, _tail@26_0/bitstring>> ->
            io:put_chars(begin
                             _element@31_0 = begin
                                                 _element@27_0 =
                                                     <<"header = "/utf8>>,
                                                 _element@28_0 =
                                                     io_lib:format("~ktp",
                                                                   [_header@25_0]),
                                                 _element@29_0 =
                                                     <<", tail = "/utf8>>,
                                                 _element@30_0 =
                                                     io_lib:format("~ktp",
                                                                   [_tail@26_0]),
                                                 [_element@27_0,
                                                  _element@28_0,
                                                  _element@29_0,
                                                  _element@30_0]
                                             end,
                             [_element@31_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@32_0 = [<<"did not match"/utf8>>],
                             [_element@32_0]
                         end),
            io:nl(),
            {}
    end.
