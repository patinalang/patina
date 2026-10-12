-file("tests/ui/src/fn/diverging__02.pt", 4).

-module('Elixir.Patina.Diverging02').

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

some_fn() -> {}.

main() ->
    __a@0_0 = some_fn(),
    begin
        io:put_chars(begin
                         _element@1_0 =
                             [<<"This function returns and you can see "
                                "this line."/utf8>>],
                         [_element@1_0]
                     end),
        io:nl(),
        {}
    end.
