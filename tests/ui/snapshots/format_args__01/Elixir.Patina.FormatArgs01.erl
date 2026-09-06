-file("tests/ui/src/format_args__01.pt", 4).

-module('Elixir.Patina.FormatArgs01').

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
    _name@0_0 = <<"world">>,
    _n@1_0 = 42,
    begin
        io:put_chars([[<<"hello, ">>, _name@0_0, <<"!">>]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[_name@0_0,
                       <<" = ">>,
                       io_lib:format("~ktp", [_n@1_0])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[_name@0_0,
                       <<" and ">>,
                       io_lib:format("~ktp", [_n@1_0])]]),
        io:nl(),
        {}
    end,
    begin io:put_chars([<<"no ">>]), {} end,
    begin io:put_chars([<<"newline">>]), {} end,
    begin io:nl(), {} end.
