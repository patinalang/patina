-file("tests/ui/src/custom_types/phantom_data.pt", 1).

-module('Elixir.Patina.PhantomData').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         typed/1,
         typed_value/1,
         typed_marker/1,
         typed_destructure/1,
         typed_binding/1,
         typed_constant_pattern/1,
         typed_or_pattern/1,
         typed_update/1,
         typed_effect/1,
         marker_effect/0,
         phantom_in_loop_and_closure/1,
         map_iter/1,
         sum_values/1,
         first_entry/1,
         record/2,
         record_b/1,
         record_update/1,
         record_update_marker/1,
         record_destructure/1,
         pair/2,
         pair_second/1,
         tagged_pair/2,
         tagged_pair_b/1,
         tagged_pair_update/1,
         elixir_point/2,
         elixir_point_sum/1,
         marker/0,
         marker_inner/1,
         materialized_marker/0,
         is_some_marker/1,
         term_matches_phantom_struct/1,
         widen_source/1,
         narrow_sink/1]).

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

id(_value@0_0) -> _value@0_0.

typed(_n@0_0) -> _n@0_0.

typed_value(_value@0_0) -> _value@0_0.

typed_marker(_value@0_0) ->
    'Elixir.Patina.Core.PhantomData'.

typed_destructure(_value@0_0) ->
    _n@1_0 = _value@0_0,
    _n@1_0.

typed_binding(_value@0_0) ->
    _ = _value@0_0,
    id('Elixir.Patina.Core.PhantomData').

typed_constant_pattern(_value@0_0) ->
    case _value@0_0 of _n@1_0 -> _n@1_0 end.

typed_or_pattern(_value@0_0) ->
    case [] of
        {'$patina_label_0'} -> 0;
        _ ->
            case _value@0_0 of
                1 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                2 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _n@1_0 -> _n@1_0
            end
    end.

typed_update(_value@0_0) -> 7.

marker_with_side_effect() ->
    io:put_chars(<<"made a marker\n"/utf8>>),
    'Elixir.Patina.Core.PhantomData'.

typed_effect(_n@0_0) ->
    _1@1_0 = marker_with_side_effect(),
    _n@0_0.

marker_effect() ->
    _field_base@0_0 = typed_effect(1),
    'Elixir.Patina.Core.PhantomData'.

phantom_in_loop_and_closure(_n@0_0) ->
    _ = 'Elixir.Patina.Core.PhantomData',
    _count@1_0 = 0,
    _last@2_0 = 0,
    {_count@1_1, _last@2_1} = fun _$patina_loop(_count@1_0,
                                                _last@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _count@1_2,
                                           _last@2_2} ->
                                              {_count@1_2, _last@2_2};
                                          _ ->
                                              case _count@1_0 < _n@0_0 of
                                                  true ->
                                                      _last@2_1 = _count@1_0,
                                                      _count@1_1 = _count@1_0 +
                                                                       1,
                                                      {};
                                                  _ ->
                                                      _branch_value@3_0 = begin
                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                    _count@1_0,
                                                                                                    _last@2_0),
                                                                              []
                                                                          end,
                                                      _last@2_1 = _last@2_0,
                                                      _count@1_1 = _count@1_0,
                                                      _branch_value@3_0
                                              end,
                                              _$patina_loop(_count@1_1,
                                                            _last@2_1)
                                      end
                              end(_count@1_0, _last@2_0),
    _make@5_0 = fun (_value@4_0) -> _value@4_0 end,
    _make@5_0(_last@2_1).

map_iter(_map@0_0) ->
    'Elixir.Patina.PhantomData.MapIter':new(_map@0_0).

sum_values(_map@0_0) ->
    _iter@1_0 =
        'Elixir.Patina.PhantomData.MapIter':new(_map@0_0),
    _total@2_0 = 0,
    {_iter@1_1, _total@2_1} = fun _$patina_loop(_iter@1_0,
                                                _total@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@1_2,
                                           _total@2_2} ->
                                              {_iter@1_2, _total@2_2};
                                          _ ->
                                              case
                                                  'Elixir.Patina.PhantomData.MapIter':next(_iter@1_0)
                                                  of
                                                  nil ->
                                                      _branch_value@6_0 = begin
                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                    _iter@1_0,
                                                                                                    _total@2_0),
                                                                              []
                                                                          end,
                                                      _iter@1_1 = _iter@1_0,
                                                      _total@2_1 = _total@2_0,
                                                      _branch_value@6_0;
                                                  _value@5_0 = {{_, _value@3_0},
                                                                _rest@4_0}
                                                      when
                                                        erlang:is_tuple(_value@5_0)
                                                            andalso
                                                            erlang:tuple_size(_value@5_0)
                                                                =:= 2 ->
                                                      _iter@1_1 = _rest@4_0,
                                                      _total@2_1 = _total@2_0 +
                                                                       _value@3_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@1_1,
                                                            _total@2_1)
                                      end
                              end(_iter@1_0, _total@2_0),
    _total@2_1.

first_entry(_iter@0_0) ->
    case 'Elixir.Patina.PhantomData.MapIter':next(_iter@0_0)
        of
        _value@2_0 = {_entry@1_0, _}
            when erlang:is_tuple(_value@2_0) andalso
                     erlang:tuple_size(_value@2_0) =:= 2 ->
            _entry@1_0;
        nil -> nil
    end.

record(_a@0_0, _b@1_0) ->
    'Elixir.Patina.PhantomData.Record':'$patina_constructor__Record'(_a@0_0,
                                                                     _b@1_0).

record_b(_value@0_0) ->
    _value@0_0#'Elixir.Patina.PhantomData.Record':'Record'.b.

record_update(_value@0_0) ->
    _value@0_0#'Elixir.Patina.PhantomData.Record':'Record'{b
                                                               =
                                                               <<"updated"/utf8>>}.

record_update_marker(_value@0_0) ->
    _marker@1_0 = marker_with_side_effect(),
    _value@0_0#'Elixir.Patina.PhantomData.Record':'Record'{}.

record_destructure(_value@0_0) ->
    #'Elixir.Patina.PhantomData.Record':'Record'{a = _a@1_0,
                                                 b = _} =
        _value@0_0,
    _ = id('Elixir.Patina.Core.PhantomData'),
    _a@1_0.

pair(_a@0_0, _b@1_0) ->
    {'Elixir.Patina.PhantomData.Shape.Pair',
     _a@0_0,
     _b@1_0}.

pair_second(_value@0_0) ->
    case _value@0_0 of
        {'Elixir.Patina.PhantomData.Shape.Pair', _, _b@1_0} ->
            _b@1_0
    end.

tagged_pair(_a@0_0, _b@1_0) ->
    {'Elixir.Patina.PhantomData.TaggedPair',
     _a@0_0,
     _b@1_0}.

tagged_pair_b(_value@0_0) ->
    erlang:element(3, _value@0_0).

tagged_pair_update(_value@0_0) ->
    erlang:setelement(3, _value@0_0, 99).

elixir_point(_x@0_0, _y@1_0) ->
    ('Elixir.Patina.PhantomData.ElixirPoint':'__struct__'())#{x
                                                                  := _x@0_0,
                                                              y := _y@1_0}.

elixir_point_sum(_value@0_0) ->
    #{'__struct__' :=
          'Elixir.Patina.PhantomData.ElixirPoint',
      x := _x@1_0, y := _y@2_0} =
        _value@0_0,
    _x@1_0 + _y@2_0.

marker() -> 'Elixir.Patina.PhantomData.Marker'.

marker_inner(_value@0_0) ->
    'Elixir.Patina.Core.PhantomData'.

materialized_marker() ->
    'Elixir.Patina.Core.PhantomData'.

is_some_marker(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = _
            when _value@1_0 =:= 'Elixir.Patina.Core.PhantomData' ->
            true;
        nil -> false
    end.

term_matches_phantom_struct(_value@0_0) ->
    case _value@0_0 of
        #'Elixir.Patina.PhantomData.Record':'Record'{a = _a@1_0,
                                                     b = _value@2_0}
            when erlang:is_integer(_a@1_0),
                 erlang:is_binary(_value@2_0) ->
            _a@1_0;
        _ -> -1
    end.

widen_source(_source@0_0) -> _source@0_0.

narrow_sink(_sink@0_0) -> _sink@0_0.
