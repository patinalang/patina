-file("tests/ui/src/trait/native_eq.pt", 1).

-module('Elixir.Patina.NativeEq.EqByParity').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Core::PartialEq::eq'/3,
         'Core::PartialEq::$patina_witness'/0]).

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

'Core::PartialEq::eq'(_witness_PartialEq@0_0, _self@1_0,
                      _other@2_0) ->
    _a@3_0 = _self@1_0,
    _b@4_0 = _other@2_0,
    _a@3_0 rem 2 =:= _b@4_0 rem 2.

'Core::PartialEq::$patina_witness'() ->
    {{fun 'Elixir.Patina.NativeEq.EqByParity':'Core::PartialEq::eq'/3,
      fun 'Elixir.Patina.Core':'Core::PartialEq::ne'/3},
     {},
     {}}.
