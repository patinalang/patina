-file("tests/ui/src/fn/call_qual.pt", 1).

-module('Elixir.Patina.CallQual').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

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
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_s@0_0]),
                                        [_element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.
