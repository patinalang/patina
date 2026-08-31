-file("tests/ui/src/local_calls.pt", 1).

-module('Elixir.Patina.LocalCalls').

-compile([no_auto_import]).

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

get_age(_age@0_0) -> erlang:map_get('0', _age@0_0).

main() ->
    _age@0_0 = 'Elixir.Patina.LocalCalls.Years':new(25),
    _age_value@1_0 = get_age(_age@0_0),
    begin
        io:put_chars([[<<"Age ">>,
                       io_lib:format("~ktp", [_age@0_0]),
                       <<" is ">>,
                       io_lib:format("~ktp", [_age_value@1_0])]]),
        io:nl(),
        {}
    end.
