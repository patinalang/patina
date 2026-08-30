-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      1).

-module('Elixir.Patina.TraitArgumentPatterns').

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

main() ->
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.UnboundContainer':'TraitArgumentPatterns::Contains::contains'({'Elixir.Patina.TraitArgumentPatterns.UnboundContainer',
                                                                                                                                          {}},
                                                                                                                                         {'Elixir.Patina.Core.int',
                                                                                                                                          {}},
                                                                                                                                         {'Elixir.Patina.Core.bool',
                                                                                                                                          {}},
                                                                                                                                         'Elixir.Patina.TraitArgumentPatterns.UnboundContainer',
                                                                                                                                         1,
                                                                                                                                         false)])]]),
        io:nl()
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.BoundContainer':'TraitArgumentPatterns::Contains::contains'({'Elixir.Patina.TraitArgumentPatterns.BoundContainer',
                                                                                                                                        {}},
                                                                                                                                       {'Elixir.Patina.Core.int',
                                                                                                                                        {}},
                                                                                                                                       {'Elixir.Patina.TraitArgumentPatterns.Marked',
                                                                                                                                        {}},
                                                                                                                                       'Elixir.Patina.TraitArgumentPatterns.BoundContainer',
                                                                                                                                       1,
                                                                                                                                       ('Elixir.Patina.TraitArgumentPatterns.Marked':'__struct__'())#{value
                                                                                                                                                                                                          :=
                                                                                                                                                                                                          2})])]]),
        io:nl()
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.NestedContainer':'TraitArgumentPatterns::Contains::contains'({'Elixir.Patina.TraitArgumentPatterns.NestedContainer',
                                                                                                                                         {}},
                                                                                                                                        {'Elixir.Patina.Core.int',
                                                                                                                                         {}},
                                                                                                                                        {'Elixir.Patina.TraitArgumentPatterns.Wrapped',
                                                                                                                                         {{'Elixir.Patina.Core.bool',
                                                                                                                                           {}}}},
                                                                                                                                        'Elixir.Patina.TraitArgumentPatterns.NestedContainer',
                                                                                                                                        1,
                                                                                                                                        ('Elixir.Patina.TraitArgumentPatterns.Wrapped':'__struct__'())#{value
                                                                                                                                                                                                            :=
                                                                                                                                                                                                            false})])]]),
        io:nl()
    end.
