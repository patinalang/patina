-file("tests/ui/src/custom_types/repr_patina_default.pt",
      1).

-module('Elixir.Patina.ReprPatinaDefault').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         atom_structs/0,
         inherited_atoms/0,
         explicit_atoms/0,
         atom_patterns/0,
         greedy_first/1,
         greedy_second/1,
         greedy_text/1,
         greedy_nil/0,
         small_open/1,
         small_text/1,
         small_nil/0,
         classify_small/1,
         large_open/1,
         large_text/1,
         multiple_first/1,
         multiple_second/1,
         multiple_text/1,
         later_inferred/1,
         later_explicit/1,
         wrapper_pair/1,
         wrapper_open/1,
         classify_wrapper/1,
         reserved_wrapper_reserved/1,
         reserved_wrapper_open/1,
         reserved_wrapper_available/1,
         override_inferred/1,
         override_record/1,
         unit/0,
         newtype/1,
         record/2,
         record_value/1]).

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

atom_structs() ->
    {'Elixir.Patina.ReprPatinaDefault.AtomUnit',
     'Elixir.Patina.ReprPatinaDefault.AtomBraced',
     'Elixir.Patina.ReprPatinaDefault.AtomTuple'}.

inherited_atoms() ->
    {'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Unit',
     'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Braced',
     'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Tuple'}.

explicit_atoms() ->
    {'Elixir.Patina.ReprPatinaDefault.ExplicitAtom.Unit',
     'Elixir.Patina.ReprPatinaDefault.ExplicitAtom.Braced',
     'Elixir.Patina.ReprPatinaDefault.ExplicitAtom.Tuple'}.

atom_patterns() ->
    'Elixir.Patina.ReprPatinaDefault.AtomUnit' =
        'Elixir.Patina.ReprPatinaDefault.AtomUnit',
    'Elixir.Patina.ReprPatinaDefault.AtomBraced' =
        'Elixir.Patina.ReprPatinaDefault.AtomBraced',
    'Elixir.Patina.ReprPatinaDefault.AtomTuple' =
        'Elixir.Patina.ReprPatinaDefault.AtomTuple',
    {case
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Unit'
         of
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Unit' ->
             true;
         _ -> false
     end,
     case
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Braced'
         of
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Braced' ->
             true;
         _ -> false
     end,
     case
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Tuple'
         of
         'Elixir.Patina.ReprPatinaDefault.InheritedAtom.Tuple' ->
             true;
         _ -> false
     end}.

greedy_first(_value@0_0) -> _value@0_0.

greedy_second(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.Greedy':'$patina_constructor__Second'(_value@0_0).

greedy_text(_value@0_0) -> _value@0_0.

greedy_nil() ->
    'Elixir.Patina.ReprPatinaDefault.Greedy.Nil'.

small_open(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso
                     erlang:element(1, _value@1_0) =:=
                         'Elixir.Patina.ReprPatinaDefault.Small.Open' ->
            {'Elixir.Patina.ReprPatinaDefault.Small.Open',
             _value@1_0};
        _value@1_0 when erlang:is_binary(_value@1_0) ->
            {'Elixir.Patina.ReprPatinaDefault.Small.Open',
             _value@1_0};
        _value@1_0
            when _value@1_0 =:=
                     'Elixir.Patina.ReprPatinaDefault.Small.Nil' ->
            {'Elixir.Patina.ReprPatinaDefault.Small.Open',
             _value@1_0};
        _value@1_0
            when erlang:is_record(_value@1_0,
                                  'Elixir.Patina.ReprPatinaDefault.Small',
                                  'Pair') ->
            {'Elixir.Patina.ReprPatinaDefault.Small.Open',
             _value@1_0};
        _value@1_0 -> _value@1_0
    end.

small_text(_value@0_0) -> _value@0_0.

small_nil() ->
    'Elixir.Patina.ReprPatinaDefault.Small.Nil'.

classify_small(_value@0_0) ->
    case _value@0_0 of
        {'Elixir.Patina.ReprPatinaDefault.Small.Open', _} ->
            open;
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso
                          erlang:element(1, _value@1_0) =:=
                              'Elixir.Patina.ReprPatinaDefault.Small.Open'),
                 not erlang:is_binary(_value@1_0),
                 not
                     (_value@1_0 =:=
                          'Elixir.Patina.ReprPatinaDefault.Small.Nil'),
                 not
                     erlang:is_record(_value@1_0,
                                      'Elixir.Patina.ReprPatinaDefault.Small',
                                      'Pair') ->
            open;
        _value@2_0 = _ when erlang:is_binary(_value@2_0) ->
            text;
        'Elixir.Patina.ReprPatinaDefault.Small.Nil' -> nil;
        #'Elixir.Patina.ReprPatinaDefault.Small':'Pair'{'0' = _,
                                                        '1' = _} ->
            pair
    end.

large_open(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.Large':'$patina_constructor__Open'(_value@0_0).

large_text(_value@0_0) -> _value@0_0.

multiple_first(_value@0_0) -> _value@0_0.

multiple_second(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.MultipleOpen':'$patina_constructor__Second'(_value@0_0).

multiple_text(_value@0_0) -> _value@0_0.

later_inferred(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.LaterExplicit':'$patina_constructor__Inferred'(_value@0_0).

later_explicit(_value@0_0) -> _value@0_0.

wrapper_pair(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.WrapperConflict':'$patina_constructor__Pair'(_value@0_0).

wrapper_open(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when erlang:is_record(_value@1_0,
                                  'Elixir.Patina.ReprPatinaDefault.WrapperConflict',
                                  'Pair') ->
            {open, _value@1_0};
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= open ->
            {open, _value@1_0};
        _value@1_0 -> _value@1_0
    end.

classify_wrapper(_value@0_0) ->
    case _value@0_0 of
        #'Elixir.Patina.ReprPatinaDefault.WrapperConflict':'Pair'{'0'
                                                                      = _} ->
            pair;
        {open, _} -> open;
        _value@1_0 = _
            when not
                     erlang:is_record(_value@1_0,
                                      'Elixir.Patina.ReprPatinaDefault.WrapperConflict',
                                      'Pair'),
                 not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= open) ->
            open
    end.

reserved_wrapper_reserved(_value@0_0) ->
    {open, _value@0_0}.

reserved_wrapper_open(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.ReservedWrapper':'$patina_constructor__open'(_value@0_0).

reserved_wrapper_available(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= open ->
            {later, _value@1_0};
        _value@1_0
            when erlang:is_record(_value@1_0,
                                  'Elixir.Patina.ReprPatinaDefault.ReservedWrapper',
                                  open) ->
            {later, _value@1_0};
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= later ->
            {later, _value@1_0};
        _value@1_0 -> _value@1_0
    end.

override_inferred(_value@0_0) -> _value@0_0.

override_record(_value@0_0) ->
    'Elixir.Patina.ReprPatinaDefault.Override':'$patina_constructor__Record'(_value@0_0).

unit() -> 'Elixir.Patina.ReprPatinaDefault.Unit'.

newtype(_value@0_0) -> _value@0_0.

record(_value@0_0, _label@1_0) ->
    'Elixir.Patina.ReprPatinaDefault.Record':'$patina_constructor__Record'(_value@0_0,
                                                                           _label@1_0).

record_value(_record@0_0) ->
    _record@0_0#'Elixir.Patina.ReprPatinaDefault.Record':'Record'.value.
