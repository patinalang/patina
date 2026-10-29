-file("tests/ui/src/generics/witness/global_bound.pt",
      1).

-module('Elixir.Patina.GlobalBound').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, base/0, global/0]).

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

call(_witnesses_where_call@0_0, _t@1_0) ->
    _witness_Derived@2_0 = erlang:element(1,
                                          _witnesses_where_call@0_0),
    _supertraits_Derived@3_0 = erlang:element(2,
                                              _witness_Derived@2_0),
    _witness_Base@4_0 = erlang:element(1,
                                       _supertraits_Derived@3_0),
    _vtable_Base@5_0 = erlang:element(1, _witness_Base@4_0),
    _fn_Base_base@6_0 = erlang:element(1, _vtable_Base@5_0),
    _fn_Base_base@6_0(_witness_Base@4_0, _t@1_0).

only_global_bound() ->
    _witness_Derived@0_0 =
        'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::$patina_witness'(),
    'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::derived'(_witness_Derived@0_0,
                                                                  'Elixir.Patina.GlobalBound.S').

base() ->
    _witness_Derived@0_0 =
        'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::$patina_witness'(),
    _witnesses_where_call@1_0 = {_witness_Derived@0_0},
    call(_witnesses_where_call@1_0,
         'Elixir.Patina.GlobalBound.S').

global() -> only_global_bound().
