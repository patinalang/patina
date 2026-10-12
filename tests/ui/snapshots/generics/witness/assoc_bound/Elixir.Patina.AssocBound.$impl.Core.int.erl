-file("tests/ui/src/generics/witness/assoc_bound.pt",
      8).

-module('Elixir.Patina.AssocBound.$impl.Core.Int').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'AssocBound::Show::show'/2,
         'AssocBound::Show::$patina_witness'/0]).

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

'AssocBound::Show::show'(_witness@0_0, _self@1_0) ->
    <<"an int"/utf8>>.

'AssocBound::Show::$patina_witness'() ->
    {{fun 'Elixir.Patina.AssocBound.$impl.Core.Int':'AssocBound::Show::show'/2},
     {},
     {}}.
