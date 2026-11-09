-file("tests/ui/src/trait/defaults.pt", 194).

-module('Elixir.Patina.Defaults.N').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::inner::Doubler::base'/2,
         'defaults::inner::Doubler::$patina_witness'/0]).

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

'defaults::inner::Doubler::base'(_witness_Doubler@0_0,
                                 _self@1_0) ->
    _self@1_0.

'defaults::inner::Doubler::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.N':'defaults::inner::Doubler::base'/2,
      fun 'Elixir.Patina.Defaults.Inner':'defaults::inner::Doubler::doubled'/2},
     {},
     {}}.
