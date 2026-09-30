-file("tests/ui/src/generics/witness/nested_assoc_bo"
      "und.pt",
      20).

-module('Elixir.Patina.NestedAssocBound.Tag').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'NestedAssocBound::Label::label'/2]).

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

'NestedAssocBound::Label::label'(_witness@0_0 =
                                     {'impl NestedAssocBound::Label for NestedAssocBound::Tag',
                                      _},
                                 _self@1_0) ->
    <<"tag"/utf8>>.
