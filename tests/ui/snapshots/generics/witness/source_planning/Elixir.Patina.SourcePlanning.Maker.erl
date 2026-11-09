-file("tests/ui/src/generics/witness/source_planning.pt",
      79).

-module('Elixir.Patina.SourcePlanning.Maker').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'source_planning::Factory::$patina_type::Item'/1,
         'source_planning::Factory::item'/2,
         'source_planning::Factory::$patina_witness'/0]).

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

'source_planning::Factory::$patina_type::Item'(_witness_Factory@0_0) ->
    _witness_Value@1_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Value::value'/2},
         {},
         {}},
    _witness_Middle@2_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'source_planning::Middle::middle'/2},
         {_witness_Value@1_0},
         {}},
    _witness_Deep@3_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'source_planning::Deep::deep'/2},
         {_witness_Middle@2_0},
         {}},
    {_witness_Deep@3_0, _witness_Value@1_0}.

'source_planning::Factory::item'(_witness_Factory@0_0,
                                 _self@1_0) ->
    0.

'source_planning::Factory::$patina_witness'() ->
    {{fun 'Elixir.Patina.SourcePlanning.Maker':'source_planning::Factory::$patina_type::Item'/1,
      fun 'Elixir.Patina.SourcePlanning.Maker':'source_planning::Factory::item'/2},
     {},
     {}}.
