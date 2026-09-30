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

labelled({_witness@0_0, _witness@1_0}, _t@2_0) ->
    (erlang:element(2,
                    _witness@1_0)):'ImpliedAssocBound::Label::label'(_witness@1_0,
                                                                     (erlang:element(2,
                                                                                     _witness@0_0)):'ImpliedAssocBound::HasItem::item'(_witness@0_0,
                                                                                                                                       _t@2_0)).

labelled_with_items({_witness@0_0}, _t@1_0) ->
    begin
        _witness@3_0 = erlang:element(4, _witness@0_0),
        (erlang:element(2,
                        _witness@3_0)):'ImpliedAssocBound::Label::label'(_witness@3_0,
                                                                         begin
                                                                             _witness@2_0 =
                                                                                 erlang:element(3,
                                                                                                _witness@0_0),
                                                                             (erlang:element(2,
                                                                                             _witness@2_0)):'ImpliedAssocBound::HasItem::item'(_witness@2_0,
                                                                                                                                               _t@1_0)
                                                                         end)
    end.

crate_labels() ->
    {labelled({{'impl ImpliedAssocBound::HasItem for ImpliedAssocBound::Crate',
                'Elixir.Patina.ImpliedAssocBound.Crate'},
               {'impl ImpliedAssocBound::Label for ImpliedAssocBound::Tag',
                'Elixir.Patina.ImpliedAssocBound.Tag'}},
              'Elixir.Patina.ImpliedAssocBound.Crate'),
     labelled_with_items({{'impl ImpliedAssocBound::LabelledWithItems for ImpliedAssocBound::Crate',
                           'Elixir.Patina.ImpliedAssocBound.Crate',
                           {'impl ImpliedAssocBound::HasItem for ImpliedAssocBound::Crate',
                            'Elixir.Patina.ImpliedAssocBound.Crate'},
                           {'impl ImpliedAssocBound::Label for ImpliedAssocBound::Tag',
                            'Elixir.Patina.ImpliedAssocBound.Tag'}}},
                         'Elixir.Patina.ImpliedAssocBound.Crate')}.
