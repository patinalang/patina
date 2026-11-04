-file("tests/ui/src/trait/native_eq.pt", 19).

-module('Elixir.Patina.NativeEq.HandWrittenNativeEq').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['HandWrittenNativeEq']).

-export(['__info__'/1,
         '$patina_constructor__HandWrittenNativeEq'/2,
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

-record #'HandWrittenNativeEq'{x, y}.

'$patina_constructor__HandWrittenNativeEq'(_x@0_0,
                                           _y@1_0) ->
    #'HandWrittenNativeEq'{x = _x@0_0, y = _y@1_0}.

'Core::PartialEq::eq'(_witness_PartialEq@0_0, _self@1_0,
                      _other@2_0) ->
    _self@1_0 =:= _other@2_0.

'Core::PartialEq::$patina_witness'() ->
    {{fun 'Elixir.Patina.NativeEq.HandWrittenNativeEq':'Core::PartialEq::eq'/3,
      fun 'Elixir.Patina.Core':'Core::PartialEq::ne'/3},
     {},
     {}}.
