-file("tests/ui/src/types/alias__01.pt", 5).
-module('Elixir.Patina.Alias01').
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
main() ->
    _var_0 = 5,
    _var_1 = 2,
    begin
        io:put_chars([[io_lib:format("~tp", [_var_0]),
                       <<" nanoseconds + ">>,
                       io_lib:format("~tp", [_var_1]),
                       <<" inches = ">>,
                       io_lib:format("~tp", [_var_0 + _var_1]),
                       <<" unit?">>]]),
        io:nl()
    end.
