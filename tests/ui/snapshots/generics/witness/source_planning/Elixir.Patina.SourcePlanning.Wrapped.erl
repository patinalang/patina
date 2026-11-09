-file("tests/ui/src/generics/witness/source_planning.pt",
      19).

-module('Elixir.Patina.SourcePlanning.Wrapped').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'source_planning::Value::value'/2,
         'source_planning::Value::$patina_witness'/0,
         'source_planning::Direct::direct'/2,
         'source_planning::Direct::$patina_witness'/0,
         'source_planning::Middle::$patina_witness<int>'/0,
         'source_planning::Deep::$patina_witness<int>'/0]).

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

'source_planning::Value::value'(_witness_Value@0_0,
                                _self@1_0) ->
    10.

'source_planning::Value::$patina_witness'() ->
    {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Value::value'/2},
     {},
     {}}.

'source_planning::Direct::direct'(_witness_Direct@0_0,
                                  _self@1_0) ->
    20.

'source_planning::Direct::$patina_witness'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Value::value'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Direct::direct'/2},
     {_witness_Value@0_0},
     {}}.

'source_planning::Middle::$patina_witness<int>'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Value::value'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.SourcePlanning':'source_planning::Middle::middle'/2},
     {_witness_Value@0_0},
     {}}.

'source_planning::Deep::$patina_witness<int>'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Wrapped':'source_planning::Value::value'/2},
         {},
         {}},
    _witness_Middle@1_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'source_planning::Middle::middle'/2},
         {_witness_Value@0_0},
         {}},
    {{fun 'Elixir.Patina.SourcePlanning':'source_planning::Deep::deep'/2},
     {_witness_Middle@1_0},
     {}}.
