-file("tests/ui/src/generics/witness/implied_assoc_b"
      "ound.pt",
      4).

-module('Elixir.Patina.ImpliedAssocBound').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         labelled/2,
         labelled_with_items/2,
         crate_labels/0]).

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

labelled(_witnesses_where_labelled@0_0, _t@1_0) ->
    _witness_Label@2_0 = erlang:element(2,
                                        _witnesses_where_labelled@0_0),
    _witness_HasItem@3_0 = erlang:element(1,
                                          _witnesses_where_labelled@0_0),
    _vtable_Label@4_0 = erlang:element(1,
                                       _witness_Label@2_0),
    _fn_Label_label@5_0 = erlang:element(1,
                                         _vtable_Label@4_0),
    _vtable_HasItem@6_0 = erlang:element(1,
                                         _witness_HasItem@3_0),
    _fn_HasItem_item@7_0 = erlang:element(2,
                                          _vtable_HasItem@6_0),
    _fn_Label_label@5_0(_witness_Label@2_0,
                        _fn_HasItem_item@7_0(_witness_HasItem@3_0, _t@1_0)).

labelled_with_items(_witnesses_where_labelled_with_items@0_0,
                    _t@1_0) ->
    _witness_LabelledWithItems@2_0 = erlang:element(1,
                                                    _witnesses_where_labelled_with_items@0_0),
    _supertraits_LabelledWithItems@3_0 = erlang:element(2,
                                                        _witness_LabelledWithItems@2_0),
    _witness_Label@4_0 = erlang:element(2,
                                        _supertraits_LabelledWithItems@3_0),
    _witness_HasItem@5_0 = erlang:element(1,
                                          _supertraits_LabelledWithItems@3_0),
    _vtable_Label@6_0 = erlang:element(1,
                                       _witness_Label@4_0),
    _fn_Label_label@7_0 = erlang:element(1,
                                         _vtable_Label@6_0),
    _vtable_HasItem@8_0 = erlang:element(1,
                                         _witness_HasItem@5_0),
    _fn_HasItem_item@9_0 = erlang:element(2,
                                          _vtable_HasItem@8_0),
    _fn_Label_label@7_0(_witness_Label@4_0,
                        _fn_HasItem_item@9_0(_witness_HasItem@5_0, _t@1_0)).

crate_labels() ->
    _witness_LabelledWithItems@0_0 =
        'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::LabelledWithItems::$patina_witness'(),
    _supertraits_LabelledWithItems@1_0 = erlang:element(2,
                                                        _witness_LabelledWithItems@0_0),
    _witness_Label@2_0 = erlang:element(2,
                                        _supertraits_LabelledWithItems@1_0),
    _witness_HasItem@3_0 = erlang:element(1,
                                          _supertraits_LabelledWithItems@1_0),
    _witnesses_where_labelled@4_0 = {_witness_HasItem@3_0,
                                     _witness_Label@2_0},
    _witnesses_where_labelled_with_items@5_0 =
        {_witness_LabelledWithItems@0_0},
    _element@6_0 = labelled(_witnesses_where_labelled@4_0,
                            'Elixir.Patina.ImpliedAssocBound.Crate'),
    _element@7_0 =
        labelled_with_items(_witnesses_where_labelled_with_items@5_0,
                            'Elixir.Patina.ImpliedAssocBound.Crate'),
    {_element@6_0, _element@7_0}.
