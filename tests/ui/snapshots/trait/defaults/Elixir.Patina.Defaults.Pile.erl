-file("tests/ui/src/trait/defaults.pt", 132).

-module('Elixir.Patina.Defaults.Pile').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Value::value'/2,
         'Defaults::Combine::uses_rhs_witness'/4]).

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

'Defaults::Value::value'(_witness@0_0 =
                             {'impl Defaults::Value for Defaults::Pile<_>',
                              _,
                              _witness@1_0},
                         _self@2_0) ->
    (erlang:element(2,
                    _witness@1_0)):'Defaults::Value::value'(_witness@1_0,
                                                            _self@2_0)
        + 1.

'Defaults::Combine::uses_rhs_witness'(_witness@0_0 =
                                          {'impl Defaults::Combine for Defaults::Pile<_>',
                                           _,
                                           _,
                                           _},
                                      _arg@1_0, _arg@2_0, _arg@3_0) ->
    'Elixir.Patina.Defaults':'Defaults::Combine::uses_rhs_witness'(_witness@0_0,
                                                                   _arg@1_0,
                                                                   _arg@2_0,
                                                                   _arg@3_0).
