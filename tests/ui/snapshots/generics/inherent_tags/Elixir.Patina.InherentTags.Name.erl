-file("tests/ui/src/generics/inherent_tags.pt", 26).

-module('Elixir.Patina.InherentTags.Name').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'InherentTags::Show::show'/2,
         'InherentTags::Show::$patina_witness'/0]).

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

'InherentTags::Show::show'(_witness_Show@0_0,
                           _self@1_0) ->
    <<"a name"/utf8>>.

'InherentTags::Show::$patina_witness'() ->
    {{fun 'Elixir.Patina.InherentTags.Name':'InherentTags::Show::show'/2},
     {},
     {}}.
