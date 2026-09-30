-file("tests/ui/src/generics/witness/nested_assoc_bo"
      "und.pt",
      22).

-module('Elixir.Patina.NestedAssocBound.Root').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'NestedAssocBound::Outer::type::Inner'/1,
         'NestedAssocBound::Outer::inner'/2]).

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

'NestedAssocBound::Outer::type::Inner'(_witness@0_0 =
                                           {'impl NestedAssocBound::Outer for NestedAssocBound::Root',
                                            _}) ->
    {{'impl NestedAssocBound::HasItem for NestedAssocBound::Leaf',
      'Elixir.Patina.NestedAssocBound.Leaf'},
     {'impl NestedAssocBound::Label for NestedAssocBound::Tag',
      'Elixir.Patina.NestedAssocBound.Tag'}}.

'NestedAssocBound::Outer::inner'(_witness@0_0 =
                                     {'impl NestedAssocBound::Outer for NestedAssocBound::Root',
                                      _},
                                 _self@1_0) ->
    'Elixir.Patina.NestedAssocBound.Leaf'.
