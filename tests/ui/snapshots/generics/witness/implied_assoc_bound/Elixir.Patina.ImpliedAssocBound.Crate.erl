-file("tests/ui/src/generics/witness/implied_assoc_b"
      "ound.pt",
      23).

-module('Elixir.Patina.ImpliedAssocBound.Crate').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'ImpliedAssocBound::HasItem::$patina_type::Item'/1,
         'ImpliedAssocBound::HasItem::item'/2,
         'ImpliedAssocBound::LabelledWithItems::other'/2]).

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

'ImpliedAssocBound::HasItem::$patina_type::Item'(_witness@0_0 =
                                                     {'impl ImpliedAssocBound::HasItem for ImpliedAssocBound::Crate',
                                                      _}) ->
    {}.

'ImpliedAssocBound::HasItem::item'(_witness@0_0 =
                                       {'impl ImpliedAssocBound::HasItem for ImpliedAssocBound::Crate',
                                        _},
                                   _self@1_0) ->
    'Elixir.Patina.ImpliedAssocBound.Tag'.

'ImpliedAssocBound::LabelledWithItems::other'(_witness@0_0 =
                                                  {'impl ImpliedAssocBound::LabelledWithItems for ImpliedAssocBound::Crate',
                                                   _,
                                                   _witness@1_0,
                                                   _witness@2_0},
                                              _self@3_0) ->
    1.
