-file("tests/ui/src/local_calls.pt", 1).
-module('Elixir.Patina.LocalCalls').
-compile([no_auto_import]).
-export(['__info__'/1,main/0]).
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
get_age(_var_0) ->
    map_get('0', _var_0).
main() ->
    _var_0 = 'Elixir.Patina.LocalCalls.Years':new(25),
    _var_1 = get_age(_var_0),
    begin
        io:put_chars([[<<"Age ">>,
                       io_lib:format("~tp", [_var_0]),
                       <<" is ">>,
                       io_lib:format("~tp", [_var_1])]]),
        io:nl()
    end.
