-file("tests/ui/src/flow_control/for_loops.pt", 1).

-module('Elixir.Patina.ForLoops').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         sum_list/1,
         sum_twice/1,
         sum_range/2,
         sum_range_inclusive/2,
         weighted_map_sum/1,
         map_entry_count/1,
         count_list/1,
         count_map/1,
         total_range/1,
         total_list/1,
         skip_odd_until/2,
         find_index/2,
         first_pair_with_sum/2,
         count_skipping_rows/1,
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

sum_list(_xs@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    begin
        _total@3_0 = 0,
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                      _xs@0_0)
            of
            _iter@4_0 ->
                case fun _$patina_loop(_iter@4_0, _total@3_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@2_0,
                                                                                              _iter@4_0)
                                         of
                                         nil ->
                                             _branch_value@8_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@4_0,
                                                                                             _total@3_0}),
                                                                     []
                                                                 end,
                                             _iter@4_1 = _iter@4_0,
                                             _total@3_1 = _total@3_0,
                                             _branch_value@8_0;
                                         _value@7_0 = {_x@5_0, _next@6_0}
                                             when erlang:is_tuple(_value@7_0)
                                                      andalso
                                                      erlang:tuple_size(_value@7_0)
                                                          =:= 2 ->
                                             _iter@4_1 = _next@6_0,
                                             begin
                                                 _total@3_1 = _total@3_0 +
                                                                  _x@5_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@4_1, _total@3_1)
                     end(_iter@4_0, _total@3_0)
                    of
                    {break, 0, _loop_value@9_0, _iter@4_1, _total@3_1} ->
                        _loop_value@9_0
                end
        end,
        _total@3_1
    end.

sum_twice(_xs@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    begin
        _total@5_0 = 0,
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                       'Elixir.Patina.Core.List':iter(_xs@0_0))
            of
            _iter@6_0 ->
                case fun _$patina_loop(_iter@6_0, _total@5_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@1_0,
                                                                                              _iter@6_0)
                                         of
                                         nil ->
                                             _branch_value@10_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@6_0,
                                                                                              _total@5_0}),
                                                                      []
                                                                  end,
                                             _iter@6_1 = _iter@6_0,
                                             _total@5_1 = _total@5_0,
                                             _branch_value@10_0;
                                         _value@9_0 = {_x@7_0, _next@8_0}
                                             when erlang:is_tuple(_value@9_0)
                                                      andalso
                                                      erlang:tuple_size(_value@9_0)
                                                          =:= 2 ->
                                             _iter@6_1 = _next@8_0,
                                             begin
                                                 _total@5_1 = _total@5_0 +
                                                                  _x@7_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@6_1, _total@5_1)
                     end(_iter@6_0, _total@5_0)
                    of
                    {break, 0, _loop_value@11_0, _iter@6_1, _total@5_1} ->
                        _loop_value@11_0
                end
        end,
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                      _xs@0_0)
            of
            _iter@12_0 ->
                case fun _$patina_loop(_iter@12_0, _total@5_1) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@1_0,
                                                                                              _iter@12_0)
                                         of
                                         nil ->
                                             _branch_value@16_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@12_0,
                                                                                              _total@5_1}),
                                                                      []
                                                                  end,
                                             _iter@12_1 = _iter@12_0,
                                             _total@5_2 = _total@5_1,
                                             _branch_value@16_0;
                                         _value@15_0 = {_x@13_0, _next@14_0}
                                             when erlang:is_tuple(_value@15_0)
                                                      andalso
                                                      erlang:tuple_size(_value@15_0)
                                                          =:= 2 ->
                                             _iter@12_1 = _next@14_0,
                                             begin
                                                 _total@5_2 = _total@5_1 +
                                                                  _x@13_0 * 100,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@12_1, _total@5_2)
                     end(_iter@12_0, _total@5_1)
                    of
                    {break, 0, _loop_value@17_0, _iter@12_1, _total@5_2} ->
                        _loop_value@17_0
                end
        end,
        _total@5_2
    end.

sum_range(_start@0_0, _end@1_0) ->
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.Range':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@3_0 =
        {_witness_Iterator@2_0},
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@3_0),
    begin
        _total@5_0 = 0,
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                       'Elixir.Patina.Core.Range':'$patina_constructor__Range'(_start@0_0,
                                                                                                                               _end@1_0))
            of
            _iter@6_0 ->
                case fun _$patina_loop(_iter@6_0, _total@5_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.Range':'Core::Iterator::next<int>'(_witness_Iterator@2_0,
                                                                                                _iter@6_0)
                                         of
                                         nil ->
                                             _branch_value@10_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@6_0,
                                                                                              _total@5_0}),
                                                                      []
                                                                  end,
                                             _iter@6_1 = _iter@6_0,
                                             _total@5_1 = _total@5_0,
                                             _branch_value@10_0;
                                         _value@9_0 = {_n@7_0, _next@8_0}
                                             when erlang:is_tuple(_value@9_0)
                                                      andalso
                                                      erlang:tuple_size(_value@9_0)
                                                          =:= 2 ->
                                             _iter@6_1 = _next@8_0,
                                             begin
                                                 _total@5_1 = _total@5_0 +
                                                                  _n@7_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@6_1, _total@5_1)
                     end(_iter@6_0, _total@5_0)
                    of
                    {break, 0, _loop_value@11_0, _iter@6_1, _total@5_1} ->
                        _loop_value@11_0
                end
        end,
        _total@5_1
    end.

sum_range_inclusive(_start@0_0, _end@1_0) ->
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@3_0 =
        {_witness_Iterator@2_0},
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@3_0),
    begin
        _total@5_0 = 0,
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                       'Elixir.Patina.Core.RangeInclusive':new(_start@0_0,
                                                                                                               _end@1_0))
            of
            _iter@6_0 ->
                case fun _$patina_loop(_iter@6_0, _total@5_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next<int>'(_witness_Iterator@2_0,
                                                                                                         _iter@6_0)
                                         of
                                         nil ->
                                             _branch_value@10_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@6_0,
                                                                                              _total@5_0}),
                                                                      []
                                                                  end,
                                             _iter@6_1 = _iter@6_0,
                                             _total@5_1 = _total@5_0,
                                             _branch_value@10_0;
                                         _value@9_0 = {_n@7_0, _next@8_0}
                                             when erlang:is_tuple(_value@9_0)
                                                      andalso
                                                      erlang:tuple_size(_value@9_0)
                                                          =:= 2 ->
                                             _iter@6_1 = _next@8_0,
                                             begin
                                                 _total@5_1 = _total@5_0 +
                                                                  _n@7_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@6_1, _total@5_1)
                     end(_iter@6_0, _total@5_0)
                    of
                    {break, 0, _loop_value@11_0, _iter@6_1, _total@5_1} ->
                        _loop_value@11_0
                end
        end,
        _total@5_1
    end.

weighted_map_sum(_map@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.Map':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.MapIter':'Core::Iterator::$patina_witness'(),
    begin
        _total@3_0 = 0,
        case
            'Elixir.Patina.Core.Map':'Core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                     _map@0_0)
            of
            _iter@4_0 ->
                case fun _$patina_loop(_iter@4_0, _total@3_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.MapIter':'Core::Iterator::next'(_witness_Iterator@2_0,
                                                                                             _iter@4_0)
                                         of
                                         nil ->
                                             _branch_value@9_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@4_0,
                                                                                             _total@3_0}),
                                                                     []
                                                                 end,
                                             _iter@4_1 = _iter@4_0,
                                             _total@3_1 = _total@3_0,
                                             _branch_value@9_0;
                                         _value@8_0 = {{_key@5_0, _value@6_0},
                                                       _next@7_0}
                                             when erlang:is_tuple(_value@8_0)
                                                      andalso
                                                      erlang:tuple_size(_value@8_0)
                                                          =:= 2 ->
                                             _iter@4_1 = _next@7_0,
                                             begin
                                                 _total@3_1 = _total@3_0 +
                                                                  _key@5_0 *
                                                                      _value@6_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@4_1, _total@3_1)
                     end(_iter@4_0, _total@3_0)
                    of
                    {break, 0, _loop_value@10_0, _iter@4_1, _total@3_1} ->
                        _loop_value@10_0
                end
        end,
        _total@3_1
    end.

map_entry_count(_map@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.MapIter':'Core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    begin
        _count@4_0 = 0,
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                       'Elixir.Patina.Core.Map':iter(_map@0_0))
            of
            _iter@5_0 ->
                case fun _$patina_loop(_iter@5_0, _count@4_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.MapIter':'Core::Iterator::next'(_witness_Iterator@1_0,
                                                                                             _iter@5_0)
                                         of
                                         nil ->
                                             _branch_value@8_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@5_0,
                                                                                             _count@4_0}),
                                                                     []
                                                                 end,
                                             _iter@5_1 = _iter@5_0,
                                             _count@4_1 = _count@4_0,
                                             _branch_value@8_0;
                                         _value@7_0 = {_, _next@6_0}
                                             when erlang:is_tuple(_value@7_0)
                                                      andalso
                                                      erlang:tuple_size(_value@7_0)
                                                          =:= 2 ->
                                             _iter@5_1 = _next@6_0,
                                             begin
                                                 _count@4_1 = _count@4_0 + 1, {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@5_1, _count@4_1)
                     end(_iter@5_0, _count@4_0)
                    of
                    {break, 0, _loop_value@9_0, _iter@5_1, _count@4_1} ->
                        _loop_value@9_0
                end
        end,
        _count@4_1
    end.

count(_witnesses_where_count@0_0, _items@1_0) ->
    _witness_IntoIterator@2_0 = erlang:element(1,
                                               _witnesses_where_count@0_0),
    _vtable_IntoIterator@3_0 = erlang:element(1,
                                              _witness_IntoIterator@2_0),
    _fn_IntoIterator_into_iter@4_0 = erlang:element(3,
                                                    _vtable_IntoIterator@3_0),
    _type_IntoIterator_IntoIter@5_0 = erlang:element(2,
                                                     _vtable_IntoIterator@3_0),
    _where_IntoIterator@6_0 =
        _type_IntoIterator_IntoIter@5_0(_witness_IntoIterator@2_0),
    _witness_Iterator@7_0 = erlang:element(1,
                                           _where_IntoIterator@6_0),
    _vtable_Iterator@8_0 = erlang:element(1,
                                          _witness_Iterator@7_0),
    _fn_Iterator_next@9_0 = erlang:element(2,
                                           _vtable_Iterator@8_0),
    begin
        _n@10_0 = 0,
        case
            _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                           _items@1_0)
            of
            _iter@11_0 ->
                case fun _$patina_loop(_iter@11_0, _n@10_0) ->
                             _ = begin
                                     case
                                         _fn_Iterator_next@9_0(_witness_Iterator@7_0,
                                                               _iter@11_0)
                                         of
                                         nil ->
                                             _branch_value@14_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@11_0,
                                                                                              _n@10_0}),
                                                                      []
                                                                  end,
                                             _iter@11_1 = _iter@11_0,
                                             _n@10_1 = _n@10_0,
                                             _branch_value@14_0;
                                         {some, {_, _next@12_0}} ->
                                             _iter@11_1 = _next@12_0,
                                             begin
                                                 _n@10_1 = _n@10_0 + 1, {}
                                             end;
                                         _value@13_0 = {_, _next@12_0}
                                             when not
                                                      ((erlang:is_tuple(_value@13_0)
                                                            andalso
                                                            erlang:tuple_size(_value@13_0)
                                                                =:= 2)
                                                           andalso
                                                           erlang:element(1,
                                                                          _value@13_0)
                                                               =:= some) ->
                                             _iter@11_1 = _next@12_0,
                                             begin _n@10_1 = _n@10_0 + 1, {} end
                                     end
                                 end,
                             _$patina_loop(_iter@11_1, _n@10_1)
                     end(_iter@11_0, _n@10_0)
                    of
                    {break, 0, _loop_value@15_0, _iter@11_1, _n@10_1} ->
                        _loop_value@15_0
                end
        end,
        _n@10_1
    end.

count_list(_xs@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    begin count(_witnesses_where_count@2_0, _xs@0_0) end.

count_map(_map@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.Map':'Core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    begin count(_witnesses_where_count@2_0, _map@0_0) end.

total(_witnesses_where_total@0_0, _items@1_0) ->
    _witness_IntoIterator@2_0 = erlang:element(1,
                                               _witnesses_where_total@0_0),
    _vtable_IntoIterator@3_0 = erlang:element(1,
                                              _witness_IntoIterator@2_0),
    _fn_IntoIterator_into_iter@4_0 = erlang:element(3,
                                                    _vtable_IntoIterator@3_0),
    _type_IntoIterator_IntoIter@5_0 = erlang:element(2,
                                                     _vtable_IntoIterator@3_0),
    _where_IntoIterator@6_0 =
        _type_IntoIterator_IntoIter@5_0(_witness_IntoIterator@2_0),
    _witness_Iterator@7_0 = erlang:element(1,
                                           _where_IntoIterator@6_0),
    _vtable_Iterator@8_0 = erlang:element(1,
                                          _witness_Iterator@7_0),
    _fn_Iterator_next@9_0 = erlang:element(2,
                                           _vtable_Iterator@8_0),
    begin
        _total@10_0 = 0,
        case
            _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                           _items@1_0)
            of
            _iter@11_0 ->
                case fun _$patina_loop(_iter@11_0, _total@10_0) ->
                             _ = begin
                                     case
                                         _fn_Iterator_next@9_0(_witness_Iterator@7_0,
                                                               _iter@11_0)
                                         of
                                         nil ->
                                             _branch_value@15_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@11_0,
                                                                                              _total@10_0}),
                                                                      []
                                                                  end,
                                             _iter@11_1 = _iter@11_0,
                                             _total@10_1 = _total@10_0,
                                             _branch_value@15_0;
                                         _value@14_0 = {_item@12_0, _next@13_0}
                                             when erlang:is_tuple(_value@14_0)
                                                      andalso
                                                      erlang:tuple_size(_value@14_0)
                                                          =:= 2 ->
                                             _iter@11_1 = _next@13_0,
                                             begin
                                                 _total@10_1 = _total@10_0 +
                                                                   _item@12_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@11_1, _total@10_1)
                     end(_iter@11_0, _total@10_0)
                    of
                    {break, 0, _loop_value@16_0, _iter@11_1, _total@10_1} ->
                        _loop_value@16_0
                end
        end,
        _total@10_1
    end.

total_range(_end@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.Range':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _witnesses_where_total@4_0 =
        {_witness_IntoIterator@3_0},
    begin
        total(_witnesses_where_total@4_0,
              'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                      _end@0_0))
    end.

total_list(_xs@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witnesses_where_total@2_0 =
        {_witness_IntoIterator@1_0},
    begin total(_witnesses_where_total@2_0, _xs@0_0) end.

skip_odd_until(_xs@0_0, _limit@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    begin
        _total@4_0 = 0,
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                      _xs@0_0)
            of
            _iter@5_0 ->
                case fun _$patina_loop(_iter@5_0, _total@4_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@3_0,
                                                                                              _iter@5_0)
                                         of
                                         nil ->
                                             _branch_value@9_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@5_0,
                                                                                             _total@4_0}),
                                                                     []
                                                                 end,
                                             _iter@5_1 = _iter@5_0,
                                             _total@4_1 = _total@4_0,
                                             _branch_value@9_0;
                                         _value@8_0 = {_x@6_0, _next@7_0}
                                             when erlang:is_tuple(_value@8_0)
                                                      andalso
                                                      erlang:tuple_size(_value@8_0)
                                                          =:= 2 ->
                                             _iter@5_1 = _next@7_0,
                                             begin
                                                 case _x@6_0 rem 2 =:= 1 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':continue(_iter@5_1,
                                                                                       _total@4_0),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end,
                                                 case _x@6_0 > _limit@1_0 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':break({break,
                                                                                     0,
                                                                                     {},
                                                                                     _iter@5_1,
                                                                                     _total@4_0}),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end,
                                                 _total@4_1 = _total@4_0 +
                                                                  _x@6_0,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@5_1, _total@4_1)
                     end(_iter@5_0, _total@4_0)
                    of
                    {break, 0, _loop_value@10_0, _iter@5_1, _total@4_1} ->
                        _loop_value@10_0
                end
        end,
        _total@4_1
    end.

find_index(_xs@0_0, _target@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    begin
        _index@4_0 = 0,
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                      _xs@0_0)
            of
            _iter@5_0 ->
                case fun _$patina_loop(_iter@5_0, _index@4_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@3_0,
                                                                                              _iter@5_0)
                                         of
                                         nil ->
                                             _branch_value@9_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@5_0,
                                                                                             _index@4_0}),
                                                                     []
                                                                 end,
                                             _iter@5_1 = _iter@5_0,
                                             _index@4_1 = _index@4_0,
                                             _branch_value@9_0;
                                         _value@8_0 = {_x@6_0, _next@7_0}
                                             when erlang:is_tuple(_value@8_0)
                                                      andalso
                                                      erlang:tuple_size(_value@8_0)
                                                          =:= 2 ->
                                             _iter@5_1 = _next@7_0,
                                             begin
                                                 case _x@6_0 =:= _target@1_0 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':return({return,
                                                                                      _index@4_0,
                                                                                      _iter@5_1,
                                                                                      _index@4_0}),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end,
                                                 _index@4_1 = _index@4_0 + 1,
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@5_1, _index@4_1)
                     end(_iter@5_0, _index@4_0)
                    of
                    {break, 0, _loop_value@10_0, _iter@5_1, _index@4_1} ->
                        _loop_value@10_0;
                    {return, _loop_value@11_0, _iter@5_1, _index@4_1} ->
                        '$patina_marker':return(_loop_value@11_0),
                        []
                end
        end,
        -1
    end.

first_pair_with_sum(_xs@0_0, _sum@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    begin
        _found@4_0 = {0, 0},
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                      _xs@0_0)
            of
            _iter@5_0 ->
                case fun _$patina_loop(_iter@5_0, _found@4_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@3_0,
                                                                                              _iter@5_0)
                                         of
                                         nil ->
                                             _branch_value@18_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@5_0,
                                                                                              _found@4_0}),
                                                                      []
                                                                  end,
                                             _iter@5_1 = _iter@5_0,
                                             _found@4_1 = _found@4_0,
                                             _branch_value@18_0;
                                         _value@8_0 = {_a@6_0, _next@7_0}
                                             when erlang:is_tuple(_value@8_0)
                                                      andalso
                                                      erlang:tuple_size(_value@8_0)
                                                          =:= 2 ->
                                             _iter@5_1 = _next@7_0,
                                             begin
                                                 case
                                                     'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                                                               _xs@0_0)
                                                     of
                                                     _iter@9_0 ->
                                                         case fun
                                                                  _$patina_loop(_iter@9_0,
                                                                                _found@4_0) ->
                                                                      _ = begin
                                                                              case
                                                                                  'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@3_0,
                                                                                                                                       _iter@9_0)
                                                                                  of
                                                                                  nil ->
                                                                                      _branch_value@14_0 =
                                                                                          begin
                                                                                              '$patina_marker':break({break,
                                                                                                                      1,
                                                                                                                      {},
                                                                                                                      _iter@9_0,
                                                                                                                      _found@4_0}),
                                                                                              []
                                                                                          end,
                                                                                      _iter@9_1 =
                                                                                          _iter@9_0,
                                                                                      _found@4_1 =
                                                                                          _found@4_0,
                                                                                      _branch_value@14_0;
                                                                                  _value@12_0 =
                                                                                      {_b@10_0,
                                                                                       _next@11_0}
                                                                                      when
                                                                                        erlang:is_tuple(_value@12_0)
                                                                                            andalso
                                                                                            erlang:tuple_size(_value@12_0)
                                                                                                =:=
                                                                                                2 ->
                                                                                      _iter@9_1 =
                                                                                          _next@11_0,
                                                                                      begin
                                                                                          case
                                                                                              _b@10_0
                                                                                                  =<
                                                                                                  _a@6_0
                                                                                              of
                                                                                              true ->
                                                                                                  begin
                                                                                                      '$patina_marker':continue(_iter@9_1,
                                                                                                                                _found@4_0),
                                                                                                      []
                                                                                                  end;
                                                                                              _ ->
                                                                                                  {}
                                                                                          end,
                                                                                          case
                                                                                              _a@6_0
                                                                                                  +
                                                                                                  _b@10_0
                                                                                                  =:=
                                                                                                  _sum@1_0
                                                                                              of
                                                                                              true ->
                                                                                                  _found@4_1 =
                                                                                                      {_a@6_0,
                                                                                                       _b@10_0},
                                                                                                  begin
                                                                                                      '$patina_marker':break({break,
                                                                                                                              0,
                                                                                                                              {},
                                                                                                                              _iter@9_1,
                                                                                                                              _found@4_1}),
                                                                                                      []
                                                                                                  end;
                                                                                              _ ->
                                                                                                  _branch_value@13_0 =
                                                                                                      {},
                                                                                                  _found@4_1 =
                                                                                                      _found@4_0,
                                                                                                  _branch_value@13_0
                                                                                          end
                                                                                      end
                                                                              end
                                                                          end,
                                                                      _$patina_loop(_iter@9_1,
                                                                                    _found@4_1)
                                                              end(_iter@9_0,
                                                                  _found@4_0)
                                                             of
                                                             {break,
                                                              1,
                                                              _loop_value@15_0,
                                                              _iter@9_1,
                                                              _found@4_1} ->
                                                                 _loop_value@15_0;
                                                             {break,
                                                              _target_loop_depth@16_0,
                                                              _loop_value@17_0,
                                                              _iter@9_1,
                                                              _found@4_1} ->
                                                                 '$patina_marker':break({break,
                                                                                         _target_loop_depth@16_0,
                                                                                         _loop_value@17_0,
                                                                                         _iter@5_1,
                                                                                         _found@4_1}),
                                                                 []
                                                         end
                                                 end
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@5_1, _found@4_1)
                     end(_iter@5_0, _found@4_0)
                    of
                    {break, 0, _loop_value@19_0, _iter@5_1, _found@4_1} ->
                        _loop_value@19_0
                end
        end,
        _found@4_1
    end.

count_skipping_rows(_rows@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    begin
        _visited@3_0 = 0,
        case
            'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                      _rows@0_0)
            of
            _iter@4_0 ->
                case fun _$patina_loop(_iter@4_0, _visited@3_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@2_0,
                                                                                              _iter@4_0)
                                         of
                                         nil ->
                                             _branch_value@14_0 = begin
                                                                      '$patina_marker':break({break,
                                                                                              0,
                                                                                              {},
                                                                                              _iter@4_0,
                                                                                              _visited@3_0}),
                                                                      []
                                                                  end,
                                             _iter@4_1 = _iter@4_0,
                                             _visited@3_1 = _visited@3_0,
                                             _branch_value@14_0;
                                         _value@7_0 = {_row@5_0, _next@6_0}
                                             when erlang:is_tuple(_value@7_0)
                                                      andalso
                                                      erlang:tuple_size(_value@7_0)
                                                          =:= 2 ->
                                             _iter@4_1 = _next@6_0,
                                             begin
                                                 case
                                                     'Elixir.Patina.Core.List':'Core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                                                               _row@5_0)
                                                     of
                                                     _iter@8_0 ->
                                                         case fun
                                                                  _$patina_loop(_iter@8_0,
                                                                                _visited@3_0) ->
                                                                      _ = begin
                                                                              case
                                                                                  'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@2_0,
                                                                                                                                       _iter@8_0)
                                                                                  of
                                                                                  nil ->
                                                                                      _branch_value@12_0 =
                                                                                          begin
                                                                                              '$patina_marker':break({break,
                                                                                                                      1,
                                                                                                                      {},
                                                                                                                      _iter@8_0,
                                                                                                                      _visited@3_0}),
                                                                                              []
                                                                                          end,
                                                                                      _iter@8_1 =
                                                                                          _iter@8_0,
                                                                                      _visited@3_1 =
                                                                                          _visited@3_0,
                                                                                      _branch_value@12_0;
                                                                                  _value@11_0 =
                                                                                      {_x@9_0,
                                                                                       _next@10_0}
                                                                                      when
                                                                                        erlang:is_tuple(_value@11_0)
                                                                                            andalso
                                                                                            erlang:tuple_size(_value@11_0)
                                                                                                =:=
                                                                                                2 ->
                                                                                      _iter@8_1 =
                                                                                          _next@10_0,
                                                                                      begin
                                                                                          case
                                                                                              _x@9_0
                                                                                                  <
                                                                                                  0
                                                                                              of
                                                                                              true ->
                                                                                                  begin
                                                                                                      '$patina_marker':break({continue,
                                                                                                                              0,
                                                                                                                              _iter@8_1,
                                                                                                                              _visited@3_0}),
                                                                                                      []
                                                                                                  end;
                                                                                              _ ->
                                                                                                  {}
                                                                                          end,
                                                                                          _visited@3_1 =
                                                                                              _visited@3_0
                                                                                                  +
                                                                                                  1,
                                                                                          {}
                                                                                      end
                                                                              end
                                                                          end,
                                                                      _$patina_loop(_iter@8_1,
                                                                                    _visited@3_1)
                                                              end(_iter@8_0,
                                                                  _visited@3_0)
                                                             of
                                                             {break,
                                                              1,
                                                              _loop_value@13_0,
                                                              _iter@8_1,
                                                              _visited@3_1} ->
                                                                 _loop_value@13_0;
                                                             {continue,
                                                              0,
                                                              _iter@8_1,
                                                              _visited@3_1} ->
                                                                 '$patina_marker':continue(_iter@4_1,
                                                                                           _visited@3_1),
                                                                 []
                                                         end
                                                 end
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@4_1, _visited@3_1)
                     end(_iter@4_0, _visited@3_0)
                    of
                    {break, 0, _loop_value@15_0, _iter@4_1, _visited@3_1} ->
                        _loop_value@15_0
                end
        end,
        _visited@3_1
    end.

countdown(_from@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.ForLoops.Countdown':'Core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    begin
        _seen@4_0 = [],
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                       _from@0_0)
            of
            _iter@5_0 ->
                case fun _$patina_loop(_iter@5_0, _seen@4_0) ->
                             _ = begin
                                     case
                                         'Elixir.Patina.ForLoops.Countdown':'Core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                   _iter@5_0)
                                         of
                                         nil ->
                                             _branch_value@9_0 = begin
                                                                     '$patina_marker':break({break,
                                                                                             0,
                                                                                             {},
                                                                                             _iter@5_0,
                                                                                             _seen@4_0}),
                                                                     []
                                                                 end,
                                             _iter@5_1 = _iter@5_0,
                                             _seen@4_1 = _seen@4_0,
                                             _branch_value@9_0;
                                         _value@8_0 = {_n@6_0, _next@7_0}
                                             when erlang:is_tuple(_value@8_0)
                                                      andalso
                                                      erlang:tuple_size(_value@8_0)
                                                          =:= 2 ->
                                             _iter@5_1 = _next@7_0,
                                             begin
                                                 _seen@4_1 = [_n@6_0
                                                              | _seen@4_0],
                                                 {}
                                             end
                                     end
                                 end,
                             _$patina_loop(_iter@5_1, _seen@4_1)
                     end(_iter@5_0, _seen@4_0)
                    of
                    {break, 0, _loop_value@10_0, _iter@5_1, _seen@4_1} ->
                        _loop_value@10_0
                end
        end,
        _seen@4_1
    end.

bag_count(_items@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.ForLoops.Bag':'Core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    begin count(_witnesses_where_count@2_0, _items@0_0) end.
