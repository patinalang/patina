-file("tests/ui/src/generics/witness/global_bound.pt",
      9).

-module('Elixir.Patina.GlobalBound.S').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'GlobalBound::Derived::derived'/2,
         'GlobalBound::Derived::$patina_witness'/0,
         'GlobalBound::Base::base'/2,
         'GlobalBound::Base::$patina_witness'/0]).

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

'GlobalBound::Derived::derived'(_witness_Derived@0_0,
                                _self@1_0) ->
    1.

'GlobalBound::Derived::$patina_witness'() ->
    _witness_Base@0_0 =
        {{fun 'Elixir.Patina.GlobalBound.S':'GlobalBound::Base::base'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::derived'/2},
     {_witness_Base@0_0},
     {}}.

'GlobalBound::Base::base'(_witness_Base@0_0,
                          _self@1_0) ->
    _witness_Derived@2_0 =
        'GlobalBound::Derived::$patina_witness'(),
    'GlobalBound::Derived::derived'(_witness_Derived@2_0,
                                    _self@1_0)
        + 1.

'GlobalBound::Base::$patina_witness'() ->
    {{fun 'Elixir.Patina.GlobalBound.S':'GlobalBound::Base::base'/2},
     {},
     {}}.
