-file("tests/ui/src/flow_control/match/binary_matchi"
      "ng__06.pt",
      4).

-module('Elixir.Patina.BinaryMatching06').

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
    _name@0_0 = <<"Patina"/utf8>>,
    _version@1_0 = 3,
    _greeting@2_0 = <<"Hello, "/utf8, _name@0_0/binary,
                      $!/utf8>>,
    _header@3_0 = <<80, 84, _version@1_0:16/integer,
                    1.5:32/float, $é/utf16>>,
    _flags@4_0 = <<1:1/integer, 0:1/integer,
                   (_version@1_0 + 1):3/integer>>,
    begin
        io:put_chars([[io_lib:format("~ktp", [_greeting@2_0]),
                       <<" "/utf8>>,
                       io_lib:format("~ktp", [_header@3_0]),
                       <<" "/utf8>>,
                       io_lib:format("~ktp", [_flags@4_0])]]),
        io:nl(),
        {}
    end.
