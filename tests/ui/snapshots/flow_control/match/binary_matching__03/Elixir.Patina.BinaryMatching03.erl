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
            io:put_chars([[<<"kind = "/utf8>>,
                           io_lib:format("~ktp", [_kind@1_0]),
                           <<", payload = "/utf8>>,
                           io_lib:format("~ktp", [_payload@3_0]),
                           <<", rest = "/utf8>>,
                           io_lib:format("~ktp", [_rest@4_0])]]),
            io:nl(),
            {};
        <<_kind@5_0:8/integer, _len@6_0:8/integer,
          _payload@7_0:(_len@6_0 div 2)/binary,
          _rest@8_0/binary>> ->
            io:put_chars([[<<"kind = "/utf8>>,
                           io_lib:format("~ktp", [_kind@5_0]),
                           <<", payload = "/utf8>>,
                           io_lib:format("~ktp", [_payload@7_0]),
                           <<", rest = "/utf8>>,
                           io_lib:format("~ktp", [_rest@8_0])]]),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"invalid packet"/utf8>>]]),
            io:nl(),
            {}
    end,
    _packet@9_0 = <<3:4/integer, 5:4/integer, 7:3/integer>>,
    case _packet@9_0 of
        <<_header@10_0:8/integer, _tail@11_0/bitstring>> ->
            io:put_chars([[<<"header = "/utf8>>,
                           io_lib:format("~ktp", [_header@10_0]),
                           <<", tail = "/utf8>>,
                           io_lib:format("~ktp", [_tail@11_0])]]),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"did not match"/utf8>>]]),
            io:nl(),
            {}
    end.
