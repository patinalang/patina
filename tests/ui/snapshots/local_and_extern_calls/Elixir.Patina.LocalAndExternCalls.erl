-file("tests/ui/src/local_and_extern_calls.pt", 1).
-module('Elixir.Patina.LocalAndExternCalls').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,public_main/0,main/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
priv_main() ->
    'Elixir.Patina.LocalAndExternCalls.StructWithNew':new(42).
public_main() ->
    _ = priv_main(),
    _ = priv_main(),
    priv_main().
main() ->
    _ = public_main(),
    _ = public_main(),
    _ = 'Elixir.Patina.LocalAndExternCalls.Nested':main(),
    _ = priv_main(),
    _var_0 = 'Elixir.Patina.LocalAndExternCalls.StructWithNew':new(42),
    begin
        io:put_chars([[io_lib:format("~tp", [map_get(value, _var_0)])]]),
        io:nl()
    end.
