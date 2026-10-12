-file("tests/ui/src/trait/tuple_impls.pt", 11).

-module('Elixir.Patina.TupleImpls.$impl.Core.Str').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TupleImpls::Show::show'/2,
         'TupleImpls::Show::$patina_witness'/0]).

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

'TupleImpls::Show::show'(_witness_self@0_0 = {_,
                                              {},
                                              {}},
                         _self@1_0) ->
    _self@1_0.

'TupleImpls::Show::$patina_witness'() ->
    {{fun 'Elixir.Patina.TupleImpls.$impl.Core.Str':'TupleImpls::Show::show'/2},
     {},
     {}}.
