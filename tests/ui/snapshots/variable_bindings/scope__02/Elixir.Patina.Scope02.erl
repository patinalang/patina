-file("tests/ui/src/variable_bindings/scope__02.pt", 4).

-module('Elixir.Patina.Scope02').

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
    _shadowed_binding@0_0 = 1,
    begin
        begin
            io:put_chars([[<<"before being shadowed: "/utf8>>,
                           io_lib:format("~ktp", [_shadowed_binding@0_0])]]),
            io:nl(),
            {}
        end,
        _shadowed_binding@1_0 = <<"abc"/utf8>>,
        begin
            io:put_chars([[<<"shadowed in inner block: "/utf8>>,
                           _shadowed_binding@1_0]]),
            io:nl(),
            {}
        end
    end,
    begin
        io:put_chars([[<<"outside inner block: "/utf8>>,
                       io_lib:format("~ktp", [_shadowed_binding@0_0])]]),
        io:nl(),
        {}
    end,
    _shadowed_binding@2_0 = 2,
    begin
        io:put_chars([[<<"shadowed in outer block: "/utf8>>,
                       io_lib:format("~ktp", [_shadowed_binding@2_0])]]),
        io:nl(),
        {}
    end.
