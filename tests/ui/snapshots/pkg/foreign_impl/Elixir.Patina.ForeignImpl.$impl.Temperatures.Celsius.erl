-file("tests/ui/src/pkg/foreign_impl.pt", 24).

-module('Elixir.Patina.ForeignImpl.$impl.Temperatures.Celsius').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Temperatures::From<ForeignImpl::LocalDegrees>::from'/2,
         'Temperatures::From<ForeignImpl::LocalDegrees>::$patina_witness'/0]).

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

'Temperatures::From<ForeignImpl::LocalDegrees>::from'(_witness_From@0_0,
                                                      _d@1_0) ->
    _0@2_0 = _d@1_0,
    _0@2_0.

'Temperatures::From<ForeignImpl::LocalDegrees>::$patina_witness'() ->
    {{fun 'Elixir.Patina.ForeignImpl.$impl.Temperatures.Celsius':'Temperatures::From<ForeignImpl::LocalDegrees>::from'/2},
     {},
     {}}.
