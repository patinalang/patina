-file("tests/ui/src/trait/disambiguated_names.pt", 13).

-module('Elixir.Patina.DisambiguatedNames.Celsius').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'DisambiguatedNames::From::from'/2]).

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

'DisambiguatedNames::From::from'(_witness@0_0 =
                                     {'impl DisambiguatedNames::From<DisambiguatedNames::sibling_blocks::Foo> for DisambiguatedNames::Celsius',
                                      _},
                                 __foo@1_0) ->
    'Elixir.Patina.DisambiguatedNames.Celsius';
'DisambiguatedNames::From::from'(_witness@0_0 =
                                     {'impl DisambiguatedNames::From<DisambiguatedNames::sibling_blocks::Foo#1> for DisambiguatedNames::Celsius',
                                      _},
                                 __foo@1_0) ->
    'Elixir.Patina.DisambiguatedNames.Celsius'.
