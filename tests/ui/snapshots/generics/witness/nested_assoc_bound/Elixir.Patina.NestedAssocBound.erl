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

nested_label({_witness@0_0}, _t@1_0) ->
    begin
        _witness@3_0 = erlang:element(2,
                                      (erlang:element(2,
                                                      _witness@0_0)):'NestedAssocBound::Outer::type::Inner'(_witness@0_0)),
        (erlang:element(2,
                        _witness@3_0)):'NestedAssocBound::Label::label'(_witness@3_0,
                                                                        begin
                                                                            _witness@2_0 =
                                                                                erlang:element(1,
                                                                                               (erlang:element(2,
                                                                                                               _witness@0_0)):'NestedAssocBound::Outer::type::Inner'(_witness@0_0)),
                                                                            (erlang:element(2,
                                                                                            _witness@2_0)):'NestedAssocBound::HasItem::item'(_witness@2_0,
                                                                                                                                             (erlang:element(2,
                                                                                                                                                             _witness@0_0)):'NestedAssocBound::Outer::inner'(_witness@0_0,
                                                                                                                                                                                                             _t@1_0))
                                                                        end)
    end.

root_label() ->
    nested_label({{'impl NestedAssocBound::Outer for NestedAssocBound::Root',
                   'Elixir.Patina.NestedAssocBound.Root'}},
                 'Elixir.Patina.NestedAssocBound.Root').
