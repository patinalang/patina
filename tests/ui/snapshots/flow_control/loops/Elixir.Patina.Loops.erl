-file("tests/ui/src/flow_control/loops.pt", 1).

-module('Elixir.Patina.Loops').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'normal_continue$patina_loop_1'/3,
         normal_continue/1,
         'loop_value$patina_loop_1'/2,
         loop_value/1,
         'zero_parameter_loop$patina_loop_1'/0,
         zero_parameter_loop/0,
         'initialize_in_loop$patina_loop_1'/1,
         initialize_in_loop/0,
         'while_sum$patina_loop_1'/3,
         while_sum/2,
         'nested$patina_loop_2'/4,
         'nested$patina_loop_1'/4,
         nested/2,
         'return_from_loop$patina_loop_1'/2,
         return_from_loop/1,
         'return_from_nested$patina_loop_2'/4,
         'return_from_nested$patina_loop_1'/3,
         return_from_nested/2,
         'return_from_nested_uneven_rebinds$patina_loop_2'/5,
         'return_from_nested_uneven_rebinds$patina_loop_1'/4,
         return_from_nested_uneven_rebinds/2,
         'continue_outer$patina_loop_2'/3,
         'continue_outer$patina_loop_1'/3,
         continue_outer/1,
         'continue_through_middle$patina_loop_3'/3,
         'continue_through_middle$patina_loop_2'/3,
         'continue_through_middle$patina_loop_1'/3,
         continue_through_middle/1,
         'cast_loop_value$patina_loop_1'/0,
         cast_loop_value/0,
         'helper_names$patina_loop_1'/3,
         'helper_names$patina_loop_3'/3,
         'helper_names$patina_loop_4'/3,
         'helper_names$patina_loop_2'/3,
         'helper_names$patina_loop_5'/2,
         helper_names/1,
         trait_helper_names/1]).

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

'normal_continue$patina_loop_1'(_count@1_0, _total@2_0,
                                _limit@0_0) ->
    _ = begin
            _count@1_1 = _count@1_0 + 1,
            case _count@1_1 rem 2 =:= 0 of
                true ->
                    '$patina_marker':continue(_count@1_1,
                                              _total@2_0,
                                              _limit@0_0);
                _ -> {}
            end,
            _total@2_1 = _total@2_0 + _count@1_1,
            case _count@1_1 >= _limit@0_0 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            {},
                                            _count@1_1,
                                            _total@2_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_count@1_1,
                              _total@2_1,
                              _limit@0_0).

normal_continue(_limit@0_0) ->
    _count@1_0 = 0,
    _total@2_0 = 0,
    case 'normal_continue$patina_loop_1'(_count@1_0,
                                         _total@2_0,
                                         _limit@0_0)
        of
        {break, 0, _loop_value@3_0, _count@1_1, _total@2_1} ->
            _loop_value@3_0
    end,
    _total@2_1 * 100 + _count@1_1.

'loop_value$patina_loop_1'(_counter@1_0, _target@0_0) ->
    _ = begin
            _counter@1_1 = _counter@1_0 + 1,
            case _counter@1_1 =:= _target@0_0 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            _counter@1_1 * 2,
                                            _counter@1_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_counter@1_1, _target@0_0).

loop_value(_target@0_0) ->
    _counter@1_0 = 0,
    _result@3_0 = case
                      'loop_value$patina_loop_1'(_counter@1_0, _target@0_0)
                      of
                      {break, 0, _loop_value@2_0, _counter@1_1} ->
                          _loop_value@2_0
                  end,
    _result@3_0 * 10 + _counter@1_1.

'zero_parameter_loop$patina_loop_1'() ->
    _ = begin '$patina_marker':break({break, 0, 7}) end,
    '$patina_marker':continue().

zero_parameter_loop() ->
    case 'zero_parameter_loop$patina_loop_1'() of
        {break, 0, _loop_value@0_0} -> _loop_value@0_0
    end.

'initialize_in_loop$patina_loop_1'(_value@0_0) ->
    _ = begin
            _value@0_1 = 7,
            '$patina_marker':break({break, 0, {}, _value@0_1})
        end,
    '$patina_marker':continue(_value@0_1).

initialize_in_loop() ->
    begin _value@0_0 = {} end,
    case 'initialize_in_loop$patina_loop_1'(_value@0_0) of
        {break, 0, _loop_value@1_0, _value@0_1} ->
            _loop_value@1_0
    end,
    _value@0_1.

'while_sum$patina_loop_1'(_n@2_0, _limit@1_0,
                          _total@3_0) ->
    _ = begin
            case _n@2_0 < _limit@1_0 of
                true ->
                    _total@3_1 = _total@3_0 + _n@2_0,
                    _n@2_1 = _n@2_0 + 1,
                    {};
                _ ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@2_0,
                                                                    _total@3_0})
                                        end,
                    _total@3_1 = _total@3_0,
                    _n@2_1 = _n@2_0,
                    _branch_value@4_0
            end
        end,
    '$patina_marker':continue(_n@2_1,
                              _limit@1_0,
                              _total@3_1).

while_sum(_start@0_0, _limit@1_0) ->
    _n@2_0 = _start@0_0,
    _total@3_0 = 0,
    case 'while_sum$patina_loop_1'(_n@2_0,
                                   _limit@1_0,
                                   _total@3_0)
        of
        {break, 0, _loop_value@5_0, _n@2_1, _total@3_1} ->
            _loop_value@5_0
    end,
    _total@3_1 * 100 + _n@2_1.

'nested$patina_loop_2'(_inner@4_0, _inner_limit@1_0,
                       _visits@3_0, _outer@2_1) ->
    _ = begin
            _inner@4_1 = _inner@4_0 + 1,
            case _inner@4_1 < _inner_limit@1_0 of
                true ->
                    '$patina_marker':continue(_inner@4_1,
                                              _inner_limit@1_0,
                                              _visits@3_0,
                                              _outer@2_1);
                _ -> {}
            end,
            _visits@3_1 = _visits@3_0 +
                              (_outer@2_1 * 10 + _inner@4_1),
            '$patina_marker':break({break,
                                    1,
                                    {},
                                    _inner@4_1,
                                    _visits@3_1})
        end,
    '$patina_marker':continue(_inner@4_1,
                              _inner_limit@1_0,
                              _visits@3_1,
                              _outer@2_1).

'nested$patina_loop_1'(_outer@2_0, _inner_limit@1_0,
                       _visits@3_0, _outer_limit@0_0) ->
    _ = begin
            _outer@2_1 = _outer@2_0 + 1,
            _inner@4_0 = 0,
            case 'nested$patina_loop_2'(_inner@4_0,
                                        _inner_limit@1_0,
                                        _visits@3_0,
                                        _outer@2_1)
                of
                {break, 1, _loop_value@5_0, _inner@4_1, _visits@3_1} ->
                    _loop_value@5_0
            end,
            case _outer@2_1 >= _outer_limit@0_0 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            {},
                                            _outer@2_1,
                                            _visits@3_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_outer@2_1,
                              _inner_limit@1_0,
                              _visits@3_1,
                              _outer_limit@0_0).

nested(_outer_limit@0_0, _inner_limit@1_0) ->
    _outer@2_0 = 0,
    _visits@3_0 = 0,
    case 'nested$patina_loop_1'(_outer@2_0,
                                _inner_limit@1_0,
                                _visits@3_0,
                                _outer_limit@0_0)
        of
        {break, 0, _loop_value@6_0, _outer@2_1, _visits@3_1} ->
            _loop_value@6_0
    end,
    _visits@3_1 * 100 + _outer@2_1.

'return_from_loop$patina_loop_1'(_n@1_0,
                                 _stop_at@0_0) ->
    _ = begin
            _n@1_1 = _n@1_0 + 1,
            case _n@1_1 =:= _stop_at@0_0 of
                true ->
                    '$patina_marker':return({return, _n@1_1 * 10, _n@1_1});
                _ -> {}
            end,
            case _n@1_1 >= 10 of
                true -> '$patina_marker':break({break, 0, {}, _n@1_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_n@1_1, _stop_at@0_0).

return_from_loop(_stop_at@0_0) ->
    _n@1_0 = 0,
    case 'return_from_loop$patina_loop_1'(_n@1_0,
                                          _stop_at@0_0)
        of
        {break, 0, _loop_value@2_0, _n@1_1} -> _loop_value@2_0;
        {return, _loop_value@3_0, _n@1_1} ->
            '$patina_marker':return(_loop_value@3_0)
    end,
    -1.

'return_from_nested$patina_loop_2'(_inner@3_0,
                                   _outer@2_1, _outer_stop@0_0,
                                   _inner_stop@1_0) ->
    _ = begin
            _inner@3_1 = _inner@3_0 + 1,
            case _outer@2_1 =:= _outer_stop@0_0 andalso
                     _inner@3_1 =:= _inner_stop@1_0
                of
                true ->
                    '$patina_marker':return({return,
                                             _outer@2_1 * 100 + _inner@3_1,
                                             _inner@3_1});
                _ -> {}
            end,
            case _inner@3_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 1, {}, _inner@3_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_inner@3_1,
                              _outer@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0).

'return_from_nested$patina_loop_1'(_outer@2_0,
                                   _outer_stop@0_0, _inner_stop@1_0) ->
    _ = begin
            _outer@2_1 = _outer@2_0 + 1,
            _inner@3_0 = 0,
            case 'return_from_nested$patina_loop_2'(_inner@3_0,
                                                    _outer@2_1,
                                                    _outer_stop@0_0,
                                                    _inner_stop@1_0)
                of
                {break, 1, _loop_value@4_0, _inner@3_1} ->
                    _loop_value@4_0;
                {return, _loop_value@5_0, _inner@3_1} ->
                    '$patina_marker':return({return,
                                             _loop_value@5_0,
                                             _outer@2_1})
            end,
            case _outer@2_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 0, {}, _outer@2_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_outer@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0).

return_from_nested(_outer_stop@0_0, _inner_stop@1_0) ->
    _outer@2_0 = 0,
    case 'return_from_nested$patina_loop_1'(_outer@2_0,
                                            _outer_stop@0_0,
                                            _inner_stop@1_0)
        of
        {break, 0, _loop_value@6_0, _outer@2_1} ->
            _loop_value@6_0;
        {return, _loop_value@7_0, _outer@2_1} ->
            '$patina_marker':return(_loop_value@7_0)
    end,
    -1.

'return_from_nested_uneven_rebinds$patina_loop_2'(_inner@4_0,
                                                  _a@2_1, _outer_stop@0_0,
                                                  _inner_stop@1_0, _b@3_1) ->
    _ = begin
            _inner@4_1 = _inner@4_0 + 1,
            case _a@2_1 =:= _outer_stop@0_0 andalso
                     _inner@4_1 =:= _inner_stop@1_0
                of
                true ->
                    '$patina_marker':return({return,
                                             _a@2_1 * 1000 + _b@3_1 * 10 +
                                                 _inner@4_1,
                                             _inner@4_1});
                _ -> {}
            end,
            case _inner@4_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 1, {}, _inner@4_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_inner@4_1,
                              _a@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0,
                              _b@3_1).

'return_from_nested_uneven_rebinds$patina_loop_1'(_a@2_0,
                                                  _b@3_0, _outer_stop@0_0,
                                                  _inner_stop@1_0) ->
    _ = begin
            _a@2_1 = _a@2_0 + 1,
            _b@3_1 = _b@3_0 + 10,
            _inner@4_0 = 0,
            case
                'return_from_nested_uneven_rebinds$patina_loop_2'(_inner@4_0,
                                                                  _a@2_1,
                                                                  _outer_stop@0_0,
                                                                  _inner_stop@1_0,
                                                                  _b@3_1)
                of
                {break, 1, _loop_value@5_0, _inner@4_1} ->
                    _loop_value@5_0;
                {return, _loop_value@6_0, _inner@4_1} ->
                    '$patina_marker':return({return,
                                             _loop_value@6_0,
                                             _a@2_1,
                                             _b@3_1})
            end,
            case _a@2_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 0, {}, _a@2_1, _b@3_1});
                _ -> {}
            end
        end,
    '$patina_marker':continue(_a@2_1,
                              _b@3_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0).

return_from_nested_uneven_rebinds(_outer_stop@0_0,
                                  _inner_stop@1_0) ->
    _a@2_0 = 0,
    _b@3_0 = 0,
    case
        'return_from_nested_uneven_rebinds$patina_loop_1'(_a@2_0,
                                                          _b@3_0,
                                                          _outer_stop@0_0,
                                                          _inner_stop@1_0)
        of
        {break, 0, _loop_value@7_0, _a@2_1, _b@3_1} ->
            _loop_value@7_0;
        {return, _loop_value@8_0, _a@2_1, _b@3_1} ->
            '$patina_marker':return(_loop_value@8_0)
    end,
    -1.

'continue_outer$patina_loop_2'(_visits@2_0, _outer@1_1,
                               _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1});
                _ -> {}
            end,
            '$patina_marker':break({break, 0, {}, _visits@2_1})
        end,
    '$patina_marker':continue(_visits@2_1,
                              _outer@1_1,
                              _limit@0_0).

'continue_outer$patina_loop_1'(_outer@1_0, _visits@2_0,
                               _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case 'continue_outer$patina_loop_2'(_visits@2_0,
                                                _outer@1_1,
                                                _limit@0_0)
                of
                {break,
                 _target_loop_depth@3_0,
                 _loop_value@4_0,
                 _visits@2_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@3_0,
                                            _loop_value@4_0,
                                            _outer@1_1,
                                            _visits@2_1});
                {continue, 0, _visits@2_1} ->
                    '$patina_marker':continue(_outer@1_1,
                                              _visits@2_1,
                                              _limit@0_0)
            end
        end,
    '$patina_marker':continue(_outer@1_1,
                              _visits@2_1,
                              _limit@0_0).

continue_outer(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    case 'continue_outer$patina_loop_1'(_outer@1_0,
                                        _visits@2_0,
                                        _limit@0_0)
        of
        {break, 0, _loop_value@5_0, _outer@1_1, _visits@2_1} ->
            _loop_value@5_0
    end,
    _visits@2_1 * 100 + _outer@1_1.

'continue_through_middle$patina_loop_3'(_visits@2_0,
                                        _outer@1_1, _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1});
                _ -> {}
            end,
            '$patina_marker':break({break, 0, {}, _visits@2_1})
        end,
    '$patina_marker':continue(_visits@2_1,
                              _outer@1_1,
                              _limit@0_0).

'continue_through_middle$patina_loop_2'(_visits@2_0,
                                        _outer@1_1, _limit@0_0) ->
    _ = begin
            case
                'continue_through_middle$patina_loop_3'(_visits@2_0,
                                                        _outer@1_1,
                                                        _limit@0_0)
                of
                {break,
                 _target_loop_depth@3_0,
                 _loop_value@4_0,
                 _visits@2_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@3_0,
                                            _loop_value@4_0,
                                            _visits@2_1});
                {continue, _target_loop_depth@5_0, _visits@2_1} ->
                    '$patina_marker':break({continue,
                                            _target_loop_depth@5_0,
                                            _visits@2_1})
            end
        end,
    '$patina_marker':continue(_visits@2_1,
                              _outer@1_1,
                              _limit@0_0).

'continue_through_middle$patina_loop_1'(_outer@1_0,
                                        _visits@2_0, _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case
                'continue_through_middle$patina_loop_2'(_visits@2_0,
                                                        _outer@1_1,
                                                        _limit@0_0)
                of
                {break,
                 _target_loop_depth@6_0,
                 _loop_value@7_0,
                 _visits@2_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@6_0,
                                            _loop_value@7_0,
                                            _outer@1_1,
                                            _visits@2_1});
                {continue, 0, _visits@2_1} ->
                    '$patina_marker':continue(_outer@1_1,
                                              _visits@2_1,
                                              _limit@0_0)
            end
        end,
    '$patina_marker':continue(_outer@1_1,
                              _visits@2_1,
                              _limit@0_0).

continue_through_middle(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    case 'continue_through_middle$patina_loop_1'(_outer@1_0,
                                                 _visits@2_0,
                                                 _limit@0_0)
        of
        {break, 0, _loop_value@8_0, _outer@1_1, _visits@2_1} ->
            _loop_value@8_0
    end,
    _visits@2_1 * 100 + _outer@1_1.

'cast_loop_value$patina_loop_1'() ->
    _ = begin '$patina_marker':break({break, 0, 1}) end,
    '$patina_marker':continue().

cast_loop_value() ->
    _value@1_0 = case 'cast_loop_value$patina_loop_1'() of
                     {break, 0, _loop_value@0_0} -> _loop_value@0_0
                 end,
    _value@1_0.

'helper_names$patina_loop_1'(_i@2_0, _limit@0_0,
                             _total@1_0) ->
    _ = begin
            case _i@2_0 < _limit@0_0 of
                true ->
                    _total@1_1 = _total@1_0 + 1,
                    _i@2_1 = _i@2_0 + 1,
                    {};
                _ ->
                    _branch_value@3_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _i@2_0,
                                                                    _total@1_0})
                                        end,
                    _total@1_1 = _total@1_0,
                    _i@2_1 = _i@2_0,
                    _branch_value@3_0
            end
        end,
    '$patina_marker':continue(_i@2_1,
                              _limit@0_0,
                              _total@1_1).

'helper_names$patina_loop_3'(_k@6_0, _limit@0_0,
                             _total@1_1) ->
    _ = begin
            case _k@6_0 < _limit@0_0 of
                true ->
                    _total@1_2 = _total@1_1 + 10,
                    _k@6_1 = _k@6_0 + 1,
                    {};
                _ ->
                    _branch_value@7_0 = begin
                                            '$patina_marker':break({break,
                                                                    1,
                                                                    {},
                                                                    _k@6_0,
                                                                    _total@1_1})
                                        end,
                    _total@1_2 = _total@1_1,
                    _k@6_1 = _k@6_0,
                    _branch_value@7_0
            end
        end,
    '$patina_marker':continue(_k@6_1,
                              _limit@0_0,
                              _total@1_2).

'helper_names$patina_loop_4'(_l@9_0, _limit@0_0,
                             _total@1_2) ->
    _ = begin
            case _l@9_0 < _limit@0_0 of
                true ->
                    _total@1_3 = _total@1_2 + 100,
                    _l@9_1 = _l@9_0 + 1,
                    {};
                _ ->
                    _branch_value@10_0 = begin
                                             '$patina_marker':break({break,
                                                                     1,
                                                                     {},
                                                                     _l@9_0,
                                                                     _total@1_2})
                                         end,
                    _total@1_3 = _total@1_2,
                    _l@9_1 = _l@9_0,
                    _branch_value@10_0
            end
        end,
    '$patina_marker':continue(_l@9_1,
                              _limit@0_0,
                              _total@1_3).

'helper_names$patina_loop_2'(_j@5_0, _limit@0_0,
                             _total@1_1) ->
    _ = begin
            case _j@5_0 < _limit@0_0 of
                true ->
                    _k@6_0 = 0,
                    case 'helper_names$patina_loop_3'(_k@6_0,
                                                      _limit@0_0,
                                                      _total@1_1)
                        of
                        {break, 1, _loop_value@8_0, _k@6_1, _total@1_2} ->
                            _loop_value@8_0
                    end,
                    _l@9_0 = 0,
                    case 'helper_names$patina_loop_4'(_l@9_0,
                                                      _limit@0_0,
                                                      _total@1_2)
                        of
                        {break, 1, _loop_value@11_0, _l@9_1, _total@1_3} ->
                            _loop_value@11_0
                    end,
                    _j@5_1 = _j@5_0 + 1,
                    {};
                _ ->
                    _branch_value@12_0 = begin
                                             '$patina_marker':break({break,
                                                                     0,
                                                                     {},
                                                                     _j@5_0,
                                                                     _total@1_1})
                                         end,
                    _total@1_3 = _total@1_1,
                    _j@5_1 = _j@5_0,
                    _branch_value@12_0
            end
        end,
    '$patina_marker':continue(_j@5_1,
                              _limit@0_0,
                              _total@1_3).

'helper_names$patina_loop_5'(_m@15_0, _n@14_0) ->
    _ = begin
            case _m@15_0 < _n@14_0 of
                true ->
                    _m@15_1 = _m@15_0 + 1,
                    {};
                _ ->
                    _branch_value@16_0 = begin
                                             '$patina_marker':break({break,
                                                                     0,
                                                                     {},
                                                                     _m@15_0})
                                         end,
                    _m@15_1 = _m@15_0,
                    _branch_value@16_0
            end
        end,
    '$patina_marker':continue(_m@15_1, _n@14_0).

helper_names(_limit@0_0) ->
    _total@1_0 = 0,
    _i@2_0 = 0,
    case 'helper_names$patina_loop_1'(_i@2_0,
                                      _limit@0_0,
                                      _total@1_0)
        of
        {break, 0, _loop_value@4_0, _i@2_1, _total@1_1} ->
            _loop_value@4_0
    end,
    _j@5_0 = 0,
    case 'helper_names$patina_loop_2'(_j@5_0,
                                      _limit@0_0,
                                      _total@1_1)
        of
        {break, 0, _loop_value@13_0, _j@5_1, _total@1_2} ->
            _loop_value@13_0
    end,
    _f@18_0 = fun (_n@14_0) ->
                      _m@15_0 = 0,
                      case 'helper_names$patina_loop_5'(_m@15_0, _n@14_0) of
                          {break, 0, _loop_value@17_0, _m@15_1} ->
                              _loop_value@17_0
                      end,
                      _m@15_1 * 1000
              end,
    _total@1_2 + _f@18_0(_limit@0_0).

trait_helper_names(_limit@0_0) ->
    _witness_Count@1_0 =
        'Elixir.Patina.Loops.Counter':'Loops::Count::$patina_witness<int>'(),
    _witness_Count@2_0 =
        'Elixir.Patina.Loops.Counter':'Loops::Count::$patina_witness<str>'(),
    begin
        'Elixir.Patina.Loops.Counter':'Loops::Count::count<int>'(_witness_Count@1_0,
                                                                 0,
                                                                 _limit@0_0)
            +
            'Elixir.Patina.Loops.Counter':'Loops::Count::count<str>'(_witness_Count@2_0,
                                                                     <<"a"/utf8>>,
                                                                     _limit@0_0)
    end.
