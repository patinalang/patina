-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      3).

-module('Elixir.Patina.TraitArgumentPatterns.NestedContainer').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::contains'/4,
         'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::$patina_witness'/0]).

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

'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::contains'(_witness_Contains@0_0,
                                                                                        _self@1_0,
                                                                                        _a@2_0,
                                                                                        _wrapped@3_0) ->
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [_self@1_0]),
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [_a@2_0]),
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_wrapped@3_0]),
                                        [<<"NestedContainer::contains("/utf8>>,
                                         _element@4_0,
                                         <<", "/utf8>>,
                                         _element@5_0,
                                         <<", "/utf8>>,
                                         _element@6_0,
                                         <<")"/utf8>>]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    true.

'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::$patina_witness'() ->
    {{fun 'Elixir.Patina.TraitArgumentPatterns.NestedContainer':'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::contains'/4},
     {},
     {}}.
