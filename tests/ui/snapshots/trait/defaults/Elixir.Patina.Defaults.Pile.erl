-file("tests/ui/src/trait/defaults.pt", 132).

-module('Elixir.Patina.Defaults.Pile').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Value::value'/2,
         'defaults::Value::$patina_witness'/1,
         'defaults::Combine::$patina_witness'/1]).

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
    _where_Value@2_0 = erlang:element(3,
                                      _witness_Value@0_0),
    _witness_Value@3_0 = erlang:element(1,
                                        _where_Value@2_0),
    _vtable_Value@4_0 = erlang:element(1,
                                       _witness_Value@3_0),
    _fn_Value_value@5_0 = erlang:element(1,
                                         _vtable_Value@4_0),
    _fn_Value_value@5_0(_witness_Value@3_0, _self@1_0) + 1.

'defaults::Value::$patina_witness'(_witnesses_where_Value@0_0) ->
    {{fun 'Elixir.Patina.Defaults.Pile':'defaults::Value::value'/2},
     {},
     _witnesses_where_Value@0_0}.

'defaults::Combine::$patina_witness'(_witnesses_where_Combine@0_0) ->
    _witness_Value@1_0 = erlang:element(1,
                                        _witnesses_where_Combine@0_0),
    _witness_Value@2_0 =
        {{fun 'Elixir.Patina.Defaults.Pile':'defaults::Value::value'/2},
         {},
         {_witness_Value@1_0}},
    {{fun 'Elixir.Patina.Defaults':'defaults::Combine::uses_rhs_witness'/4},
     {_witness_Value@2_0},
     _witnesses_where_Combine@0_0}.
