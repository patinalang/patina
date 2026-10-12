-file("tests/ui/src/generics/witness/nested_assoc_bo"
      "und.pt",
      22).

-module('Elixir.Patina.NestedAssocBound.Root').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'NestedAssocBound::Outer::$patina_type::Inner'/1,
         'NestedAssocBound::Outer::inner'/2,
         'NestedAssocBound::Outer::$patina_witness'/0]).

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

'NestedAssocBound::Outer::$patina_type::Inner'(_witness@0_0) ->
    {'Elixir.Patina.NestedAssocBound.Leaf':'NestedAssocBound::HasItem::$patina_witness'(),
     'Elixir.Patina.NestedAssocBound.Tag':'NestedAssocBound::Label::$patina_witness'()}.

'NestedAssocBound::Outer::inner'(_witness@0_0,
                                 _self@1_0) ->
    'Elixir.Patina.NestedAssocBound.Leaf'.

'NestedAssocBound::Outer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedAssocBound.Root':'NestedAssocBound::Outer::$patina_type::Inner'/1,
      fun 'Elixir.Patina.NestedAssocBound.Root':'NestedAssocBound::Outer::inner'/2},
     {},
     {}}.
