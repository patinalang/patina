-file("tests/ui/src/generics/witness/multi_impl.pt", 7).

-module('Elixir.Patina.MultiImpl.Celsius').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'MultiImpl::From::from'/2]).

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

'MultiImpl::From::from'(_witness@0_0 =
                            {'impl MultiImpl::From<MultiImpl::Fahrenheit> for MultiImpl::Celsius',
                             _},
                        _f@1_0) ->
    (_f@1_0 - 32) * 5 div 9;
'MultiImpl::From::from'(_witness@0_0 =
                            {'impl MultiImpl::From<MultiImpl::Kelvin> for MultiImpl::Celsius',
                             _},
                        _k@1_0) ->
    _k@1_0 - 273.
