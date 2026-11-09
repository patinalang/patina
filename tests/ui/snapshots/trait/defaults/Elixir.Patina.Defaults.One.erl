-file("tests/ui/src/trait/defaults.pt", 377).

-module('Elixir.Patina.Defaults.One').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Numbered::$patina_const::C'/1,
         'defaults::Numbered::$patina_witness'/0]).

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

'defaults::Numbered::$patina_const::C'(_witness_Numbered@0_0) ->
    1.

'defaults::Numbered::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.One':'defaults::Numbered::$patina_const::C'/1,
      fun 'Elixir.Patina.Defaults':'defaults::Numbered::$patina_const::D'/1,
      fun 'Elixir.Patina.Defaults':'defaults::Numbered::consts'/2},
     {},
     {}}.
