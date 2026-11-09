-file("tests/ui/src/generics/witness/assoc_bound.pt",
      18).

-module('Elixir.Patina.AssocBound.Counter').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'assoc_bound::Iter::$patina_type::Item'/1,
         'assoc_bound::Iter::next'/2,
         'assoc_bound::Iter::$patina_witness'/0]).

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

'assoc_bound::Iter::$patina_type::Item'(_witness_Iter@0_0) ->
    _witness_Show@1_0 =
        {{fun 'Elixir.Patina.AssocBound.$impl.Core.Int':'assoc_bound::Show::show'/2},
         {},
         {}},
    {_witness_Show@1_0}.

'assoc_bound::Iter::next'(_witness_Iter@0_0,
                          _self@1_0) ->
    _self@1_0.

'assoc_bound::Iter::$patina_witness'() ->
    {{fun 'Elixir.Patina.AssocBound.Counter':'assoc_bound::Iter::$patina_type::Item'/1,
      fun 'Elixir.Patina.AssocBound.Counter':'assoc_bound::Iter::next'/2},
     {},
     {}}.
