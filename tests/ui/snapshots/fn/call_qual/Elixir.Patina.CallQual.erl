-file("tests/ui/src/fn/call_qual.pt", 1).

-module('Elixir.Patina.CallQual').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1, public_main/0, main/0]).

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

priv_main() ->
    'Elixir.Patina.CallQual.StructWithNew':new(42).

public_main() ->
    _ = priv_main(),
    _ = priv_main(),
    priv_main().

main() ->
    _ = public_main(),
    _ = public_main(),
    _ = 'Elixir.Patina.CallQual.Nested':main(),
    _ = priv_main(),
    _s@0_0 = 'Elixir.Patina.CallQual.StructWithNew':new(42),
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [erlang:map_get(value, _s@0_0)])]]),
        io:nl(),
        {}
    end.
