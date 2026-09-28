-file("tests/ui/src/trait/defaults.pt", 448).

-module('Elixir.Patina.Defaults.P').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Core::PartialEq::eq'/4,
         'Core::PartialEq::ne'/4]).

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

'Core::PartialEq::eq'(_Self@0_0,
                      _Rhs@1_0 = {'Elixir.Patina.Defaults.P', {}}, _self@2_0,
                      _other@3_0) ->
    _self@2_0 =:= _other@3_0.

'Core::PartialEq::ne'(_arg@0_0, _arg@1_0, _arg@2_0,
                      _arg@3_0) ->
    'Elixir.Patina.Core':'Core::PartialEq::ne'(_arg@0_0,
                                               _arg@1_0,
                                               _arg@2_0,
                                               _arg@3_0).
