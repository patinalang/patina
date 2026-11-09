-file("tests/ui/src/trait/defaults.pt", 218).

-module('Elixir.Patina.Defaults.Person').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Named::name'/2,
         'defaults::Named::$patina_witness'/0,
         'defaults::Greeter::$patina_witness'/0]).

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

'defaults::Named::name'(_witness_Named@0_0,
                        _self@1_0) ->
    _self@1_0.

'defaults::Named::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Person':'defaults::Named::name'/2},
     {},
     {}}.

'defaults::Greeter::$patina_witness'() ->
    _witness_Named@0_0 =
        {{fun 'Elixir.Patina.Defaults.Person':'defaults::Named::name'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.Defaults':'defaults::Greeter::greet'/2},
     {_witness_Named@0_0},
     {}}.
