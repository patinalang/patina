-file("tests/ui/src/fn/inline.pt", 1).

-module('Elixir.Patina.Inline').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         add/2,
         inlines_free_fn/0,
         binds_pattern_param/0,
         inlines_nested_inline_calls/0,
         inlines_each_call_separately/0,
         return_leaves_only_the_copy/1,
         inlines_loop/0,
         inlines_closure/0,
         countdown/1,
         recursion_inlines_once/0,
         fn_item_value_stays_reference/0,
         call_through_fn_item_inlines/0,
         'inline::Shape::double_area'/2,
         inlines_inherent_method/0,
         inlines_impl_method/0,
         inlines_default_method/0,
         copy_calls_impl_method_statically/0,
         copy_inlines_resolved_trait_method/0,
         generic_caller_dispatches_through_witness/0,
         area_plus_one/2,
         copy_builds_witness_for_generic_call/0,
         binds_const_once_for_all_copies/0,
         exports_helpers_of_inline_fns/0]).

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

add(_a@0_0, _b@1_0) -> _a@0_0 + _b@1_0.

inlines_free_fn() ->
    _a@0_0 = 1,
    _b@1_0 = 2,
    _a@0_0 + _b@1_0.

swap({_a@0_0, _b@1_0}) -> {_b@1_0, _a@0_0}.

binds_pattern_param() ->
    {_a@0_0, _b@1_0} = {1, 2},
    {_b@1_0, _a@0_0}.

add_twice(_a@0_0, _b@1_0) ->
    _a@4_0 = begin
                 _a@2_0 = _a@0_0, _b@3_0 = _b@1_0, _a@2_0 + _b@3_0
             end,
    _b@5_0 = _b@1_0,
    _a@4_0 + _b@5_0.

inlines_nested_inline_calls() ->
    _a@0_0 = 1,
    _b@1_0 = 2,
    _a@4_0 = begin
                 _a@2_0 = _a@0_0, _b@3_0 = _b@1_0, _a@2_0 + _b@3_0
             end,
    _b@5_0 = _b@1_0,
    _a@4_0 + _b@5_0.

inlines_each_call_separately() ->
    begin _a@0_0 = 1, _b@1_0 = 2, _a@0_0 + _b@1_0 end *
        begin _a@2_0 = 3, _b@3_0 = 4, _a@2_0 + _b@3_0 end.

clamp(_x@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@1_0} -> _jump_value@1_0;
        _ ->
            case _x@0_0 > 10 of
                true ->
                    '$patina_marker':jump('$patina_label_0', 10),
                    [];
                _ -> {}
            end,
            _x@0_0
    end.

return_leaves_only_the_copy(_x@0_0) ->
    begin
        _x@1_0 = _x@0_0,
        case [] of
            {'$patina_label_0', _jump_value@2_0} -> _jump_value@2_0;
            _ ->
                case _x@1_0 > 10 of
                    true ->
                        '$patina_marker':jump('$patina_label_0', 10),
                        [];
                    _ -> {}
                end,
                _x@1_0
        end
    end
        + 1.

sum_to(_n@0_0) ->
    _total@1_0 = 0,
    _iter@3_0 = begin
                    _self@2_0 =
                        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                                _n@0_0),
                    _self@2_0
                end,
    {_iter@3_1, _total@1_1} = fun _$patina_loop(_iter@3_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@3_2,
                                           _total@1_2} ->
                                              {_iter@3_2, _total@1_2};
                                          _ ->
                                              case begin
                                                       _self@4_0 = _iter@3_0,
                                                       case
                                                           _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                               <
                                                               _self@4_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                                           of
                                                           true ->
                                                               _0@8_0 = begin
                                                                            _element@5_0 =
                                                                                _self@4_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                                            _element@7_0 =
                                                                                begin
                                                                                    _start@6_0 =
                                                                                        _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                                            +
                                                                                            1,
                                                                                    _self@4_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                                     =
                                                                                                                                     _start@6_0}
                                                                                end,
                                                                            {_element@5_0,
                                                                             _element@7_0}
                                                                        end,
                                                               _0@8_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@12_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@3_0,
                                                                                                     _total@1_0),
                                                                               []
                                                                           end,
                                                      _iter@3_1 = _iter@3_0,
                                                      _total@1_1 = _total@1_0,
                                                      _branch_value@12_0;
                                                  _value@11_0 = {_i@9_0,
                                                                 _next@10_0}
                                                      when
                                                        erlang:is_tuple(_value@11_0)
                                                            andalso
                                                            erlang:tuple_size(_value@11_0)
                                                                =:= 2 ->
                                                      _iter@3_1 = _next@10_0,
                                                      _total@1_1 = _total@1_0 +
                                                                       _i@9_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@3_1,
                                                            _total@1_1)
                                      end
                              end(_iter@3_0, _total@1_0),
    _total@1_1.

inlines_loop() ->
    _n@0_0 = 4,
    _total@1_0 = 0,
    _iter@3_0 = begin
                    _self@2_0 =
                        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                                _n@0_0),
                    _self@2_0
                end,
    {_iter@3_1, _total@1_1} = fun _$patina_loop(_iter@3_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@3_2,
                                           _total@1_2} ->
                                              {_iter@3_2, _total@1_2};
                                          _ ->
                                              case begin
                                                       _self@4_0 = _iter@3_0,
                                                       case
                                                           _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                               <
                                                               _self@4_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                                           of
                                                           true ->
                                                               _0@8_0 = begin
                                                                            _element@5_0 =
                                                                                _self@4_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                                            _element@7_0 =
                                                                                begin
                                                                                    _start@6_0 =
                                                                                        _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                                            +
                                                                                            1,
                                                                                    _self@4_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                                     =
                                                                                                                                     _start@6_0}
                                                                                end,
                                                                            {_element@5_0,
                                                                             _element@7_0}
                                                                        end,
                                                               _0@8_0;
                                                           _ -> nil
                                                       end
                                                   end
                                                  of
                                                  nil ->
                                                      _branch_value@12_0 = begin
                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                     _iter@3_0,
                                                                                                     _total@1_0),
                                                                               []
                                                                           end,
                                                      _iter@3_1 = _iter@3_0,
                                                      _total@1_1 = _total@1_0,
                                                      _branch_value@12_0;
                                                  _value@11_0 = {_i@9_0,
                                                                 _next@10_0}
                                                      when
                                                        erlang:is_tuple(_value@11_0)
                                                            andalso
                                                            erlang:tuple_size(_value@11_0)
                                                                =:= 2 ->
                                                      _iter@3_1 = _next@10_0,
                                                      _total@1_1 = _total@1_0 +
                                                                       _i@9_0,
                                                      {}
                                              end,
                                              _$patina_loop(_iter@3_1,
                                                            _total@1_1)
                                      end
                              end(_iter@3_0, _total@1_0),
    _total@1_1.

adder(_n@0_0) -> fun (_x@1_0) -> _x@1_0 + _n@0_0 end.

inlines_closure() ->
    begin
        _n@0_0 = 1, fun (_x@1_0) -> _x@1_0 + _n@0_0 end
    end(2).

countdown(_n@0_0) ->
    case _n@0_0 =:= 0 of
        true -> 0;
        _ -> countdown(_n@0_0 - 1) + 1
    end.

recursion_inlines_once() ->
    _n@0_0 = 3,
    case _n@0_0 =:= 0 of
        true -> 0;
        _ -> countdown(_n@0_0 - 1) + 1
    end.

fn_item_value_stays_reference() -> fun add/2.

call_through_fn_item_inlines() ->
    _f@0_0 = fun add/2,
    _a@1_0 = 5,
    _b@2_0 = 6,
    _a@1_0 + _b@2_0.

'inline::Shape::double_area'(_witness_Shape@0_0,
                             _self@1_0) ->
    _vtable_Shape@2_0 = erlang:element(1,
                                       _witness_Shape@0_0),
    _fn_Shape_area@3_0 = erlang:element(1,
                                        _vtable_Shape@2_0),
    _fn_Shape_area@3_0(_witness_Shape@0_0, _self@1_0) * 2.

inlines_inherent_method() ->
    _self@0_0 = 3,
    _self@0_0.

inlines_impl_method() ->
    _self@0_0 = 3,
    _self@0_0 * _self@0_0.

inlines_default_method() ->
    _self@0_0 = 3,
    begin _self@1_0 = _self@0_0, _self@1_0 * _self@1_0 end *
        2.

copy_calls_impl_method_statically() ->
    _witness_Shape@0_0 =
        'Elixir.Patina.Inline.Circle':'inline::Shape::$patina_witness'(),
    _self@1_0 = 1,
    'Elixir.Patina.Inline.Circle':'inline::Shape::area'(_witness_Shape@0_0,
                                                        _self@1_0)
        * 2.

area_of(_witnesses_where_area_of@0_0, _shape@1_0) ->
    _witness_Shape@2_0 = erlang:element(1,
                                        _witnesses_where_area_of@0_0),
    _vtable_Shape@3_0 = erlang:element(1,
                                       _witness_Shape@2_0),
    _fn_Shape_area@4_0 = erlang:element(1,
                                        _vtable_Shape@3_0),
    _fn_Shape_area@4_0(_witness_Shape@2_0, _shape@1_0).

copy_inlines_resolved_trait_method() ->
    _shape@0_0 = 4,
    _self@1_0 = _shape@0_0,
    _self@1_0 * _self@1_0.

generic_caller(_witnesses_where_generic_caller@0_0,
               _shape@1_0) ->
    _witness_Shape@2_0 = erlang:element(1,
                                        _witnesses_where_generic_caller@0_0),
    _vtable_Shape@3_0 = erlang:element(1,
                                       _witness_Shape@2_0),
    _fn_Shape_area@4_0 = erlang:element(1,
                                        _vtable_Shape@3_0),
    _shape@5_0 = _shape@1_0,
    _fn_Shape_area@4_0(_witness_Shape@2_0, _shape@5_0).

generic_caller_dispatches_through_witness() ->
    _witness_Shape@0_0 =
        'Elixir.Patina.Inline.Square':'inline::Shape::$patina_witness'(),
    _witnesses_where_generic_caller@1_0 =
        {_witness_Shape@0_0},
    generic_caller(_witnesses_where_generic_caller@1_0, 5).

area_plus_one(_witnesses_where_area_plus_one@0_0,
              _shape@1_0) ->
    _witness_Shape@2_0 = erlang:element(1,
                                        _witnesses_where_area_plus_one@0_0),
    _vtable_Shape@3_0 = erlang:element(1,
                                       _witness_Shape@2_0),
    _fn_Shape_area@4_0 = erlang:element(1,
                                        _vtable_Shape@3_0),
    _fn_Shape_area@4_0(_witness_Shape@2_0, _shape@1_0) + 1.

twice_area_plus_one(_witnesses_where_twice_area_plus_one@0_0,
                    _shape@1_0) ->
    _witness_Shape@2_0 = erlang:element(1,
                                        _witnesses_where_twice_area_plus_one@0_0),
    _witnesses_where_area_plus_one@3_0 =
        {_witness_Shape@2_0},
    area_plus_one(_witnesses_where_area_plus_one@3_0,
                  _shape@1_0)
        * 2.

copy_builds_witness_for_generic_call() ->
    _witness_Shape@0_0 =
        'Elixir.Patina.Inline.Square':'inline::Shape::$patina_witness'(),
    _witnesses_where_area_plus_one@1_0 =
        {_witness_Shape@0_0},
    _shape@2_0 = 2,
    area_plus_one(_witnesses_where_area_plus_one@1_0,
                  _shape@2_0)
        * 2.

plus_limit(_x@0_0) ->
    _const_LIMIT@1_0 = 2 + 3,
    _x@0_0 + _const_LIMIT@1_0.

binds_const_once_for_all_copies() ->
    _const_LIMIT@0_0 = 2 + 3,
    begin _x@1_0 = 1, _x@1_0 + _const_LIMIT@0_0 end +
        begin _x@2_0 = 2, _x@2_0 + _const_LIMIT@0_0 end
        + _const_LIMIT@0_0.

exports_helpers_of_inline_fns() ->
    _x@0_0 = 4,
    'Elixir.Patina.Inline.Helpers':secret(_x@0_0).
