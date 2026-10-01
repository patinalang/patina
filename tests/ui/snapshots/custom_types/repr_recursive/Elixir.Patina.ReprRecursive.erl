-file("tests/ui/src/custom_types/repr_recursive.pt", 1).

-module('Elixir.Patina.ReprRecursive').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         endless_field/1,
         endless_tuple_field/1,
         struct_cycle/0,
         struct_depth/1,
         empty/1,
         nested_empty/1,
         empty_tuple/1,
         generic_empty/1,
         recursive_void/1,
         list/0,
         sum/1,
         tree/0,
         tree_sum/1,
         chain/0,
         depth/1,
         mutual/0,
         mutual_depth/1,
         reverse_mutual/0,
         generic/0,
         generic_value/1,
         growing/0,
         growing_field/1,
         list_boundary/0,
         map_boundary/0,
         function_boundary/0,
         call_boundary/1,
         wrapped_tuple/0,
         fixed_generic/0,
         fixed_record_generic/0,
         tuple_boundary/0,
         tuple_value/1,
         tagged/0,
         tagged_depth/1,
         record/0,
         record_depth/1,
         overridden/0,
         overridden_depth/1]).

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

endless_field(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprRecursive.Endless':'Endless'.'0'.

endless_tuple_field(_value@0_0) ->
    {_next@1_0, _} = _value@0_0,
    _next@1_0.

struct_cycle() ->
    'Elixir.Patina.ReprRecursive.Outer':'$patina_constructor__Outer'('Elixir.Patina.ReprRecursive.Inner':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.Outer':'$patina_constructor__Outer'('Elixir.Patina.ReprRecursive.Inner.End'))).

struct_depth(_value@0_0) ->
    case
        _value@0_0#'Elixir.Patina.ReprRecursive.Outer':'Outer'.'0'
        of
        'Elixir.Patina.ReprRecursive.Inner.End' -> 0;
        #'Elixir.Patina.ReprRecursive.Inner':'Next'{'0' =
                                                        _outer@1_0} ->
            1 + struct_depth(_outer@1_0)
    end.

empty(_value@0_0) -> case _value@0_0 of  end.

nested_empty(_value@0_0) -> case _value@0_0 of  end.

empty_tuple(_value@0_0) -> case _value@0_0 of  end.

generic_empty(_value@0_0) -> _value@0_0.

recursive_void(_value@0_0) -> case _value@0_0 of  end.

list() ->
    'Elixir.Patina.ReprRecursive.List':'$patina_constructor__Cons'(1,
                                                                   'Elixir.Patina.ReprRecursive.List':'$patina_constructor__Cons'(2,
                                                                                                                                  'Elixir.Patina.ReprRecursive.List.Nil')).

sum(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.List.Nil' -> 0;
        #'Elixir.Patina.ReprRecursive.List':'Cons'{'0' =
                                                       _head@1_0,
                                                   '1' = _tail@2_0} ->
            _head@1_0 + sum(_tail@2_0)
    end.

tree() ->
    'Elixir.Patina.ReprRecursive.Tree':'$patina_constructor__Branch'(3,
                                                                     4).

tree_sum(_value@0_0) ->
    case _value@0_0 of
        _value@2_0 = _n@1_0
            when erlang:is_integer(_value@2_0) ->
            _n@1_0;
        #'Elixir.Patina.ReprRecursive.Tree':'Branch'{'0' =
                                                         _left@3_0,
                                                     '1' = _right@4_0} ->
            tree_sum(_left@3_0) + tree_sum(_right@4_0)
    end.

chain() ->
    'Elixir.Patina.ReprRecursive.Chain':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.Chain':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.Chain.End')).

depth(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.Chain.End' -> 0;
        #'Elixir.Patina.ReprRecursive.Chain':'Next'{'0' =
                                                        _tail@1_0} ->
            1 + depth(_tail@1_0)
    end.

mutual() ->
    'Elixir.Patina.ReprRecursive.Left':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.Right':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.Left.End')).

mutual_depth(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.Left.End' -> 0;
        #'Elixir.Patina.ReprRecursive.Left':'Next'{'0' =
                                                       _right@1_0} ->
            case _right@1_0 of
                'Elixir.Patina.ReprRecursive.Right.End' -> 1;
                #'Elixir.Patina.ReprRecursive.Right':'Next'{'0' =
                                                                _left@2_0} ->
                    2 + mutual_depth(_left@2_0)
            end
    end.

reverse_mutual() ->
    'Elixir.Patina.ReprRecursive.RightFirst':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.LeftLater':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.RightFirst.End')).

generic() ->
    'Elixir.Patina.ReprRecursive.Generic':'$patina_constructor__Next'(9).

generic_value(_value@0_0) ->
    case _value@0_0 of
        _value@2_0 = _n@1_0
            when erlang:is_integer(_value@2_0) ->
            _n@1_0;
        #'Elixir.Patina.ReprRecursive.Generic':'Next'{'0' =
                                                          _wrapper@3_0} ->
            generic_value(_wrapper@3_0)
    end.

growing() ->
    'Elixir.Patina.ReprRecursive.Growing':'$patina_constructor__Next'([2]).

growing_field(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = _ when erlang:is_integer(_value@1_0) -> [];
        #'Elixir.Patina.ReprRecursive.Growing':'Next'{'0' =
                                                          _inner@2_0} ->
            case _inner@2_0 of
                _value@4_0 = _list@3_0
                    when erlang:is_list(_value@4_0) ->
                    _list@3_0;
                _ -> []
            end
    end.

list_boundary() -> [[]].

map_boundary() -> #{child => #{}}.

function_boundary() -> fun function_boundary/0.

call_boundary(_value@0_0) -> _value@0_0().

wrapped_tuple() ->
    {6, 'Elixir.Patina.ReprRecursive.WrappedTuple.End'}.

fixed_generic() ->
    {wrapped,
     'Elixir.Patina.ReprRecursive.FixedGeneric.End'}.

fixed_record_generic() ->
    'Elixir.Patina.ReprRecursive.NativeWrapper':'$patina_constructor__NativeWrapper'('Elixir.Patina.ReprRecursive.FixedRecordGeneric.End').

tuple_boundary() ->
    {5, 'Elixir.Patina.ReprRecursive.TupleChain.End'}.

tuple_value(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.TupleChain.End' -> 0;
        _value@2_0 = _wrapper@1_0
            when erlang:is_tuple(_value@2_0) andalso
                     erlang:tuple_size(_value@2_0) =:= 2 ->
            {_n@3_0, _} = _wrapper@1_0,
            _n@3_0
    end.

tagged() ->
    {next, 'Elixir.Patina.ReprRecursive.TaggedChain.End'}.

tagged_depth(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.TaggedChain.End' -> 0;
        {next, _tail@1_0} -> 1 + tagged_depth(_tail@1_0)
    end.

record() ->
    'Elixir.Patina.ReprRecursive.RecordChain':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.RecordChain.End').

record_depth(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.RecordChain.End' -> 0;
        #'Elixir.Patina.ReprRecursive.RecordChain':'Next'{'0' =
                                                              _tail@1_0} ->
            1 + record_depth(_tail@1_0)
    end.

overridden() ->
    'Elixir.Patina.ReprRecursive.OverriddenChain':'$patina_constructor__Next'('Elixir.Patina.ReprRecursive.OverriddenChain.End').

overridden_depth(_value@0_0) ->
    case _value@0_0 of
        'Elixir.Patina.ReprRecursive.OverriddenChain.End' -> 0;
        #'Elixir.Patina.ReprRecursive.OverriddenChain':'Next'{'0'
                                                                  =
                                                                  _tail@1_0} ->
            1 + overridden_depth(_tail@1_0)
    end.
