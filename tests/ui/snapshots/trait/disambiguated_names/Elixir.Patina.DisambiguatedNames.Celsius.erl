-file("tests/ui/src/trait/disambiguated_names.pt", 13).

-module('Elixir.Patina.DisambiguatedNames.Celsius').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo>::from'/2,
         'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo>::$patina_witness'/0,
         'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo#1>::from'/2,
         'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo#1>::$patina_witness'/0]).

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

'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo>::from'(_witness_From@0_0,
                                                                            __foo@1_0) ->
    'Elixir.Patina.DisambiguatedNames.Celsius'.

'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo>::$patina_witness'() ->
    {{fun 'Elixir.Patina.DisambiguatedNames.Celsius':'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo>::from'/2},
     {},
     {}}.

'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo#1>::from'(_witness_From@0_0,
                                                                              __foo@1_0) ->
    'Elixir.Patina.DisambiguatedNames.Celsius'.

'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo#1>::$patina_witness'() ->
    {{fun 'Elixir.Patina.DisambiguatedNames.Celsius':'disambiguated_names::From<disambiguated_names::sibling_blocks::Foo#1>::from'/2},
     {},
     {}}.
