-file("tests/ui/src/generics/witness/multi_impl.pt", 3).

-module('Elixir.Patina.MultiImpl').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         from_fahrenheit/1,
         from_kelvin/1,
         from_kelvin_static/1]).

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

to_celsius(_witness@1_0 = {_witness@0_0}, _t@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'MultiImpl::From::from'(_witness@0_0,
                                                           _t@2_0).

from_fahrenheit(_f@0_0) ->
    to_celsius({{'impl MultiImpl::From<MultiImpl::Fahrenheit> for MultiImpl::Celsius',
                 'Elixir.Patina.MultiImpl.Celsius'}},
               _f@0_0).

from_kelvin(_k@0_0) ->
    to_celsius({{'impl MultiImpl::From<MultiImpl::Kelvin> for MultiImpl::Celsius',
                 'Elixir.Patina.MultiImpl.Celsius'}},
               _k@0_0).

from_kelvin_static(_k@0_0) ->
    'Elixir.Patina.MultiImpl.Celsius':'MultiImpl::From::from'({'impl MultiImpl::From<MultiImpl::Kelvin> for MultiImpl::Celsius',
                                                               'Elixir.Patina.MultiImpl.Celsius'},
                                                              _k@0_0).
