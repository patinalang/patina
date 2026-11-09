-file("tests/ui/src/generics/witness/assoc_bound.pt",
      36).

-module('Elixir.Patina.AssocBound.Pair').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'assoc_bound::Show::show'/2,
         'assoc_bound::Show::$patina_witness'/1]).

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

'assoc_bound::Show::show'(_witness_Show@0_0,
                          _self@1_0) ->
    _where_Show@2_0 = erlang:element(3, _witness_Show@0_0),
    _witness_Show@3_0 = erlang:element(1, _where_Show@2_0),
    _vtable_Show@4_0 = erlang:element(1, _witness_Show@3_0),
    _fn_Show_show@5_0 = erlang:element(1, _vtable_Show@4_0),
    _fn_Show_show@5_0(_witness_Show@3_0, _self@1_0).

'assoc_bound::Show::$patina_witness'(_witnesses_where_Show@0_0) ->
    {{fun 'Elixir.Patina.AssocBound.Pair':'assoc_bound::Show::show'/2},
     {},
     _witnesses_where_Show@0_0}.
