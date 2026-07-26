-file("tests/ui/src/fn/diverging__02.pt", 4).
-module('Elixir.Patina.Diverging02').
-compile([no_auto_import]).
-export_record([]).
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
some_fn() ->
    {}.
main() ->
    __a@0_0 = some_fn(),
    begin
        io:put_chars([[<<"This function returns and you can see this li"
                         "ne.">>]]),
        io:nl()
    end.
