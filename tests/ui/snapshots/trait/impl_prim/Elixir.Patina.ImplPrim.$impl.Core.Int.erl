-file("tests/ui/src/trait/impl_prim.pt", 7).

-module('Elixir.Patina.ImplPrim.$impl.Core.Int').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'ImplPrim::MyTrait::f'/2,
         'ImplPrim::MyTrait::$patina_witness'/0]).

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

'ImplPrim::MyTrait::f'(_witness@0_0, _self@1_0) -> 1.

'ImplPrim::MyTrait::$patina_witness'() ->
    {{fun 'Elixir.Patina.ImplPrim.$impl.Core.Int':'ImplPrim::MyTrait::f'/2},
     {},
     {}}.
