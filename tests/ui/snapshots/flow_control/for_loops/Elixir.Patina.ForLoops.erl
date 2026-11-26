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
    _total@1_0 = 0,
    _iter@4_0 = begin
                    _self@2_0 = _xs@0_0, _self@3_0 = _self@2_0, _self@3_0
                end,
    {_iter@4_1, _total@1_1} = fun _$patina_loop(_iter@4_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@4_2,
                                           _total@1_2} ->
                                              {_iter@4_2, _total@1_2};
                                          _ ->
                                              case begin
                                                       _self@5_0 = _iter@4_0,
                                                       case _self@5_0 of
                                                           [_head@7_0
                                                            | _tail@6_0 = _] ->
                                                               _0@9_0 = begin
                                                                            _element@8_0 =
                                                                                _tail@6_0,
                                                                            {_head@7_0,
                                                                             _element@8_0}
                                                                        end,
                                                               _0@9_0;
                                                           [] -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@13_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@4_0,
                                                                                                     _total@1_0),
                                                                               []
                                                                           end,
                                                      _iter@4_1 = _iter@4_0,
                                                      _total@1_1 = _total@1_0,
                                                      _branch_value@13_0;
                                                  _value@12_0 = {_x@10_0,
                                                                 _next@11_0}
                                                      when
                                                        erlang:is_tuple(_value@12_0)
                                                            andalso
                                                            erlang:tuple_size(_value@12_0)
                                                                =:= 2 ->
                                                      _iter@4_1 = _next@11_0,
                                                      _total@1_1 = _total@1_0 +
                                                                       _x@10_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@4_1,
                                                            _total@1_1)
                                      end
                              end(_iter@4_0, _total@1_0),
    _total@1_1.

sum_twice(_xs@0_0) ->
    _total@1_0 = 0,
    _iter@4_0 = begin
                    _self@3_0 = begin _self@2_0 = _xs@0_0, _self@2_0 end,
                    _self@3_0
                end,
    {_iter@4_1, _total@1_1} = fun _$patina_loop(_iter@4_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@4_2,
                                           _total@1_2} ->
                                              {_iter@4_2, _total@1_2};
                                          _ ->
                                              case begin
                                                       _self@5_0 = _iter@4_0,
                                                       case _self@5_0 of
                                                           [_head@7_0
                                                            | _tail@6_0 = _] ->
                                                               _0@9_0 = begin
                                                                            _element@8_0 =
                                                                                _tail@6_0,
                                                                            {_head@7_0,
                                                                             _element@8_0}
                                                                        end,
                                                               _0@9_0;
                                                           [] -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@13_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@4_0,
                                                                                                     _total@1_0),
                                                                               []
                                                                           end,
                                                      _iter@4_1 = _iter@4_0,
                                                      _total@1_1 = _total@1_0,
                                                      _branch_value@13_0;
                                                  _value@12_0 = {_x@10_0,
                                                                 _next@11_0}
                                                      when
                                                        erlang:is_tuple(_value@12_0)
                                                            andalso
                                                            erlang:tuple_size(_value@12_0)
                                                                =:= 2 ->
                                                      _iter@4_1 = _next@11_0,
                                                      _total@1_1 = _total@1_0 +
                                                                       _x@10_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@4_1,
                                                            _total@1_1)
                                      end
                              end(_iter@4_0, _total@1_0),
    _iter@16_0 = begin
                     _self@14_0 = _xs@0_0,
                     _self@15_0 = _self@14_0,
                     _self@15_0
                 end,
    {_iter@16_1, _total@1_2} = fun _$patina_loop(_iter@16_0,
                                                 _total@1_1) ->
                                       case [] of
                                           {'$patina_label_1',
                                            _iter@16_2,
                                            _total@1_3} ->
                                               {_iter@16_2, _total@1_3};
                                           _ ->
                                               case begin
                                                        _self@17_0 = _iter@16_0,
                                                        case _self@17_0 of
                                                            [_head@19_0
                                                             | _tail@18_0 =
                                                                   _] ->
                                                                _0@21_0 = begin
                                                                              _element@20_0 =
                                                                                  _tail@18_0,
                                                                              {_head@19_0,
                                                                               _element@20_0}
                                                                          end,
                                                                _0@21_0;
                                                            [] -> nil
                                                        end
                                                    end
                                                   of
                                                   nil ->
                                                       _branch_value@25_0 =
                                                           begin
                                                               '$patina_marker':jump('$patina_label_1',
                                                                                     _iter@16_0,
                                                                                     _total@1_1),
                                                               []
                                                           end,
                                                       _iter@16_1 = _iter@16_0,
                                                       _total@1_2 = _total@1_1,
                                                       _branch_value@25_0;
                                                   _value@24_0 = {_x@22_0,
                                                                  _next@23_0}
                                                       when
                                                         erlang:is_tuple(_value@24_0)
                                                             andalso
                                                             erlang:tuple_size(_value@24_0)
                                                                 =:= 2 ->
                                                       _iter@16_1 = _next@23_0,
                                                       _total@1_2 = _total@1_1 +
                                                                        _x@22_0
                                                                            *
                                                                            100,
                                                       {}
                                               end,
                                               _$patina_loop(_iter@16_1,
                                                             _total@1_2)
                                       end
                               end(_iter@16_0, _total@1_1),
    _total@1_2.

sum_range(_start@0_0, _end@1_0) ->
    _total@2_0 = 0,
    _iter@4_0 = begin
                    _self@3_0 =
                        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(_start@0_0,
                                                                                _end@1_0),
                    _self@3_0
                end,
    {_iter@4_1, _total@2_1} = fun _$patina_loop(_iter@4_0,
                                                _total@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@4_2,
                                           _total@2_2} ->
                                              {_iter@4_2, _total@2_2};
                                          _ ->
                                              case begin
                                                       _self@5_0 = _iter@4_0,
                                                       case
                                                           _self@5_0#'Elixir.Patina.Core.Range':'Range'.start
                                                               <
                                                               _self@5_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                                           of
                                                           true ->
                                                               _0@9_0 = begin
                                                                            _element@6_0 =
                                                                                _self@5_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                                            _element@8_0 =
                                                                                begin
                                                                                    _start@7_0 =
                                                                                        _self@5_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                                            +
                                                                                            1,
                                                                                    _self@5_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                                     =
                                                                                                                                     _start@7_0}
                                                                                end,
                                                                            {_element@6_0,
                                                                             _element@8_0}
                                                                        end,
                                                               _0@9_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@13_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@4_0,
                                                                                                     _total@2_0),
                                                                               []
                                                                           end,
                                                      _iter@4_1 = _iter@4_0,
                                                      _total@2_1 = _total@2_0,
                                                      _branch_value@13_0;
                                                  _value@12_0 = {_n@10_0,
                                                                 _next@11_0}
                                                      when
                                                        erlang:is_tuple(_value@12_0)
                                                            andalso
                                                            erlang:tuple_size(_value@12_0)
                                                                =:= 2 ->
                                                      _iter@4_1 = _next@11_0,
                                                      _total@2_1 = _total@2_0 +
                                                                       _n@10_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@4_1,
                                                            _total@2_1)
                                      end
                              end(_iter@4_0, _total@2_0),
    _total@2_1.

sum_range_inclusive(_start@0_0, _end@1_0) ->
    _total@2_0 = 0,
    _iter@4_0 = begin
                    _self@3_0 =
                        'Elixir.Patina.Core.RangeInclusive':new(_start@0_0,
                                                                _end@1_0),
                    _self@3_0
                end,
    {_iter@4_1, _total@2_1} = fun _$patina_loop(_iter@4_0,
                                                _total@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@4_2,
                                           _total@2_2} ->
                                              {_iter@4_2, _total@2_2};
                                          _ ->
                                              case begin
                                                       _self@5_0 = _iter@4_0,
                                                       case
                                                           _self@5_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.start
                                                               =<
                                                               _self@5_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.'end'
                                                           of
                                                           true ->
                                                               _0@9_0 = begin
                                                                            _element@6_0 =
                                                                                _self@5_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.start,
                                                                            _element@8_0 =
                                                                                begin
                                                                                    _start@7_0 =
                                                                                        _self@5_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.start
                                                                                            +
                                                                                            1,
                                                                                    _self@5_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'{start
                                                                                                                                                       =
                                                                                                                                                       _start@7_0}
                                                                                end,
                                                                            {_element@6_0,
                                                                             _element@8_0}
                                                                        end,
                                                               _0@9_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@13_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@4_0,
                                                                                                     _total@2_0),
                                                                               []
                                                                           end,
                                                      _iter@4_1 = _iter@4_0,
                                                      _total@2_1 = _total@2_0,
                                                      _branch_value@13_0;
                                                  _value@12_0 = {_n@10_0,
                                                                 _next@11_0}
                                                      when
                                                        erlang:is_tuple(_value@12_0)
                                                            andalso
                                                            erlang:tuple_size(_value@12_0)
                                                                =:= 2 ->
                                                      _iter@4_1 = _next@11_0,
                                                      _total@2_1 = _total@2_0 +
                                                                       _n@10_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@4_1,
                                                            _total@2_1)
                                      end
                              end(_iter@4_0, _total@2_0),
    _total@2_1.

weighted_map_sum(_map@0_0) ->
    _total@1_0 = 0,
    _iter@5_0 = begin
                    _self@2_0 = _map@0_0,
                    _self@3_0 = _self@2_0,
                    _0@4_0 = maps:iterator(_self@3_0),
                    _0@4_0
                end,
    {_iter@5_1, _total@1_1} = fun _$patina_loop(_iter@5_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@5_2,
                                           _total@1_2} ->
                                              {_iter@5_2, _total@1_2};
                                          _ ->
                                              case begin
                                                       _self@6_0 = _iter@5_0,
                                                       case maps:next(_self@6_0)
                                                           of
                                                           {_key@7_0,
                                                            _value@8_0,
                                                            _rest@9_0} ->
                                                               _entry@12_0 =
                                                                   begin
                                                                       _element@10_0 =
                                                                           _key@7_0,
                                                                       _element@11_0 =
                                                                           _value@8_0,
                                                                       {_element@10_0,
                                                                        _element@11_0}
                                                                   end,
                                                               _0@14_0 = begin
                                                                             _element@13_0 =
                                                                                 _rest@9_0,
                                                                             {_entry@12_0,
                                                                              _element@13_0}
                                                                         end,
                                                               _0@14_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@19_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@5_0,
                                                                                                     _total@1_0),
                                                                               []
                                                                           end,
                                                      _iter@5_1 = _iter@5_0,
                                                      _total@1_1 = _total@1_0,
                                                      _branch_value@19_0;
                                                  _value@18_0 = {{_key@15_0,
                                                                  _value@16_0},
                                                                 _next@17_0}
                                                      when
                                                        erlang:is_tuple(_value@18_0)
                                                            andalso
                                                            erlang:tuple_size(_value@18_0)
                                                                =:= 2 ->
                                                      _iter@5_1 = _next@17_0,
                                                      _total@1_1 = _total@1_0 +
                                                                       _key@15_0
                                                                           *
                                                                           _value@16_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@5_1,
                                                            _total@1_1)
                                      end
                              end(_iter@5_0, _total@1_0),
    _total@1_1.

map_entry_count(_map@0_0) ->
    _count@1_0 = 0,
    _iter@5_0 = begin
                    _self@4_0 = begin
                                    _self@2_0 = _map@0_0,
                                    _0@3_0 = maps:iterator(_self@2_0),
                                    _0@3_0
                                end,
                    _self@4_0
                end,
    {_iter@5_1, _count@1_1} = fun _$patina_loop(_iter@5_0,
                                                _count@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@5_2,
                                           _count@1_2} ->
                                              {_iter@5_2, _count@1_2};
                                          _ ->
                                              case begin
                                                       _self@6_0 = _iter@5_0,
                                                       case maps:next(_self@6_0)
                                                           of
                                                           {_key@7_0,
                                                            _value@8_0,
                                                            _rest@9_0} ->
                                                               _entry@12_0 =
                                                                   begin
                                                                       _element@10_0 =
                                                                           _key@7_0,
                                                                       _element@11_0 =
                                                                           _value@8_0,
                                                                       {_element@10_0,
                                                                        _element@11_0}
                                                                   end,
                                                               _0@14_0 = begin
                                                                             _element@13_0 =
                                                                                 _rest@9_0,
                                                                             {_entry@12_0,
                                                                              _element@13_0}
                                                                         end,
                                                               _0@14_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@17_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@5_0,
                                                                                                     _count@1_0),
                                                                               []
                                                                           end,
                                                      _iter@5_1 = _iter@5_0,
                                                      _count@1_1 = _count@1_0,
                                                      _branch_value@17_0;
                                                  _value@16_0 = {_, _next@15_0}
                                                      when
                                                        erlang:is_tuple(_value@16_0)
                                                            andalso
                                                            erlang:tuple_size(_value@16_0)
                                                                =:= 2 ->
                                                      _iter@5_1 = _next@15_0,
                                                      _count@1_1 = _count@1_0 +
                                                                       1,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@5_1,
                                                            _count@1_1)
                                      end
                              end(_iter@5_0, _count@1_0),
    _count@1_1.

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
    _iter@11_0 =
        _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                       _items@1_0),
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
                                                    _branch_value@14_0 = begin
                                                                             '$patina_marker':jump('$patina_label_0',
                                                                                                   _iter@11_0,
                                                                                                   _n@10_0),
                                                                             []
                                                                         end,
                                                    _iter@11_1 = _iter@11_0,
                                                    _n@10_1 = _n@10_0,
                                                    _branch_value@14_0;
                                                {some, {_, _next@12_0}} ->
                                                    _iter@11_1 = _next@12_0,
                                                    _n@10_1 = _n@10_0 + 1,
                                                    {};
                                                _value@13_0 = {_, _next@12_0}
                                                    when not
                                                             ((erlang:is_tuple(_value@13_0)
                                                                   andalso
                                                                   erlang:tuple_size(_value@13_0)
                                                                       =:= 2)
                                                                  andalso
                                                                  erlang:element(1,
                                                                                 _value@13_0)
                                                                      =:=
                                                                      some) ->
                                                    _iter@11_1 = _next@12_0,
                                                    _n@10_1 = _n@10_0 + 1,
                                                    {}
                                            end,
                                            _$patina_loop(_iter@11_1, _n@10_1)
                                    end
                            end(_iter@11_0, _n@10_0),
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
    _iter@11_0 =
        _fn_IntoIterator_into_iter@4_0(_witness_IntoIterator@2_0,
                                       _items@1_0),
    {_iter@11_1, _total@10_1} = fun
                                    _$patina_loop(_iter@11_0, _total@10_0) ->
                                        case [] of
                                            {'$patina_label_0',
                                             _iter@11_2,
                                             _total@10_2} ->
                                                {_iter@11_2, _total@10_2};
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
                                                        _iter@11_1 = _iter@11_0,
                                                        _total@10_1 =
                                                            _total@10_0,
                                                        _branch_value@15_0;
                                                    _value@14_0 = {_item@12_0,
                                                                   _next@13_0}
                                                        when
                                                          erlang:is_tuple(_value@14_0)
                                                              andalso
                                                              erlang:tuple_size(_value@14_0)
                                                                  =:= 2 ->
                                                        _iter@11_1 = _next@13_0,
                                                        _total@10_1 =
                                                            _total@10_0 +
                                                                _item@12_0,
                                                        {}
                                                end,
                                                _$patina_loop(_iter@11_1,
                                                              _total@10_1)
                                        end
                                end(_iter@11_0, _total@10_0),
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
    _total@2_0 = 0,
    _iter@5_0 = begin
                    _self@3_0 = _xs@0_0, _self@4_0 = _self@3_0, _self@4_0
                end,
    {_iter@5_1, _total@2_1} = fun _$patina_loop(_iter@5_0,
                                                _total@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@5_2,
                                           _total@2_2} ->
                                              {_iter@5_2, _total@2_2};
                                          _ ->
                                              case [] of
                                                  {'$patina_label_1',
                                                   _iter@5_1,
                                                   _total@2_1} ->
                                                      {};
                                                  _ ->
                                                      case begin
                                                               _self@6_0 =
                                                                   _iter@5_0,
                                                               case _self@6_0 of
                                                                   [_head@8_0
                                                                    | _tail@7_0 =
                                                                          _] ->
                                                                       _0@10_0 =
                                                                           begin
                                                                               _element@9_0 =
                                                                                   _tail@7_0,
                                                                               {_head@8_0,
                                                                                _element@9_0}
                                                                           end,
                                                                       _0@10_0;
                                                                   [] -> nil
                                                               end
                                                           end
                                                          of
                                                          nil ->
                                                              _branch_value@14_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@5_0,
                                                                                            _total@2_0),
                                                                      []
                                                                  end,
                                                              _iter@5_1 =
                                                                  _iter@5_0,
                                                              _total@2_1 =
                                                                  _total@2_0,
                                                              _branch_value@14_0;
                                                          _value@13_0 =
                                                              {_x@11_0,
                                                               _next@12_0}
                                                              when
                                                                erlang:is_tuple(_value@13_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@13_0)
                                                                        =:= 2 ->
                                                              _iter@5_1 =
                                                                  _next@12_0,
                                                              case _x@11_0 rem 2
                                                                       =:= 1
                                                                  of
                                                                  true ->
                                                                      '$patina_marker':jump('$patina_label_1',
                                                                                            _iter@5_1,
                                                                                            _total@2_0),
                                                                      [];
                                                                  _ -> {}
                                                              end,
                                                              case _x@11_0 >
                                                                       _limit@1_0
                                                                  of
                                                                  true ->
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@5_1,
                                                                                            _total@2_0),
                                                                      [];
                                                                  _ -> {}
                                                              end,
                                                              _total@2_1 =
                                                                  _total@2_0 +
                                                                      _x@11_0,
                                                              {}
                                                      end
                                              end,
                                              _$patina_loop(_iter@5_1,
                                                            _total@2_1)
                                      end
                              end(_iter@5_0, _total@2_0),
    _total@2_1.

find_index(_xs@0_0, _target@1_0) ->
    case [] of
        {'$patina_label_1', _jump_value@15_0} ->
            _jump_value@15_0;
        _ ->
            _index@2_0 = 0,
            _iter@5_0 = begin
                            _self@3_0 = _xs@0_0,
                            _self@4_0 = _self@3_0,
                            _self@4_0
                        end,
            {_iter@5_1, _index@2_1} = fun _$patina_loop(_iter@5_0,
                                                        _index@2_0) ->
                                              case [] of
                                                  {'$patina_label_0',
                                                   _iter@5_2,
                                                   _index@2_2} ->
                                                      {_iter@5_2, _index@2_2};
                                                  _ ->
                                                      case begin
                                                               _self@6_0 =
                                                                   _iter@5_0,
                                                               case _self@6_0 of
                                                                   [_head@8_0
                                                                    | _tail@7_0 =
                                                                          _] ->
                                                                       _0@10_0 =
                                                                           begin
                                                                               _element@9_0 =
                                                                                   _tail@7_0,
                                                                               {_head@8_0,
                                                                                _element@9_0}
                                                                           end,
                                                                       _0@10_0;
                                                                   [] -> nil
                                                               end
                                                           end
                                                          of
                                                          nil ->
                                                              _branch_value@14_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@5_0,
                                                                                            _index@2_0),
                                                                      []
                                                                  end,
                                                              _iter@5_1 =
                                                                  _iter@5_0,
                                                              _index@2_1 =
                                                                  _index@2_0,
                                                              _branch_value@14_0;
                                                          _value@13_0 =
                                                              {_x@11_0,
                                                               _next@12_0}
                                                              when
                                                                erlang:is_tuple(_value@13_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@13_0)
                                                                        =:= 2 ->
                                                              _iter@5_1 =
                                                                  _next@12_0,
                                                              case _x@11_0 =:=
                                                                       _target@1_0
                                                                  of
                                                                  true ->
                                                                      '$patina_marker':jump('$patina_label_1',
                                                                                            _index@2_0),
                                                                      [];
                                                                  _ -> {}
                                                              end,
                                                              _index@2_1 =
                                                                  _index@2_0 +
                                                                      1,
                                                              {}
                                                      end,
                                                      _$patina_loop(_iter@5_1,
                                                                    _index@2_1)
                                              end
                                      end(_iter@5_0, _index@2_0),
            -1
    end.

first_pair_with_sum(_xs@0_0, _sum@1_0) ->
    _found@2_0 = {0, 0},
    _iter@5_0 = begin
                    _self@3_0 = _xs@0_0, _self@4_0 = _self@3_0, _self@4_0
                end,
    {_iter@5_1, _found@2_1} = fun _$patina_loop(_iter@5_0,
                                                _found@2_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@5_2,
                                           _found@2_2} ->
                                              {_iter@5_2, _found@2_2};
                                          _ ->
                                              case begin
                                                       _self@6_0 = _iter@5_0,
                                                       case _self@6_0 of
                                                           [_head@8_0
                                                            | _tail@7_0 = _] ->
                                                               _0@10_0 = begin
                                                                             _element@9_0 =
                                                                                 _tail@7_0,
                                                                             {_head@8_0,
                                                                              _element@9_0}
                                                                         end,
                                                               _0@10_0;
                                                           [] -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@27_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@5_0,
                                                                                                     _found@2_0),
                                                                               []
                                                                           end,
                                                      _iter@5_1 = _iter@5_0,
                                                      _found@2_1 = _found@2_0,
                                                      _branch_value@27_0;
                                                  _value@13_0 = {_a@11_0,
                                                                 _next@12_0}
                                                      when
                                                        erlang:is_tuple(_value@13_0)
                                                            andalso
                                                            erlang:tuple_size(_value@13_0)
                                                                =:= 2 ->
                                                      _iter@5_1 = _next@12_0,
                                                      _iter@16_0 = begin
                                                                       _self@14_0 =
                                                                           _xs@0_0,
                                                                       _self@15_0 =
                                                                           _self@14_0,
                                                                       _self@15_0
                                                                   end,
                                                      {_iter@16_1, _found@2_1} =
                                                          fun
                                                              _$patina_loop(_iter@16_0,
                                                                            _found@2_0) ->
                                                                  case [] of
                                                                      {'$patina_label_1',
                                                                       _iter@16_2,
                                                                       _found@2_2} ->
                                                                          {_iter@16_2,
                                                                           _found@2_2};
                                                                      _ ->
                                                                          case
                                                                              []
                                                                              of
                                                                              {'$patina_label_2',
                                                                               _iter@16_1,
                                                                               _found@2_1} ->
                                                                                  {};
                                                                              _ ->
                                                                                  case
                                                                                      begin
                                                                                          _self@17_0 =
                                                                                              _iter@16_0,
                                                                                          case
                                                                                              _self@17_0
                                                                                              of
                                                                                              [_head@19_0
                                                                                               | _tail@18_0 =
                                                                                                     _] ->
                                                                                                  _0@21_0 =
                                                                                                      begin
                                                                                                          _element@20_0 =
                                                                                                              _tail@18_0,
                                                                                                          {_head@19_0,
                                                                                                           _element@20_0}
                                                                                                      end,
                                                                                                  _0@21_0;
                                                                                              [] ->
                                                                                                  nil
                                                                                          end
                                                                                      end
                                                                                      of
                                                                                      nil ->
                                                                                          _branch_value@26_0 =
                                                                                              begin
                                                                                                  '$patina_marker':jump('$patina_label_1',
                                                                                                                        _iter@16_0,
                                                                                                                        _found@2_0),
                                                                                                  []
                                                                                              end,
                                                                                          _iter@16_1 =
                                                                                              _iter@16_0,
                                                                                          _found@2_1 =
                                                                                              _found@2_0,
                                                                                          _branch_value@26_0;
                                                                                      _value@24_0 =
                                                                                          {_b@22_0,
                                                                                           _next@23_0}
                                                                                          when
                                                                                            erlang:is_tuple(_value@24_0)
                                                                                                andalso
                                                                                                erlang:tuple_size(_value@24_0)
                                                                                                    =:=
                                                                                                    2 ->
                                                                                          _iter@16_1 =
                                                                                              _next@23_0,
                                                                                          case
                                                                                              _b@22_0
                                                                                                  =<
                                                                                                  _a@11_0
                                                                                              of
                                                                                              true ->
                                                                                                  '$patina_marker':jump('$patina_label_2',
                                                                                                                        _iter@16_1,
                                                                                                                        _found@2_0),
                                                                                                  [];
                                                                                              _ ->
                                                                                                  {}
                                                                                          end,
                                                                                          case
                                                                                              _a@11_0
                                                                                                  +
                                                                                                  _b@22_0
                                                                                                  =:=
                                                                                                  _sum@1_0
                                                                                              of
                                                                                              true ->
                                                                                                  _found@2_1 =
                                                                                                      {_a@11_0,
                                                                                                       _b@22_0},
                                                                                                  '$patina_marker':jump('$patina_label_0',
                                                                                                                        _iter@5_1,
                                                                                                                        _found@2_1),
                                                                                                  [];
                                                                                              _ ->
                                                                                                  _branch_value@25_0 =
                                                                                                      {},
                                                                                                  _found@2_1 =
                                                                                                      _found@2_0,
                                                                                                  _branch_value@25_0
                                                                                          end
                                                                                  end
                                                                          end,
                                                                          _$patina_loop(_iter@16_1,
                                                                                        _found@2_1)
                                                                  end
                                                          end(_iter@16_0,
                                                              _found@2_0),
                                                      {}
                                              end,
                                              _$patina_loop(_iter@5_1,
                                                            _found@2_1)
                                      end
                              end(_iter@5_0, _found@2_0),
    _found@2_1.

count_skipping_rows(_rows@0_0) ->
    _visited@1_0 = 0,
    _iter@4_0 = begin
                    _self@2_0 = _rows@0_0, _self@3_0 = _self@2_0, _self@3_0
                end,
    {_iter@4_1, _visited@1_1} = fun _$patina_loop(_iter@4_0,
                                                  _visited@1_0) ->
                                        case [] of
                                            {'$patina_label_0',
                                             _iter@4_2,
                                             _visited@1_2} ->
                                                {_iter@4_2, _visited@1_2};
                                            _ ->
                                                case [] of
                                                    {'$patina_label_2',
                                                     _iter@4_1,
                                                     _visited@1_1} ->
                                                        {};
                                                    _ ->
                                                        case begin
                                                                 _self@5_0 =
                                                                     _iter@4_0,
                                                                 case _self@5_0
                                                                     of
                                                                     [_head@7_0
                                                                      | _tail@6_0 =
                                                                            _] ->
                                                                         _0@9_0 =
                                                                             begin
                                                                                 _element@8_0 =
                                                                                     _tail@6_0,
                                                                                 {_head@7_0,
                                                                                  _element@8_0}
                                                                             end,
                                                                         _0@9_0;
                                                                     [] -> nil
                                                                 end
                                                             end
                                                            of
                                                            nil ->
                                                                _branch_value@25_0 =
                                                                    begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _iter@4_0,
                                                                                              _visited@1_0),
                                                                        []
                                                                    end,
                                                                _iter@4_1 =
                                                                    _iter@4_0,
                                                                _visited@1_1 =
                                                                    _visited@1_0,
                                                                _branch_value@25_0;
                                                            _value@12_0 =
                                                                {_row@10_0,
                                                                 _next@11_0}
                                                                when
                                                                  erlang:is_tuple(_value@12_0)
                                                                      andalso
                                                                      erlang:tuple_size(_value@12_0)
                                                                          =:=
                                                                          2 ->
                                                                _iter@4_1 =
                                                                    _next@11_0,
                                                                _iter@15_0 =
                                                                    begin
                                                                        _self@13_0 =
                                                                            _row@10_0,
                                                                        _self@14_0 =
                                                                            _self@13_0,
                                                                        _self@14_0
                                                                    end,
                                                                {_iter@15_1,
                                                                 _visited@1_1} =
                                                                    fun
                                                                        _$patina_loop(_iter@15_0,
                                                                                      _visited@1_0) ->
                                                                            case
                                                                                []
                                                                                of
                                                                                {'$patina_label_1',
                                                                                 _iter@15_2,
                                                                                 _visited@1_2} ->
                                                                                    {_iter@15_2,
                                                                                     _visited@1_2};
                                                                                _ ->
                                                                                    case
                                                                                        begin
                                                                                            _self@16_0 =
                                                                                                _iter@15_0,
                                                                                            case
                                                                                                _self@16_0
                                                                                                of
                                                                                                [_head@18_0
                                                                                                 | _tail@17_0 =
                                                                                                       _] ->
                                                                                                    _0@20_0 =
                                                                                                        begin
                                                                                                            _element@19_0 =
                                                                                                                _tail@17_0,
                                                                                                            {_head@18_0,
                                                                                                             _element@19_0}
                                                                                                        end,
                                                                                                    _0@20_0;
                                                                                                [] ->
                                                                                                    nil
                                                                                            end
                                                                                        end
                                                                                        of
                                                                                        nil ->
                                                                                            _branch_value@24_0 =
                                                                                                begin
                                                                                                    '$patina_marker':jump('$patina_label_1',
                                                                                                                          _iter@15_0,
                                                                                                                          _visited@1_0),
                                                                                                    []
                                                                                                end,
                                                                                            _iter@15_1 =
                                                                                                _iter@15_0,
                                                                                            _visited@1_1 =
                                                                                                _visited@1_0,
                                                                                            _branch_value@24_0;
                                                                                        _value@23_0 =
                                                                                            {_x@21_0,
                                                                                             _next@22_0}
                                                                                            when
                                                                                              erlang:is_tuple(_value@23_0)
                                                                                                  andalso
                                                                                                  erlang:tuple_size(_value@23_0)
                                                                                                      =:=
                                                                                                      2 ->
                                                                                            _iter@15_1 =
                                                                                                _next@22_0,
                                                                                            case
                                                                                                _x@21_0
                                                                                                    <
                                                                                                    0
                                                                                                of
                                                                                                true ->
                                                                                                    '$patina_marker':jump('$patina_label_2',
                                                                                                                          _iter@4_1,
                                                                                                                          _visited@1_0),
                                                                                                    [];
                                                                                                _ ->
                                                                                                    {}
                                                                                            end,
                                                                                            _visited@1_1 =
                                                                                                _visited@1_0
                                                                                                    +
                                                                                                    1,
                                                                                            {}
                                                                                    end,
                                                                                    _$patina_loop(_iter@15_1,
                                                                                                  _visited@1_1)
                                                                            end
                                                                    end(_iter@15_0,
                                                                        _visited@1_0),
                                                                {}
                                                        end
                                                end,
                                                _$patina_loop(_iter@4_1,
                                                              _visited@1_1)
                                        end
                                end(_iter@4_0, _visited@1_0),
    _visited@1_1.

countdown(_from@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.ForLoops.Countdown':'core::Iterator::$patina_witness'(),
    _seen@2_0 = [],
    _iter@4_0 = begin _self@3_0 = _from@0_0, _self@3_0 end,
    {_iter@4_1, _seen@2_1} = fun _$patina_loop(_iter@4_0,
                                               _seen@2_0) ->
                                     case [] of
                                         {'$patina_label_0',
                                          _iter@4_2,
                                          _seen@2_2} ->
                                             {_iter@4_2, _seen@2_2};
                                         _ ->
                                             case
                                                 'Elixir.Patina.ForLoops.Countdown':'core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                           _iter@4_0)
                                                 of
                                                 nil ->
                                                     _branch_value@8_0 = begin
                                                                             '$patina_marker':jump('$patina_label_0',
                                                                                                   _iter@4_0,
                                                                                                   _seen@2_0),
                                                                             []
                                                                         end,
                                                     _iter@4_1 = _iter@4_0,
                                                     _seen@2_1 = _seen@2_0,
                                                     _branch_value@8_0;
                                                 _value@7_0 = {_n@5_0,
                                                               _next@6_0}
                                                     when
                                                       erlang:is_tuple(_value@7_0)
                                                           andalso
                                                           erlang:tuple_size(_value@7_0)
                                                               =:= 2 ->
                                                     _iter@4_1 = _next@6_0,
                                                     _seen@2_1 = [_n@5_0
                                                                  | _seen@2_0],
                                                     {}
                                             end,
                                             _$patina_loop(_iter@4_1, _seen@2_1)
                                     end
                             end(_iter@4_0, _seen@2_0),
    _seen@2_1.

bag_count(_items@0_0) ->
    _witness_IntoIterator@1_0 =
        'Elixir.Patina.ForLoops.Bag':'core::IntoIterator::$patina_witness'(),
    _witnesses_where_count@2_0 =
        {_witness_IntoIterator@1_0},
    count(_witnesses_where_count@2_0, _items@0_0).
