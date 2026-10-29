-file("tests/ui/src/generics/witness/nested_assoc_bo"
      "und.pt",
      4).

-module('Elixir.Patina.NestedAssocBound').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, root_label/0]).

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

nested_label(_witnesses_where_nested_label@0_0,
             _t@1_0) ->
    _witness_Outer@2_0 = erlang:element(1,
                                        _witnesses_where_nested_label@0_0),
    _vtable_Outer@3_0 = erlang:element(1,
                                       _witness_Outer@2_0),
    _fn_Outer_inner@4_0 = erlang:element(2,
                                         _vtable_Outer@3_0),
    _type_Outer_Inner@5_0 = erlang:element(1,
                                           _vtable_Outer@3_0),
    _where_Outer@6_0 =
        _type_Outer_Inner@5_0(_witness_Outer@2_0),
    _witness_Label@7_0 = erlang:element(2,
                                        _where_Outer@6_0),
    _witness_HasItem@8_0 = erlang:element(1,
                                          _where_Outer@6_0),
    _vtable_Label@9_0 = erlang:element(1,
                                       _witness_Label@7_0),
    _fn_Label_label@10_0 = erlang:element(1,
                                          _vtable_Label@9_0),
    _vtable_HasItem@11_0 = erlang:element(1,
                                          _witness_HasItem@8_0),
    _fn_HasItem_item@12_0 = erlang:element(2,
                                           _vtable_HasItem@11_0),
    _fn_Label_label@10_0(_witness_Label@7_0,
                         _fn_HasItem_item@12_0(_witness_HasItem@8_0,
                                               _fn_Outer_inner@4_0(_witness_Outer@2_0,
                                                                   _t@1_0))).

root_label() ->
    _witness_Outer@0_0 =
        'Elixir.Patina.NestedAssocBound.Root':'NestedAssocBound::Outer::$patina_witness'(),
    _witnesses_where_nested_label@1_0 =
        {_witness_Outer@0_0},
    nested_label(_witnesses_where_nested_label@1_0,
                 'Elixir.Patina.NestedAssocBound.Root').
