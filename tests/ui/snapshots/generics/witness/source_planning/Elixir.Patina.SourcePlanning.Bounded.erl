-file("tests/ui/src/generics/witness/source_planning.pt",
      56).

-module('Elixir.Patina.SourcePlanning.Bounded').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'SourcePlanning::Value::value<int>'/2,
         'SourcePlanning::Value::$patina_witness<int>'/0,
         'SourcePlanning::Middle::$patina_witness<int>'/0,
         'SourcePlanning::Deep::$patina_witness<int>'/0,
         'SourcePlanning::Direct::direct'/2,
         'SourcePlanning::Direct::$patina_witness'/1]).

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

'SourcePlanning::Value::value<int>'(_witness_Value@0_0,
                                    _self@1_0) ->
    30.

'SourcePlanning::Value::$patina_witness<int>'() ->
    {{fun 'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Value::value<int>'/2},
     {},
     {}}.

'SourcePlanning::Middle::$patina_witness<int>'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Value::value<int>'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.SourcePlanning':'SourcePlanning::Middle::middle'/2},
     {_witness_Value@0_0},
     {}}.

'SourcePlanning::Deep::$patina_witness<int>'() ->
    _witness_Value@0_0 =
        {{fun 'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Value::value<int>'/2},
         {},
         {}},
    _witness_Middle@1_0 =
        {{fun 'Elixir.Patina.SourcePlanning':'SourcePlanning::Middle::middle'/2},
         {_witness_Value@0_0},
         {}},
    {{fun 'Elixir.Patina.SourcePlanning':'SourcePlanning::Deep::deep'/2},
     {_witness_Middle@1_0},
     {}}.

'SourcePlanning::Direct::direct'(_witness_Direct@0_0,
                                 _self@1_0) ->
    _where_Direct@2_0 = erlang:element(3,
                                       _witness_Direct@0_0),
    _witness_Value@3_0 = erlang:element(1,
                                        _where_Direct@2_0),
    _vtable_Value@4_0 = erlang:element(1,
                                       _witness_Value@3_0),
    _fn_Value_value@5_0 = erlang:element(1,
                                         _vtable_Value@4_0),
    _fn_Value_value@5_0(_witness_Value@3_0, _self@1_0).

'SourcePlanning::Direct::$patina_witness'(_witnesses_where_Direct@0_0) ->
    _witness_Value@1_0 = erlang:element(1,
                                        _witnesses_where_Direct@0_0),
    {{fun 'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Direct::direct'/2},
     {_witness_Value@1_0},
     _witnesses_where_Direct@0_0}.
