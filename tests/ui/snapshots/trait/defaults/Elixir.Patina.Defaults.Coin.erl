-file("tests/ui/src/trait/defaults.pt", 128).

-module('Elixir.Patina.Defaults.Coin').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Value::value'/2,
         'defaults::Value::$patina_witness'/0,
         'defaults::Combine<defaults::Note>::$patina_witness'/0]).

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

'defaults::Value::value'(_witness_Value@0_0,
                         _self@1_0) ->
    _self@1_0.

'defaults::Value::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Coin':'defaults::Value::value'/2},
     {},
     {}}.

'defaults::Combine<defaults::Note>::$patina_witness'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.Defaults.Coin':'defaults::Value::value'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.Defaults':'defaults::Combine::uses_rhs_witness'/4},
     {_witness_Value@0_0},
     {}}.
