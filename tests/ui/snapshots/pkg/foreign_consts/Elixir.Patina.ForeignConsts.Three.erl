-file("tests/ui/src/pkg/foreign_consts.pt", 25).

-module('Elixir.Patina.ForeignConsts.Three').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'consts::HasValue::$patina_const::VALUE'/1,
         'consts::HasValue::$patina_witness'/0]).

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

'consts::HasValue::$patina_const::VALUE'(_witness_HasValue@0_0) ->
    3.

'consts::HasValue::$patina_witness'() ->
    {{fun 'Elixir.Patina.ForeignConsts.Three':'consts::HasValue::$patina_const::VALUE'/1},
     {},
     {}}.
