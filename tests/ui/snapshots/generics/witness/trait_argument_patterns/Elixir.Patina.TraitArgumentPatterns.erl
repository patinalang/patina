-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      1).

-module('Elixir.Patina.TraitArgumentPatterns').

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
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.UnboundContainer':'TraitArgumentPatterns::Contains<int, _>::contains'('Elixir.Patina.TraitArgumentPatterns.UnboundContainer':'TraitArgumentPatterns::Contains<int, _>::$patina_witness'(),
                                                                                                                                                 'Elixir.Patina.TraitArgumentPatterns.UnboundContainer',
                                                                                                                                                 1,
                                                                                                                                                 false)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.BoundContainer':'TraitArgumentPatterns::Contains<int, _>::contains'('Elixir.Patina.TraitArgumentPatterns.BoundContainer':'TraitArgumentPatterns::Contains<int, _>::$patina_witness'({'Elixir.Patina.TraitArgumentPatterns.Marked':'TraitArgumentPatterns::Marker::$patina_witness'()}),
                                                                                                                                               'Elixir.Patina.TraitArgumentPatterns.BoundContainer',
                                                                                                                                               1,
                                                                                                                                               2)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.TraitArgumentPatterns.NestedContainer':'TraitArgumentPatterns::Contains<int, TraitArgumentPatterns::Wrapped<_>>::contains'('Elixir.Patina.TraitArgumentPatterns.NestedContainer':'TraitArgumentPatterns::Contains<int, TraitArgumentPatterns::Wrapped<_>>::$patina_witness'(),
                                                                                                                                                                                'Elixir.Patina.TraitArgumentPatterns.NestedContainer',
                                                                                                                                                                                1,
                                                                                                                                                                                false)])]]),
        io:nl(),
        {}
    end.
