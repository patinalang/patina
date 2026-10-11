-file("tests/ui/src/flow_control/for_loops.pt", 1).

-module('Elixir.Patina.ForLoops').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'sum_list$patina_loop_1'/2,
         sum_list/1,
         'sum_twice$patina_loop_1'/2,
         'sum_twice$patina_loop_2'/2,
         sum_twice/1,
         'sum_range$patina_loop_1'/2,
         sum_range/2,
         'sum_range_inclusive$patina_loop_1'/2,
         sum_range_inclusive/2,
         'weighted_map_sum$patina_loop_1'/2,
         weighted_map_sum/1,
         'map_entry_count$patina_loop_1'/2,
         map_entry_count/1,
         'count$patina_loop_1'/3,
         count_list/1,
         count_map/1,
         'total$patina_loop_1'/3,
         total_range/1,
         total_list/1,
         'skip_odd_until$patina_loop_1'/3,
         skip_odd_until/2,
         'find_index$patina_loop_1'/3,
         find_index/2,
         'first_pair_with_sum$patina_loop_2'/4,
         'first_pair_with_sum$patina_loop_1'/4,
         first_pair_with_sum/2,
         'count_skipping_rows$patina_loop_2'/2,
         'count_skipping_rows$patina_loop_1'/2,
         count_skipping_rows/1,
         'countdown$patina_loop_1'/2,
         countdown/1,
         bag_count/1]).

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

'sum_list$patina_loop_1'(_iter@2_0, _total@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@2_0)
                of
                nil ->
                    _branch_value@6_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _total@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _total@1_1 = _total@1_0,
                    _branch_value@6_0;
                _value@5_0 = {_x@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin _total@1_1 = _total@1_0 + _x@3_0, {} end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _total@1_1).

sum_list(_xs@0_0) ->
    _total@1_0 = 0,
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _xs@0_0)
        of
        _iter@2_0 ->
            case 'sum_list$patina_loop_1'(_iter@2_0, _total@1_0) of
                {break, 0, _loop_value@7_0, _iter@2_1, _total@1_1} ->
                    _loop_value@7_0
            end
    end,
    _total@1_1.

'sum_twice$patina_loop_1'(_iter@2_0, _total@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@2_0)
                of
                nil ->
                    _branch_value@6_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _total@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _total@1_1 = _total@1_0,
                    _branch_value@6_0;
                _value@5_0 = {_x@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin _total@1_1 = _total@1_0 + _x@3_0, {} end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _total@1_1).

'sum_twice$patina_loop_2'(_iter@8_0, _total@1_1) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@8_0)
                of
                nil ->
                    _branch_value@12_0 = '$patina_marker':break({break,
                                                                 0,
                                                                 {},
                                                                 _iter@8_0,
                                                                 _total@1_1}),
                    _iter@8_1 = _iter@8_0,
                    _total@1_2 = _total@1_1,
                    _branch_value@12_0;
                _value@11_0 = {_x@9_0, _next@10_0}
                    when erlang:is_tuple(_value@11_0) andalso
                             erlang:tuple_size(_value@11_0) =:= 2 ->
                    _iter@8_1 = _next@10_0,
                    begin _total@1_2 = _total@1_1 + _x@9_0 * 100, {} end
            end
        end,
    '$patina_marker':continue(_iter@8_1, _total@1_2).

sum_twice(_xs@0_0) ->
    _total@1_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::ListIter<_>',
                                                                     'Elixir.Patina.Core.ListIter'}},
                                                                   'Elixir.Patina.Core.List':iter(_xs@0_0))
        of
        _iter@2_0 ->
            case 'sum_twice$patina_loop_1'(_iter@2_0, _total@1_0) of
                {break, 0, _loop_value@7_0, _iter@2_1, _total@1_1} ->
                    _loop_value@7_0
            end
    end,
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _xs@0_0)
        of
        _iter@8_0 ->
            case 'sum_twice$patina_loop_2'(_iter@8_0, _total@1_1) of
                {break, 0, _loop_value@13_0, _iter@8_1, _total@1_2} ->
                    _loop_value@13_0
            end
    end,
    _total@1_2.

'sum_range$patina_loop_1'(_iter@3_0, _total@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.Range':'Core::Iterator::next'({'impl Core::Iterator for Core::Range<int>',
                                                                   'Elixir.Patina.Core.Range'},
                                                                  _iter@3_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@3_0,
                                                                _total@2_0}),
                    _iter@3_1 = _iter@3_0,
                    _total@2_1 = _total@2_0,
                    _branch_value@7_0;
                _value@6_0 = {_n@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin _total@2_1 = _total@2_0 + _n@4_0, {} end
            end
        end,
    '$patina_marker':continue(_iter@3_1, _total@2_1).

sum_range(_start@0_0, _end@1_0) ->
    _total@2_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::Range<int>',
                                                                     'Elixir.Patina.Core.Range'}},
                                                                   'Elixir.Patina.Core.Range':'$patina_constructor__Range'(_start@0_0,
                                                                                                                           _end@1_0))
        of
        _iter@3_0 ->
            case 'sum_range$patina_loop_1'(_iter@3_0, _total@2_0) of
                {break, 0, _loop_value@8_0, _iter@3_1, _total@2_1} ->
                    _loop_value@8_0
            end
    end,
    _total@2_1.

'sum_range_inclusive$patina_loop_1'(_iter@3_0,
                                    _total@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next'({'impl Core::Iterator for Core::RangeInclusive<int>',
                                                                            'Elixir.Patina.Core.RangeInclusive'},
                                                                           _iter@3_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@3_0,
                                                                _total@2_0}),
                    _iter@3_1 = _iter@3_0,
                    _total@2_1 = _total@2_0,
                    _branch_value@7_0;
                _value@6_0 = {_n@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin _total@2_1 = _total@2_0 + _n@4_0, {} end
            end
        end,
    '$patina_marker':continue(_iter@3_1, _total@2_1).

sum_range_inclusive(_start@0_0, _end@1_0) ->
    _total@2_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::RangeInclusive<int>',
                                                                     'Elixir.Patina.Core.RangeInclusive'}},
                                                                   'Elixir.Patina.Core.RangeInclusive':new(_start@0_0,
                                                                                                           _end@1_0))
        of
        _iter@3_0 ->
            case 'sum_range_inclusive$patina_loop_1'(_iter@3_0,
                                                     _total@2_0)
                of
                {break, 0, _loop_value@8_0, _iter@3_1, _total@2_1} ->
                    _loop_value@8_0
            end
    end,
    _total@2_1.

'weighted_map_sum$patina_loop_1'(_iter@2_0,
                                 _total@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.MapIter':'Core::Iterator::next'({'impl Core::Iterator for Core::MapIter<_, _>',
                                                                     'Elixir.Patina.Core.MapIter'},
                                                                    _iter@2_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _total@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _total@1_1 = _total@1_0,
                    _branch_value@7_0;
                _value@6_0 = {{_key@3_0, _value@4_0}, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@2_1 = _next@5_0,
                    begin
                        _total@1_1 = _total@1_0 + _key@3_0 * _value@4_0, {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _total@1_1).

weighted_map_sum(_map@0_0) ->
    _total@1_0 = 0,
    case
        'Elixir.Patina.Core.Map':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for #{_ => _}',
                                                                  'Elixir.Patina.Core.Map'},
                                                                 _map@0_0)
        of
        _iter@2_0 ->
            case 'weighted_map_sum$patina_loop_1'(_iter@2_0,
                                                  _total@1_0)
                of
                {break, 0, _loop_value@8_0, _iter@2_1, _total@1_1} ->
                    _loop_value@8_0
            end
    end,
    _total@1_1.

'map_entry_count$patina_loop_1'(_iter@2_0,
                                _count@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.MapIter':'Core::Iterator::next'({'impl Core::Iterator for Core::MapIter<_, _>',
                                                                     'Elixir.Patina.Core.MapIter'},
                                                                    _iter@2_0)
                of
                nil ->
                    _branch_value@5_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _count@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _count@1_1 = _count@1_0,
                    _branch_value@5_0;
                _value@4_0 = {_, _next@3_0}
                    when erlang:is_tuple(_value@4_0) andalso
                             erlang:tuple_size(_value@4_0) =:= 2 ->
                    _iter@2_1 = _next@3_0,
                    begin _count@1_1 = _count@1_0 + 1, {} end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _count@1_1).

map_entry_count(_map@0_0) ->
    _count@1_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::MapIter<_, _>',
                                                                     'Elixir.Patina.Core.MapIter'}},
                                                                   'Elixir.Patina.Core.Map':iter(_map@0_0))
        of
        _iter@2_0 ->
            case 'map_entry_count$patina_loop_1'(_iter@2_0,
                                                 _count@1_0)
                of
                {break, 0, _loop_value@6_0, _iter@2_1, _count@1_1} ->
                    _loop_value@6_0
            end
    end,
    _count@1_1.

'count$patina_loop_1'(_witness@1_0 = {_witness@0_0},
                      _iter@4_0, _n@3_0) ->
    _ = begin
            case begin
                     _witness@5_0 = erlang:element(1,
                                                   (erlang:element(2,
                                                                   _witness@0_0)):'Core::IntoIterator::type::IntoIter'(_witness@0_0)),
                     (erlang:element(2,
                                     _witness@5_0)):'Core::Iterator::next'(_witness@5_0,
                                                                           _iter@4_0)
                 end
                of
                nil ->
                    _branch_value@8_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@4_0,
                                                                _n@3_0}),
                    _iter@4_1 = _iter@4_0,
                    _n@3_1 = _n@3_0,
                    _branch_value@8_0;
                {some, {_, _next@6_0}} ->
                    _iter@4_1 = _next@6_0,
                    begin _n@3_1 = _n@3_0 + 1, {} end;
                _value@7_0 = {_, _next@6_0}
                    when not
                             ((erlang:is_tuple(_value@7_0) andalso
                                   erlang:tuple_size(_value@7_0) =:= 2)
                                  andalso
                                  erlang:element(1, _value@7_0) =:= some) ->
                    _iter@4_1 = _next@6_0,
                    begin _n@3_1 = _n@3_0 + 1, {} end
            end
        end,
    '$patina_marker':continue(_witness@1_0,
                              _iter@4_1,
                              _n@3_1).

count(_witness@1_0 = {_witness@0_0}, _items@2_0) ->
    _n@3_0 = 0,
    case (erlang:element(2,
                         _witness@0_0)):'Core::IntoIterator::into_iter'(_witness@0_0,
                                                                        _items@2_0)
        of
        _iter@4_0 ->
            case 'count$patina_loop_1'(_witness@1_0,
                                       _iter@4_0,
                                       _n@3_0)
                of
                {break, 0, _loop_value@9_0, _iter@4_1, _n@3_1} ->
                    _loop_value@9_0
            end
    end,
    _n@3_1.

count_list(_xs@0_0) ->
    count({{'impl Core::IntoIterator for [_]',
            'Elixir.Patina.Core.List'}},
          _xs@0_0).

count_map(_map@0_0) ->
    count({{'impl Core::IntoIterator for #{_ => _}',
            'Elixir.Patina.Core.Map'}},
          _map@0_0).

'total$patina_loop_1'(_witness@1_0 = {_witness@0_0},
                      _iter@4_0, _total@3_0) ->
    _ = begin
            case begin
                     _witness@5_0 = erlang:element(1,
                                                   (erlang:element(2,
                                                                   _witness@0_0)):'Core::IntoIterator::type::IntoIter'(_witness@0_0)),
                     (erlang:element(2,
                                     _witness@5_0)):'Core::Iterator::next'(_witness@5_0,
                                                                           _iter@4_0)
                 end
                of
                nil ->
                    _branch_value@9_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@4_0,
                                                                _total@3_0}),
                    _iter@4_1 = _iter@4_0,
                    _total@3_1 = _total@3_0,
                    _branch_value@9_0;
                _value@8_0 = {_item@6_0, _next@7_0}
                    when erlang:is_tuple(_value@8_0) andalso
                             erlang:tuple_size(_value@8_0) =:= 2 ->
                    _iter@4_1 = _next@7_0,
                    begin _total@3_1 = _total@3_0 + _item@6_0, {} end
            end
        end,
    '$patina_marker':continue(_witness@1_0,
                              _iter@4_1,
                              _total@3_1).

total(_witness@1_0 = {_witness@0_0}, _items@2_0) ->
    _total@3_0 = 0,
    case (erlang:element(2,
                         _witness@0_0)):'Core::IntoIterator::into_iter'(_witness@0_0,
                                                                        _items@2_0)
        of
        _iter@4_0 ->
            case 'total$patina_loop_1'(_witness@1_0,
                                       _iter@4_0,
                                       _total@3_0)
                of
                {break, 0, _loop_value@10_0, _iter@4_1, _total@3_1} ->
                    _loop_value@10_0
            end
    end,
    _total@3_1.

total_range(_end@0_0) ->
    total({{'impl Core::IntoIterator for _',
            'Elixir.Patina.Core.$impl',
            {'impl Core::Iterator for Core::Range<int>',
             'Elixir.Patina.Core.Range'}}},
          'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                  _end@0_0)).

total_list(_xs@0_0) ->
    total({{'impl Core::IntoIterator for [_]',
            'Elixir.Patina.Core.List'}},
          _xs@0_0).

'skip_odd_until$patina_loop_1'(_iter@3_0, _limit@1_0,
                               _total@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@3_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@3_0,
                                                                _total@2_0}),
                    _iter@3_1 = _iter@3_0,
                    _total@2_1 = _total@2_0,
                    _branch_value@7_0;
                _value@6_0 = {_x@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin
                        case _x@4_0 rem 2 =:= 1 of
                            true ->
                                '$patina_marker':continue(_iter@3_1,
                                                          _limit@1_0,
                                                          _total@2_0);
                            false -> {}
                        end,
                        case _x@4_0 > _limit@1_0 of
                            true ->
                                '$patina_marker':break({break,
                                                        0,
                                                        {},
                                                        _iter@3_1,
                                                        _total@2_0});
                            false -> {}
                        end,
                        _total@2_1 = _total@2_0 + _x@4_0,
                        {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@3_1,
                              _limit@1_0,
                              _total@2_1).

skip_odd_until(_xs@0_0, _limit@1_0) ->
    _total@2_0 = 0,
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _xs@0_0)
        of
        _iter@3_0 ->
            case 'skip_odd_until$patina_loop_1'(_iter@3_0,
                                                _limit@1_0,
                                                _total@2_0)
                of
                {break, 0, _loop_value@8_0, _iter@3_1, _total@2_1} ->
                    _loop_value@8_0
            end
    end,
    _total@2_1.

'find_index$patina_loop_1'(_iter@3_0, _target@1_0,
                           _index@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@3_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@3_0,
                                                                _index@2_0}),
                    _iter@3_1 = _iter@3_0,
                    _index@2_1 = _index@2_0,
                    _branch_value@7_0;
                _value@6_0 = {_x@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin
                        case _x@4_0 =:= _target@1_0 of
                            true ->
                                '$patina_marker':return({return,
                                                         _index@2_0,
                                                         _iter@3_1,
                                                         _index@2_0});
                            false -> {}
                        end,
                        _index@2_1 = _index@2_0 + 1,
                        {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@3_1,
                              _target@1_0,
                              _index@2_1).

find_index(_xs@0_0, _target@1_0) ->
    _index@2_0 = 0,
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _xs@0_0)
        of
        _iter@3_0 ->
            case 'find_index$patina_loop_1'(_iter@3_0,
                                            _target@1_0,
                                            _index@2_0)
                of
                {break, 0, _loop_value@8_0, _iter@3_1, _index@2_1} ->
                    _loop_value@8_0;
                {return, _loop_value@9_0, _iter@3_1, _index@2_1} ->
                    '$patina_marker':return(_loop_value@9_0)
            end
    end,
    -1.

'first_pair_with_sum$patina_loop_2'(_iter@7_0, _a@4_0,
                                    _sum@1_0, _found@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@7_0)
                of
                nil ->
                    _branch_value@12_0 = '$patina_marker':break({break,
                                                                 1,
                                                                 {},
                                                                 _iter@7_0,
                                                                 _found@2_0}),
                    _iter@7_1 = _iter@7_0,
                    _found@2_1 = _found@2_0,
                    _branch_value@12_0;
                _value@10_0 = {_b@8_0, _next@9_0}
                    when erlang:is_tuple(_value@10_0) andalso
                             erlang:tuple_size(_value@10_0) =:= 2 ->
                    _iter@7_1 = _next@9_0,
                    begin
                        case _b@8_0 =< _a@4_0 of
                            true ->
                                '$patina_marker':continue(_iter@7_1,
                                                          _a@4_0,
                                                          _sum@1_0,
                                                          _found@2_0);
                            false -> {}
                        end,
                        case _a@4_0 + _b@8_0 =:= _sum@1_0 of
                            true ->
                                _found@2_1 = {_a@4_0, _b@8_0},
                                '$patina_marker':break({break,
                                                        0,
                                                        {},
                                                        _iter@7_1,
                                                        _found@2_1});
                            false ->
                                _branch_value@11_0 = {},
                                _found@2_1 = _found@2_0,
                                _branch_value@11_0
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@7_1,
                              _a@4_0,
                              _sum@1_0,
                              _found@2_1).

'first_pair_with_sum$patina_loop_1'(_iter@3_0, _xs@0_0,
                                    _sum@1_0, _found@2_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@3_0)
                of
                nil ->
                    _branch_value@16_0 = '$patina_marker':break({break,
                                                                 0,
                                                                 {},
                                                                 _iter@3_0,
                                                                 _found@2_0}),
                    _iter@3_1 = _iter@3_0,
                    _found@2_1 = _found@2_0,
                    _branch_value@16_0;
                _value@6_0 = {_a@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin
                        case
                            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                                       'Elixir.Patina.Core.List'},
                                                                                      _xs@0_0)
                            of
                            _iter@7_0 ->
                                case
                                    'first_pair_with_sum$patina_loop_2'(_iter@7_0,
                                                                        _a@4_0,
                                                                        _sum@1_0,
                                                                        _found@2_0)
                                    of
                                    {break,
                                     1,
                                     _loop_value@13_0,
                                     _iter@7_1,
                                     _found@2_1} ->
                                        _loop_value@13_0;
                                    {break,
                                     _target_loop_depth@14_0,
                                     _loop_value@15_0,
                                     _iter@7_1,
                                     _found@2_1} ->
                                        '$patina_marker':break({break,
                                                                _target_loop_depth@14_0,
                                                                _loop_value@15_0,
                                                                _iter@3_1,
                                                                _found@2_1})
                                end
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@3_1,
                              _xs@0_0,
                              _sum@1_0,
                              _found@2_1).

first_pair_with_sum(_xs@0_0, _sum@1_0) ->
    _found@2_0 = {0, 0},
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _xs@0_0)
        of
        _iter@3_0 ->
            case 'first_pair_with_sum$patina_loop_1'(_iter@3_0,
                                                     _xs@0_0,
                                                     _sum@1_0,
                                                     _found@2_0)
                of
                {break, 0, _loop_value@17_0, _iter@3_1, _found@2_1} ->
                    _loop_value@17_0
            end
    end,
    _found@2_1.

'count_skipping_rows$patina_loop_2'(_iter@6_0,
                                    _visited@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@6_0)
                of
                nil ->
                    _branch_value@10_0 = '$patina_marker':break({break,
                                                                 1,
                                                                 {},
                                                                 _iter@6_0,
                                                                 _visited@1_0}),
                    _iter@6_1 = _iter@6_0,
                    _visited@1_1 = _visited@1_0,
                    _branch_value@10_0;
                _value@9_0 = {_x@7_0, _next@8_0}
                    when erlang:is_tuple(_value@9_0) andalso
                             erlang:tuple_size(_value@9_0) =:= 2 ->
                    _iter@6_1 = _next@8_0,
                    begin
                        case _x@7_0 < 0 of
                            true ->
                                '$patina_marker':break({continue,
                                                        0,
                                                        _iter@6_1,
                                                        _visited@1_0});
                            false -> {}
                        end,
                        _visited@1_1 = _visited@1_0 + 1,
                        {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@6_1, _visited@1_1).

'count_skipping_rows$patina_loop_1'(_iter@2_0,
                                    _visited@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'({'impl Core::Iterator for Core::ListIter<_>',
                                                                      'Elixir.Patina.Core.ListIter'},
                                                                     _iter@2_0)
                of
                nil ->
                    _branch_value@12_0 = '$patina_marker':break({break,
                                                                 0,
                                                                 {},
                                                                 _iter@2_0,
                                                                 _visited@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _visited@1_1 = _visited@1_0,
                    _branch_value@12_0;
                _value@5_0 = {_row@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin
                        case
                            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                                       'Elixir.Patina.Core.List'},
                                                                                      _row@3_0)
                            of
                            _iter@6_0 ->
                                case
                                    'count_skipping_rows$patina_loop_2'(_iter@6_0,
                                                                        _visited@1_0)
                                    of
                                    {break,
                                     1,
                                     _loop_value@11_0,
                                     _iter@6_1,
                                     _visited@1_1} ->
                                        _loop_value@11_0;
                                    {continue, 0, _iter@6_1, _visited@1_1} ->
                                        '$patina_marker':continue(_iter@2_1,
                                                                  _visited@1_1)
                                end
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _visited@1_1).

count_skipping_rows(_rows@0_0) ->
    _visited@1_0 = 0,
    case
        'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for [_]',
                                                                   'Elixir.Patina.Core.List'},
                                                                  _rows@0_0)
        of
        _iter@2_0 ->
            case 'count_skipping_rows$patina_loop_1'(_iter@2_0,
                                                     _visited@1_0)
                of
                {break, 0, _loop_value@13_0, _iter@2_1, _visited@1_1} ->
                    _loop_value@13_0
            end
    end,
    _visited@1_1.

'countdown$patina_loop_1'(_iter@2_0, _seen@1_0) ->
    _ = begin
            case
                'Elixir.Patina.ForLoops.Countdown':'Core::Iterator::next'({'impl Core::Iterator for ForLoops::Countdown',
                                                                           'Elixir.Patina.ForLoops.Countdown'},
                                                                          _iter@2_0)
                of
                nil ->
                    _branch_value@6_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _seen@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _seen@1_1 = _seen@1_0,
                    _branch_value@6_0;
                _value@5_0 = {_n@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin _seen@1_1 = [_n@3_0 | _seen@1_0], {} end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _seen@1_1).

countdown(_from@0_0) ->
    _seen@1_0 = [],
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for ForLoops::Countdown',
                                                                     'Elixir.Patina.ForLoops.Countdown'}},
                                                                   _from@0_0)
        of
        _iter@2_0 ->
            case 'countdown$patina_loop_1'(_iter@2_0, _seen@1_0) of
                {break, 0, _loop_value@7_0, _iter@2_1, _seen@1_1} ->
                    _loop_value@7_0
            end
    end,
    _seen@1_1.

bag_count(_items@0_0) ->
    count({{'impl Core::IntoIterator for ForLoops::Bag',
            'Elixir.Patina.ForLoops.Bag'}},
          _items@0_0).
