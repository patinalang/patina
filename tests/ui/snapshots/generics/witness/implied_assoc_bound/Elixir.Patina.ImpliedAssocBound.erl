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

labelled(_witness_bounds@2_0 = {_witness_bound@0_0,
                                _witness_bound@1_0},
         _t@3_0) ->
    (erlang:element(1,
                    erlang:element(1,
                                   _witness_bound@1_0)))(_witness_bound@1_0,
                                                         (erlang:element(2,
                                                                         erlang:element(1,
                                                                                        _witness_bound@0_0)))(_witness_bound@0_0,
                                                                                                              _t@3_0)).

labelled_with_items(_witness_bounds@1_0 =
                        {_witness_bound@0_0},
                    _t@2_0) ->
    begin
        _witness@4_0 = erlang:element(2,
                                      erlang:element(2, _witness_bound@0_0)),
        (erlang:element(1,
                        erlang:element(1, _witness@4_0)))(_witness@4_0,
                                                          begin
                                                              _witness@3_0 =
                                                                  erlang:element(1,
                                                                                 erlang:element(2,
                                                                                                _witness_bound@0_0)),
                                                              (erlang:element(2,
                                                                              erlang:element(1,
                                                                                             _witness@3_0)))(_witness@3_0,
                                                                                                             _t@2_0)
                                                          end)
    end.

crate_labels() ->
    {labelled({'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::HasItem::$patina_witness'(),
               'Elixir.Patina.ImpliedAssocBound.Tag':'ImpliedAssocBound::Label::$patina_witness'()},
              'Elixir.Patina.ImpliedAssocBound.Crate'),
     labelled_with_items({'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::LabelledWithItems::$patina_witness'()},
                         'Elixir.Patina.ImpliedAssocBound.Crate')}.
