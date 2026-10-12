-file("tests/ui/src/generics/witness/unbounded.pt", 1).

-module('Elixir.Patina.Unbounded').

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
    _p@0_0 =
        'Elixir.Patina.Unbounded.Pair':'$patina_constructor__Pair'(1,
                                                                   2),
    begin
        io:put_chars(begin
                         _element@2_0 = begin
                                            _element@1_0 = io_lib:format("~ktp",
                                                                         ['Elixir.Patina.Unbounded.Pair':len(_p@0_0)]),
                                            [_element@1_0]
                                        end,
                         [_element@2_0]
                     end),
        io:nl(),
        {}
    end.
