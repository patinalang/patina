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
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    _total@3_0 = 0,
    case
        'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                  _xs@0_0)
        of
        _iter@4_0 ->
            {_iter@4_1, _total@3_1} = fun _$patina_loop(_iter@4_0,
                                                        _total@3_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@4_2,
                                                   _total@3_2} ->
                                                      {_iter@4_2, _total@3_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@2_0,
                                                                                                               _iter@4_0)
                                                          of
                                                          nil ->
                                                              _branch_value@8_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@4_0,
                                                                                            _total@3_0),
                                                                      []
                                                                  end,
                                                              _iter@4_1 =
                                                                  _iter@4_0,
                                                              _total@3_1 =
                                                                  _total@3_0,
                                                              _branch_value@8_0;
                                                          _value@7_0 = {_x@5_0,
                                                                        _next@6_0}
                                                              when
                                                                erlang:is_tuple(_value@7_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@7_0)
                                                                        =:= 2 ->
                                                              _iter@4_1 =
                                                                  _next@6_0,
                                                              _total@3_1 =
                                                                  _total@3_0 +
                                                                      _x@5_0,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@4_1,
                                                                    _total@3_1)
                                              end
                                      end(_iter@4_0, _total@3_0),
            {}
    end,
    _total@3_1.

sum_twice(_xs@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _total@5_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                   'Elixir.Patina.Core.List':iter(_xs@0_0))
        of
        _iter@6_0 ->
            {_iter@6_1, _total@5_1} = fun _$patina_loop(_iter@6_0,
                                                        _total@5_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@6_2,
                                                   _total@5_2} ->
                                                      {_iter@6_2, _total@5_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                               _iter@6_0)
                                                          of
                                                          nil ->
                                                              _branch_value@10_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@6_0,
                                                                                            _total@5_0),
                                                                      []
                                                                  end,
                                                              _iter@6_1 =
                                                                  _iter@6_0,
                                                              _total@5_1 =
                                                                  _total@5_0,
                                                              _branch_value@10_0;
                                                          _value@9_0 = {_x@7_0,
                                                                        _next@8_0}
                                                              when
                                                                erlang:is_tuple(_value@9_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@9_0)
                                                                        =:= 2 ->
                                                              _iter@6_1 =
                                                                  _next@8_0,
                                                              _total@5_1 =
                                                                  _total@5_0 +
                                                                      _x@7_0,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@6_1,
                                                                    _total@5_1)
                                              end
                                      end(_iter@6_0, _total@5_0),
            {}
    end,
    case
        'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                  _xs@0_0)
        of
        _iter@11_0 ->
            {_iter@11_1, _total@5_2} = fun _$patina_loop(_iter@11_0,
                                                         _total@5_1) ->
                                               case [] of
                                                   {'$patina_label_1',
                                                    _iter@11_2,
                                                    _total@5_3} ->
                                                       {_iter@11_2, _total@5_3};
                                                   _ ->
                                                       case
                                                           'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                                _iter@11_0)
                                                           of
                                                           nil ->
                                                               _branch_value@15_0 =
                                                                   begin
                                                                       '$patina_marker':jump('$patina_label_1',
                                                                                             _iter@11_0,
                                                                                             _total@5_1),
                                                                       []
                                                                   end,
                                                               _iter@11_1 =
                                                                   _iter@11_0,
                                                               _total@5_2 =
                                                                   _total@5_1,
                                                               _branch_value@15_0;
                                                           _value@14_0 =
                                                               {_x@12_0,
                                                                _next@13_0}
                                                               when
                                                                 erlang:is_tuple(_value@14_0)
                                                                     andalso
                                                                     erlang:tuple_size(_value@14_0)
                                                                         =:=
                                                                         2 ->
                                                               _iter@11_1 =
                                                                   _next@13_0,
                                                               _total@5_2 =
                                                                   _total@5_1 +
                                                                       _x@12_0 *
                                                                           100,
                                                               {}
                                                       end,
                                                       _$patina_loop(_iter@11_1,
                                                                     _total@5_2)
                                               end
                                       end(_iter@11_0, _total@5_1),
            {}
    end,
    _total@5_2.

sum_range(_start@0_0, _end@1_0) ->
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.Range':'core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@3_0 =
        {_witness_Iterator@2_0},
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@3_0),
    _total@5_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                   'Elixir.Patina.Core.Range':'$patina_constructor__Range'(_start@0_0,
                                                                                                                           _end@1_0))
        of
        _iter@6_0 ->
            {_iter@6_1, _total@5_1} = fun _$patina_loop(_iter@6_0,
                                                        _total@5_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@6_2,
                                                   _total@5_2} ->
                                                      {_iter@6_2, _total@5_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.Range':'core::Iterator::next<int>'(_witness_Iterator@2_0,
                                                                                                                 _iter@6_0)
                                                          of
                                                          nil ->
                                                              _branch_value@10_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@6_0,
                                                                                            _total@5_0),
                                                                      []
                                                                  end,
                                                              _iter@6_1 =
                                                                  _iter@6_0,
                                                              _total@5_1 =
                                                                  _total@5_0,
                                                              _branch_value@10_0;
                                                          _value@9_0 = {_n@7_0,
                                                                        _next@8_0}
                                                              when
                                                                erlang:is_tuple(_value@9_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@9_0)
                                                                        =:= 2 ->
                                                              _iter@6_1 =
                                                                  _next@8_0,
                                                              _total@5_1 =
                                                                  _total@5_0 +
                                                                      _n@7_0,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@6_1,
                                                                    _total@5_1)
                                              end
                                      end(_iter@6_0, _total@5_0),
            {}
    end,
    _total@5_1.

sum_range_inclusive(_start@0_0, _end@1_0) ->
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.RangeInclusive':'core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@3_0 =
        {_witness_Iterator@2_0},
    _witness_IntoIterator@4_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@3_0),
    _total@5_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'core::IntoIterator::into_iter'(_witness_IntoIterator@4_0,
                                                                   'Elixir.Patina.Core.RangeInclusive':new(_start@0_0,
                                                                                                           _end@1_0))
        of
        _iter@6_0 ->
            {_iter@6_1, _total@5_1} = fun _$patina_loop(_iter@6_0,
                                                        _total@5_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@6_2,
                                                   _total@5_2} ->
                                                      {_iter@6_2, _total@5_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.RangeInclusive':'core::Iterator::next<int>'(_witness_Iterator@2_0,
                                                                                                                          _iter@6_0)
                                                          of
                                                          nil ->
                                                              _branch_value@10_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@6_0,
                                                                                            _total@5_0),
                                                                      []
                                                                  end,
                                                              _iter@6_1 =
                                                                  _iter@6_0,
                                                              _total@5_1 =
                                                                  _total@5_0,
                                                              _branch_value@10_0;
                                                          _value@9_0 = {_n@7_0,
                                                                        _next@8_0}
                                                              when
                                                                erlang:is_tuple(_value@9_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@9_0)
                                                                        =:= 2 ->
                                                              _iter@6_1 =
                                                                  _next@8_0,
                                                              _total@5_1 =
                                                                  _total@5_0 +
                                                                      _n@7_0,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@6_1,
                                                                    _total@5_1)
                                              end
                                      end(_iter@6_0, _total@5_0),
            {}
    end,
    _total@5_1.

weighted_map_sum(_map@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.Map':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.MapIter':'core::Iterator::$patina_witness'(),
    _total@3_0 = 0,
    case
        'Elixir.Patina.Core.Map':'core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                 _map@0_0)
        of
        _iter@4_0 ->
            {_iter@4_1, _total@3_1} = fun _$patina_loop(_iter@4_0,
                                                        _total@3_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@4_2,
                                                   _total@3_2} ->
                                                      {_iter@4_2, _total@3_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.MapIter':'core::Iterator::next'(_witness_Iterator@2_0,
                                                                                                              _iter@4_0)
                                                          of
                                                          nil ->
                                                              _branch_value@9_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@4_0,
                                                                                            _total@3_0),
                                                                      []
                                                                  end,
                                                              _iter@4_1 =
                                                                  _iter@4_0,
                                                              _total@3_1 =
                                                                  _total@3_0,
                                                              _branch_value@9_0;
                                                          _value@8_0 =
                                                              {{_key@5_0,
                                                                _value@6_0},
                                                               _next@7_0}
                                                              when
                                                                erlang:is_tuple(_value@8_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@8_0)
                                                                        =:= 2 ->
                                                              _iter@4_1 =
                                                                  _next@7_0,
                                                              _total@3_1 =
                                                                  _total@3_0 +
                                                                      _key@5_0 *
                                                                          _value@6_0,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@4_1,
                                                                    _total@3_1)
                                              end
                                      end(_iter@4_0, _total@3_0),
            {}
    end,
    _total@3_1.

map_entry_count(_map@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.MapIter':'core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _count@4_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                   'Elixir.Patina.Core.Map':iter(_map@0_0))
        of
        _iter@5_0 ->
            {_iter@5_1, _count@4_1} = fun _$patina_loop(_iter@5_0,
                                                        _count@4_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@5_2,
                                                   _count@4_2} ->
                                                      {_iter@5_2, _count@4_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.MapIter':'core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                              _iter@5_0)
                                                          of
                                                          nil ->
                                                              _branch_value@8_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@5_0,
                                                                                            _count@4_0),
                                                                      []
                                                                  end,
                                                              _iter@5_1 =
                                                                  _iter@5_0,
                                                              _count@4_1 =
                                                                  _count@4_0,
                                                              _branch_value@8_0;
                                                          _value@7_0 = {_,
                                                                        _next@6_0}
                                                              when
                                                                erlang:is_tuple(_value@7_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@7_0)
                                                                        =:= 2 ->
                                                              _iter@5_1 =
                                                                  _next@6_0,
                                                              _count@4_1 =
                                                                  _count@4_0 +
                                                                      1,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@5_1,
                                                                    _count@4_1)
                                              end
                                      end(_iter@5_0, _count@4_0),
            {}
    end,
    _count@4_1.

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
    _n@10_0 = 0,
    case
        _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                       _items@1_0)
        of
        _iter@11_0 ->
            {_iter@11_1, _n@10_1} = fun _$patina_loop(_iter@11_0,
                                                      _n@10_0) ->
                                            case [] of
                                                {'$patina_label_0',
                                                 _iter@11_2,
                                                 _n@10_2} ->
                                                    {_iter@11_2, _n@10_2};
                                                _ ->
                                                    case
                                                        _fn_Iterator_next@9_0(_witness_Iterator@7_0,
                                                                              _iter@11_0)
                                                        of
                                                        nil ->
                                                            _branch_value@14_0 =
                                                                begin
                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                          _iter@11_0,
                                                                                          _n@10_0),
                                                                    []
                                                                end,
                                                            _iter@11_1 =
                                                                _iter@11_0,
                                                            _n@10_1 = _n@10_0,
                                                            _branch_value@14_0;
                                                        {some,
                                                         {_, _next@12_0}} ->
                                                            _iter@11_1 =
                                                                _next@12_0,
                                                            _n@10_1 = _n@10_0 +
                                                                          1,
                                                            {};
                                                        _value@13_0 = {_,
                                                                       _next@12_0}
                                                            when not
                                                                     ((erlang:is_tuple(_value@13_0)
                                                                           andalso
                                                                           erlang:tuple_size(_value@13_0)
                                                                               =:=
                                                                               2)
                                                                          andalso
                                                                          erlang:element(1,
                                                                                         _value@13_0)
                                                                              =:=
                                                                              some) ->
                                                            _iter@11_1 =
                                                                _next@12_0,
                                                            _n@10_1 = _n@10_0 +
                                                                          1,
                                                            {}
                                                    end,
                                                    _$patina_loop(_iter@11_1,
                                                                  _n@10_1)
                                            end
                                    end(_iter@11_0, _n@10_0),
            {}
    end,
    _n@10_1.

count_list(_xs@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    count(_witnesses_where_count@2_0, _xs@0_0).

count_map(_map@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.Map':'core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    count(_witnesses_where_count@2_0, _map@0_0).

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
    _total@10_0 = 0,
    case
        _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                       _items@1_0)
        of
        _iter@11_0 ->
            {_iter@11_1, _total@10_1} = fun
                                            _$patina_loop(_iter@11_0,
                                                          _total@10_0) ->
                                                case [] of
                                                    {'$patina_label_0',
                                                     _iter@11_2,
                                                     _total@10_2} ->
                                                        {_iter@11_2,
                                                         _total@10_2};
                                                    _ ->
                                                        case
                                                            _fn_Iterator_next@9_0(_witness_Iterator@7_0,
                                                                                  _iter@11_0)
                                                            of
                                                            nil ->
                                                                _branch_value@15_0 =
                                                                    begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _iter@11_0,
                                                                                              _total@10_0),
                                                                        []
                                                                    end,
                                                                _iter@11_1 =
                                                                    _iter@11_0,
                                                                _total@10_1 =
                                                                    _total@10_0,
                                                                _branch_value@15_0;
                                                            _value@14_0 =
                                                                {_item@12_0,
                                                                 _next@13_0}
                                                                when
                                                                  erlang:is_tuple(_value@14_0)
                                                                      andalso
                                                                      erlang:tuple_size(_value@14_0)
                                                                          =:=
                                                                          2 ->
                                                                _iter@11_1 =
                                                                    _next@13_0,
                                                                _total@10_1 =
                                                                    _total@10_0
                                                                        +
                                                                        _item@12_0,
                                                                {}
                                                        end,
                                                        _$patina_loop(_iter@11_1,
                                                                      _total@10_1)
                                                end
                                        end(_iter@11_0, _total@10_0),
            {}
    end,
    _total@10_1.

total_range(_end@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.Range':'core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _witnesses_where_total@4_0 =
        {_witness_IntoIterator@3_0},
    total(_witnesses_where_total@4_0,
          'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                  _end@0_0)).

total_list(_xs@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witnesses_where_total@2_0 =
        {_witness_IntoIterator@1_0},
    total(_witnesses_where_total@2_0, _xs@0_0).

skip_odd_until(_xs@0_0, _limit@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    _total@4_0 = 0,
    case
        'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                  _xs@0_0)
        of
        _iter@5_0 ->
            {_iter@5_1, _total@4_1} = fun _$patina_loop(_iter@5_0,
                                                        _total@4_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@5_2,
                                                   _total@4_2} ->
                                                      {_iter@5_2, _total@4_2};
                                                  _ ->
                                                      case [] of
                                                          {'$patina_label_1',
                                                           _iter@5_1,
                                                           _total@4_1} ->
                                                              {};
                                                          _ ->
                                                              case
                                                                  'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@3_0,
                                                                                                                       _iter@5_0)
                                                                  of
                                                                  nil ->
                                                                      _branch_value@9_0 =
                                                                          begin
                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                    _iter@5_0,
                                                                                                    _total@4_0),
                                                                              []
                                                                          end,
                                                                      _iter@5_1 =
                                                                          _iter@5_0,
                                                                      _total@4_1 =
                                                                          _total@4_0,
                                                                      _branch_value@9_0;
                                                                  _value@8_0 =
                                                                      {_x@6_0,
                                                                       _next@7_0}
                                                                      when
                                                                        erlang:is_tuple(_value@8_0)
                                                                            andalso
                                                                            erlang:tuple_size(_value@8_0)
                                                                                =:=
                                                                                2 ->
                                                                      _iter@5_1 =
                                                                          _next@7_0,
                                                                      case
                                                                          _x@6_0
                                                                              rem
                                                                              2
                                                                              =:=
                                                                              1
                                                                          of
                                                                          true ->
                                                                              '$patina_marker':jump('$patina_label_1',
                                                                                                    _iter@5_1,
                                                                                                    _total@4_0),
                                                                              [];
                                                                          _ ->
                                                                              {}
                                                                      end,
                                                                      case
                                                                          _x@6_0
                                                                              >
                                                                              _limit@1_0
                                                                          of
                                                                          true ->
                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                    _iter@5_1,
                                                                                                    _total@4_0),
                                                                              [];
                                                                          _ ->
                                                                              {}
                                                                      end,
                                                                      _total@4_1 =
                                                                          _total@4_0
                                                                              +
                                                                              _x@6_0,
                                                                      {}
                                                              end
                                                      end,
                                                      _$patina_loop(_iter@5_1,
                                                                    _total@4_1)
                                              end
                                      end(_iter@5_0, _total@4_0),
            {}
    end,
    _total@4_1.

find_index(_xs@0_0, _target@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    case [] of
        {'$patina_label_1', _jump_value@10_0} ->
            _jump_value@10_0;
        _ ->
            _index@4_0 = 0,
            case
                'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                          _xs@0_0)
                of
                _iter@5_0 ->
                    {_iter@5_1, _index@4_1} = fun _$patina_loop(_iter@5_0,
                                                                _index@4_0) ->
                                                      case [] of
                                                          {'$patina_label_0',
                                                           _iter@5_2,
                                                           _index@4_2} ->
                                                              {_iter@5_2,
                                                               _index@4_2};
                                                          _ ->
                                                              case
                                                                  'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@3_0,
                                                                                                                       _iter@5_0)
                                                                  of
                                                                  nil ->
                                                                      _branch_value@9_0 =
                                                                          begin
                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                    _iter@5_0,
                                                                                                    _index@4_0),
                                                                              []
                                                                          end,
                                                                      _iter@5_1 =
                                                                          _iter@5_0,
                                                                      _index@4_1 =
                                                                          _index@4_0,
                                                                      _branch_value@9_0;
                                                                  _value@8_0 =
                                                                      {_x@6_0,
                                                                       _next@7_0}
                                                                      when
                                                                        erlang:is_tuple(_value@8_0)
                                                                            andalso
                                                                            erlang:tuple_size(_value@8_0)
                                                                                =:=
                                                                                2 ->
                                                                      _iter@5_1 =
                                                                          _next@7_0,
                                                                      case
                                                                          _x@6_0
                                                                              =:=
                                                                              _target@1_0
                                                                          of
                                                                          true ->
                                                                              '$patina_marker':jump('$patina_label_1',
                                                                                                    _index@4_0),
                                                                              [];
                                                                          _ ->
                                                                              {}
                                                                      end,
                                                                      _index@4_1 =
                                                                          _index@4_0
                                                                              +
                                                                              1,
                                                                      {}
                                                              end,
                                                              _$patina_loop(_iter@5_1,
                                                                            _index@4_1)
                                                      end
                                              end(_iter@5_0, _index@4_0),
                    {}
            end,
            -1
    end.

first_pair_with_sum(_xs@0_0, _sum@1_0) ->
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@3_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    _found@4_0 = {0, 0},
    case
        'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                  _xs@0_0)
        of
        _iter@5_0 ->
            {_iter@5_1, _found@4_1} = fun _$patina_loop(_iter@5_0,
                                                        _found@4_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@5_2,
                                                   _found@4_2} ->
                                                      {_iter@5_2, _found@4_2};
                                                  _ ->
                                                      case
                                                          'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@3_0,
                                                                                                               _iter@5_0)
                                                          of
                                                          nil ->
                                                              _branch_value@15_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@5_0,
                                                                                            _found@4_0),
                                                                      []
                                                                  end,
                                                              _iter@5_1 =
                                                                  _iter@5_0,
                                                              _found@4_1 =
                                                                  _found@4_0,
                                                              _branch_value@15_0;
                                                          _value@8_0 = {_a@6_0,
                                                                        _next@7_0}
                                                              when
                                                                erlang:is_tuple(_value@8_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@8_0)
                                                                        =:= 2 ->
                                                              _iter@5_1 =
                                                                  _next@7_0,
                                                              case
                                                                  'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                                                                            _xs@0_0)
                                                                  of
                                                                  _iter@9_0 ->
                                                                      {_iter@9_1,
                                                                       _found@4_1} =
                                                                          fun
                                                                              _$patina_loop(_iter@9_0,
                                                                                            _found@4_0) ->
                                                                                  case
                                                                                      []
                                                                                      of
                                                                                      {'$patina_label_1',
                                                                                       _iter@9_2,
                                                                                       _found@4_2} ->
                                                                                          {_iter@9_2,
                                                                                           _found@4_2};
                                                                                      _ ->
                                                                                          case
                                                                                              []
                                                                                              of
                                                                                              {'$patina_label_2',
                                                                                               _iter@9_1,
                                                                                               _found@4_1} ->
                                                                                                  {};
                                                                                              _ ->
                                                                                                  case
                                                                                                      'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@3_0,
                                                                                                                                                           _iter@9_0)
                                                                                                      of
                                                                                                      nil ->
                                                                                                          _branch_value@14_0 =
                                                                                                              begin
                                                                                                                  '$patina_marker':jump('$patina_label_1',
                                                                                                                                        _iter@9_0,
                                                                                                                                        _found@4_0),
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
                                                                                                          case
                                                                                                              _b@10_0
                                                                                                                  =<
                                                                                                                  _a@6_0
                                                                                                              of
                                                                                                              true ->
                                                                                                                  '$patina_marker':jump('$patina_label_2',
                                                                                                                                        _iter@9_1,
                                                                                                                                        _found@4_0),
                                                                                                                  [];
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
                                                                                                                  '$patina_marker':jump('$patina_label_0',
                                                                                                                                        _iter@5_1,
                                                                                                                                        _found@4_1),
                                                                                                                  [];
                                                                                                              _ ->
                                                                                                                  _branch_value@13_0 =
                                                                                                                      {},
                                                                                                                  _found@4_1 =
                                                                                                                      _found@4_0,
                                                                                                                  _branch_value@13_0
                                                                                                          end
                                                                                                  end
                                                                                          end,
                                                                                          _$patina_loop(_iter@9_1,
                                                                                                        _found@4_1)
                                                                                  end
                                                                          end(_iter@9_0,
                                                                              _found@4_0),
                                                                      {}
                                                              end
                                                      end,
                                                      _$patina_loop(_iter@5_1,
                                                                    _found@4_1)
                                              end
                                      end(_iter@5_0, _found@4_0),
            {}
    end,
    _found@4_1.

count_skipping_rows(_rows@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.Core.List':'core::IntoIterator::$patina_witness'(),
    _witness_Iterator@2_0 =
        'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_witness'(),
    _visited@3_0 = 0,
    case
        'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                  _rows@0_0)
        of
        _iter@4_0 ->
            {_iter@4_1, _visited@3_1} = fun _$patina_loop(_iter@4_0,
                                                          _visited@3_0) ->
                                                case [] of
                                                    {'$patina_label_0',
                                                     _iter@4_2,
                                                     _visited@3_2} ->
                                                        {_iter@4_2,
                                                         _visited@3_2};
                                                    _ ->
                                                        case [] of
                                                            {'$patina_label_2',
                                                             _iter@4_1,
                                                             _visited@3_1} ->
                                                                {};
                                                            _ ->
                                                                case
                                                                    'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@2_0,
                                                                                                                         _iter@4_0)
                                                                    of
                                                                    nil ->
                                                                        _branch_value@13_0 =
                                                                            begin
                                                                                '$patina_marker':jump('$patina_label_0',
                                                                                                      _iter@4_0,
                                                                                                      _visited@3_0),
                                                                                []
                                                                            end,
                                                                        _iter@4_1 =
                                                                            _iter@4_0,
                                                                        _visited@3_1 =
                                                                            _visited@3_0,
                                                                        _branch_value@13_0;
                                                                    _value@7_0 =
                                                                        {_row@5_0,
                                                                         _next@6_0}
                                                                        when
                                                                          erlang:is_tuple(_value@7_0)
                                                                              andalso
                                                                              erlang:tuple_size(_value@7_0)
                                                                                  =:=
                                                                                  2 ->
                                                                        _iter@4_1 =
                                                                            _next@6_0,
                                                                        case
                                                                            'Elixir.Patina.Core.List':'core::IntoIterator::into_iter'(_witness_IntoIterator@1_0,
                                                                                                                                      _row@5_0)
                                                                            of
                                                                            _iter@8_0 ->
                                                                                {_iter@8_1,
                                                                                 _visited@3_1} =
                                                                                    fun
                                                                                        _$patina_loop(_iter@8_0,
                                                                                                      _visited@3_0) ->
                                                                                            case
                                                                                                []
                                                                                                of
                                                                                                {'$patina_label_1',
                                                                                                 _iter@8_2,
                                                                                                 _visited@3_2} ->
                                                                                                    {_iter@8_2,
                                                                                                     _visited@3_2};
                                                                                                _ ->
                                                                                                    case
                                                                                                        'Elixir.Patina.Core.ListIter':'core::Iterator::next'(_witness_Iterator@2_0,
                                                                                                                                                             _iter@8_0)
                                                                                                        of
                                                                                                        nil ->
                                                                                                            _branch_value@12_0 =
                                                                                                                begin
                                                                                                                    '$patina_marker':jump('$patina_label_1',
                                                                                                                                          _iter@8_0,
                                                                                                                                          _visited@3_0),
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
                                                                                                            case
                                                                                                                _x@9_0
                                                                                                                    <
                                                                                                                    0
                                                                                                                of
                                                                                                                true ->
                                                                                                                    '$patina_marker':jump('$patina_label_2',
                                                                                                                                          _iter@4_1,
                                                                                                                                          _visited@3_0),
                                                                                                                    [];
                                                                                                                _ ->
                                                                                                                    {}
                                                                                                            end,
                                                                                                            _visited@3_1 =
                                                                                                                _visited@3_0
                                                                                                                    +
                                                                                                                    1,
                                                                                                            {}
                                                                                                    end,
                                                                                                    _$patina_loop(_iter@8_1,
                                                                                                                  _visited@3_1)
                                                                                            end
                                                                                    end(_iter@8_0,
                                                                                        _visited@3_0),
                                                                                {}
                                                                        end
                                                                end
                                                        end,
                                                        _$patina_loop(_iter@4_1,
                                                                      _visited@3_1)
                                                end
                                        end(_iter@4_0, _visited@3_0),
            {}
    end,
    _visited@3_1.

countdown(_from@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.ForLoops.Countdown':'core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    _seen@4_0 = [],
    case
        'Elixir.Patina.Core.$impl':'core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                   _from@0_0)
        of
        _iter@5_0 ->
            {_iter@5_1, _seen@4_1} = fun _$patina_loop(_iter@5_0,
                                                       _seen@4_0) ->
                                             case [] of
                                                 {'$patina_label_0',
                                                  _iter@5_2,
                                                  _seen@4_2} ->
                                                     {_iter@5_2, _seen@4_2};
                                                 _ ->
                                                     case
                                                         'Elixir.Patina.ForLoops.Countdown':'core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                                   _iter@5_0)
                                                         of
                                                         nil ->
                                                             _branch_value@9_0 =
                                                                 begin
                                                                     '$patina_marker':jump('$patina_label_0',
                                                                                           _iter@5_0,
                                                                                           _seen@4_0),
                                                                     []
                                                                 end,
                                                             _iter@5_1 =
                                                                 _iter@5_0,
                                                             _seen@4_1 =
                                                                 _seen@4_0,
                                                             _branch_value@9_0;
                                                         _value@8_0 = {_n@6_0,
                                                                       _next@7_0}
                                                             when
                                                               erlang:is_tuple(_value@8_0)
                                                                   andalso
                                                                   erlang:tuple_size(_value@8_0)
                                                                       =:= 2 ->
                                                             _iter@5_1 =
                                                                 _next@7_0,
                                                             _seen@4_1 = [_n@6_0
                                                                          | _seen@4_0],
                                                             {}
                                                     end,
                                                     _$patina_loop(_iter@5_1,
                                                                   _seen@4_1)
                                             end
                                     end(_iter@5_0, _seen@4_0),
            {}
    end,
    _seen@4_1.

bag_count(_items@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.ForLoops.Bag':'core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    count(_witnesses_where_count@2_0, _items@0_0).
