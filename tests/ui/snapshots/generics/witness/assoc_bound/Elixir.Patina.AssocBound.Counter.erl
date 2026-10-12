-file("tests/ui/src/generics/witness/assoc_bound.pt",
      18).

-module('Elixir.Patina.AssocBound.Counter').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'AssocBound::Iter::$patina_type::Item'/1,
         'AssocBound::Iter::next'/2,
         'AssocBound::Iter::$patina_witness'/0]).

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

'AssocBound::Iter::$patina_type::Item'(_witness@0_0) ->
    {'Elixir.Patina.AssocBound.$impl.Core.Int':'AssocBound::Show::$patina_witness'()}.

'AssocBound::Iter::next'(_witness@0_0, _self@1_0) ->
    _self@1_0.

'AssocBound::Iter::$patina_witness'() ->
    {{fun 'Elixir.Patina.AssocBound.Counter':'AssocBound::Iter::$patina_type::Item'/1,
      fun 'Elixir.Patina.AssocBound.Counter':'AssocBound::Iter::next'/2},
     {},
     {}}.
