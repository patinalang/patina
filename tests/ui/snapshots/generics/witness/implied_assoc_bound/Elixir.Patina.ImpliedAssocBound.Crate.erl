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
         'ImpliedAssocBound::HasItem::$patina_witness'/0,
         'ImpliedAssocBound::LabelledWithItems::other'/2,
         'ImpliedAssocBound::LabelledWithItems::$patina_witness'/0]).

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

'ImpliedAssocBound::HasItem::$patina_type::Item'(_witness_self@0_0 =
                                                     {_, {}, {}}) ->
    {}.

'ImpliedAssocBound::HasItem::item'(_witness_self@0_0 =
                                       {_, {}, {}},
                                   _self@1_0) ->
    'Elixir.Patina.ImpliedAssocBound.Tag'.

'ImpliedAssocBound::HasItem::$patina_witness'() ->
    {{fun 'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::HasItem::$patina_type::Item'/1,
      fun 'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::HasItem::item'/2},
     {},
     {}}.

'ImpliedAssocBound::LabelledWithItems::other'(_witness_self@0_0 =
                                                  {_,
                                                   {_witness_supertrait@1_0,
                                                    _witness_supertrait@2_0},
                                                   {}},
                                              _self@3_0) ->
    1.

'ImpliedAssocBound::LabelledWithItems::$patina_witness'() ->
    {{fun 'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::LabelledWithItems::other'/2},
     {{{fun 'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::HasItem::$patina_type::Item'/1,
        fun 'Elixir.Patina.ImpliedAssocBound.Crate':'ImpliedAssocBound::HasItem::item'/2},
       {},
       {}},
      {{fun 'Elixir.Patina.ImpliedAssocBound.Tag':'ImpliedAssocBound::Label::label'/2},
       {},
       {}}},
     {}}.
