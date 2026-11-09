-file("tests/ui/src/pkg/foreign_inline.pt", 3).

-module('Elixir.Patina.ForeignInline.LocalImpl').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'inlines::Trait::method'/2,
         'inlines::Trait::$patina_witness'/0]).

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

'inlines::Trait::method'(_witness_Trait@0_0,
                         _self@1_0) ->
    _self@1_0.

'inlines::Trait::$patina_witness'() ->
    {{fun 'Elixir.Patina.ForeignInline.LocalImpl':'inlines::Trait::method'/2},
     {},
     {}}.
