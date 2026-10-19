-file("tests/ui/src/generics/witness/source_planning.pt",
      79).

-module('Elixir.Patina.SourcePlanning.Maker').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'SourcePlanning::Factory::$patina_type::Item'/1,
         'SourcePlanning::Factory::item'/2,
         'SourcePlanning::Factory::$patina_witness'/0]).

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

'SourcePlanning::Factory::$patina_type::Item'(_witness_Factory@0_0) ->
    _witness_Value@1_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Value::value'/2},
         {},
         {}},
    _witness_Middle@2_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'SourcePlanning::Middle::middle'/2},
         {_witness_Value@1_0},
         {}},
    _witness_Deep@3_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'SourcePlanning::Deep::deep'/2},
         {_witness_Middle@2_0},
         {}},
    {_witness_Deep@3_0, _witness_Value@1_0}.

'SourcePlanning::Factory::item'(_witness_Factory@0_0,
                                _self@1_0) ->
    0.

'SourcePlanning::Factory::$patina_witness'() ->
    {{fun 'Elixir.Patina.SourcePlanning.Maker':'SourcePlanning::Factory::$patina_type::Item'/1,
      fun 'Elixir.Patina.SourcePlanning.Maker':'SourcePlanning::Factory::item'/2},
     {},
     {}}.
