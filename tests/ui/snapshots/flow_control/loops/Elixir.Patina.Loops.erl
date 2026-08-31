-file("tests/ui/src/flow_control/loops.pt", 1).

-module('Elixir.Patina.Loops').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         '$patina_loop_10_48'/3,
         normal_continue/1,
         '$patina_loop_11_29'/2,
         loop_value/1,
         '$patina_loop_12_6'/0,
         zero_parameter_loop/0,
         '$patina_loop_13_10'/1,
         initialize_in_loop/0,
         '$patina_loop_14_34'/3,
         while_sum/2,
         '$patina_loop_15_49'/4,
         '$patina_loop_15_65'/4,
         nested/2,
         '$patina_loop_16_42'/2,
         return_from_loop/1,
         '$patina_loop_17_62'/4,
         '$patina_loop_17_78'/3,
         return_from_nested/2,
         '$patina_loop_18_78'/5,
         '$patina_loop_18_94'/4,
         return_from_nested_uneven_rebinds/2,
         '$patina_loop_19_34'/3,
         '$patina_loop_19_37'/3,
         continue_outer/1,
         '$patina_loop_20_34'/3,
         '$patina_loop_20_38'/3,
         '$patina_loop_20_41'/3,
         continue_through_middle/1,
         '$patina_loop_21_8'/0,
         cast_loop_value/0]).

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

'$patina_loop_10_48'(_count@1_0, _total@2_0,
                     _limit@0_0) ->
    _ = begin
            _count@1_1 = _count@1_0 + 1,
            case _count@1_1 rem 2 =:= 0 of
                true ->
                    '$patina_marker':continue(_count@1_1,
                                              _total@2_0,
                                              _limit@0_0),
                    {};
                false -> {}
            end,
            _total@2_1 = _total@2_0 + _count@1_1,
            case _count@1_1 >= _limit@0_0 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            {},
                                            _count@1_1,
                                            _total@2_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_count@1_1,
                              _total@2_1,
                              _limit@0_0).

normal_continue(_limit@0_0) ->
    _count@1_0 = 0,
    _total@2_0 = 0,
    case '$patina_loop_10_48'(_count@1_0,
                              _total@2_0,
                              _limit@0_0)
        of
        {break, 0, _loop_value@3_0, _count@1_1, _total@2_1} ->
            _loop_value@3_0
    end,
    _total@2_1 * 100 + _count@1_1.

'$patina_loop_11_29'(_counter@1_0, _target@0_0) ->
    _ = begin
            _counter@1_1 = _counter@1_0 + 1,
            case _counter@1_1 =:= _target@0_0 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            _counter@1_1 * 2,
                                            _counter@1_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_counter@1_1, _target@0_0).

loop_value(_target@0_0) ->
    _counter@1_0 = 0,
    _result@3_0 = case '$patina_loop_11_29'(_counter@1_0,
                                            _target@0_0)
                      of
                      {break, 0, _loop_value@2_0, _counter@1_1} ->
                          _loop_value@2_0
                  end,
    _result@3_0 * 10 + _counter@1_1.

'$patina_loop_12_6'() ->
    _ = begin '$patina_marker':break({break, 0, 7}), {} end,
    '$patina_marker':continue().

zero_parameter_loop() ->
    case '$patina_loop_12_6'() of
        {break, 0, _loop_value@0_0} -> _loop_value@0_0
    end.

'$patina_loop_13_10'(_value@0_0) ->
    _ = begin
            _value@0_1 = 7,
            '$patina_marker':break({break, 0, {}, _value@0_1}),
            {}
        end,
    '$patina_marker':continue(_value@0_1).

initialize_in_loop() ->
    case '$patina_loop_13_10'({}) of
        {break, 0, _loop_value@1_0, _value@0_1} ->
            _loop_value@1_0
    end,
    _value@0_1.

'$patina_loop_14_34'(_n@2_0, _limit@1_0, _total@3_0) ->
    _ = begin
            case _n@2_0 < _limit@1_0 of
                true ->
                    _total@3_1 = _total@3_0 + _n@2_0,
                    _n@2_1 = _n@2_0 + 1,
                    {};
                false ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _total@3_0,
                                                                    _n@2_0})
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
    case '$patina_loop_14_34'(_n@2_0,
                              _limit@1_0,
                              _total@3_0)
        of
        {break, 0, _loop_value@5_0, _total@3_1, _n@2_1} ->
            _loop_value@5_0
    end,
    _total@3_1 * 100 + _n@2_1.

'$patina_loop_15_49'(_inner@4_0, _inner_limit@1_0,
                     _visits@3_0, _outer@2_1) ->
    _ = begin
            _inner@4_1 = _inner@4_0 + 1,
            case _inner@4_1 < _inner_limit@1_0 of
                true ->
                    '$patina_marker':continue(_inner@4_1,
                                              _inner_limit@1_0,
                                              _visits@3_0,
                                              _outer@2_1),
                    {};
                false -> {}
            end,
            _visits@3_1 = _visits@3_0 +
                              (_outer@2_1 * 10 + _inner@4_1),
            '$patina_marker':break({break,
                                    1,
                                    {},
                                    _inner@4_1,
                                    _visits@3_1}),
            {}
        end,
    '$patina_marker':continue(_inner@4_1,
                              _inner_limit@1_0,
                              _visits@3_1,
                              _outer@2_1).

'$patina_loop_15_65'(_outer@2_0, _inner_limit@1_0,
                     _visits@3_0, _outer_limit@0_0) ->
    _ = begin
            _outer@2_1 = _outer@2_0 + 1,
            _inner@4_0 = 0,
            case '$patina_loop_15_49'(_inner@4_0,
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
                                            _visits@3_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_outer@2_1,
                              _inner_limit@1_0,
                              _visits@3_1,
                              _outer_limit@0_0).

nested(_outer_limit@0_0, _inner_limit@1_0) ->
    _outer@2_0 = 0,
    _visits@3_0 = 0,
    case '$patina_loop_15_65'(_outer@2_0,
                              _inner_limit@1_0,
                              _visits@3_0,
                              _outer_limit@0_0)
        of
        {break, 0, _loop_value@6_0, _outer@2_1, _visits@3_1} ->
            _loop_value@6_0
    end,
    _visits@3_1 * 100 + _outer@2_1.

'$patina_loop_16_42'(_n@1_0, _stop_at@0_0) ->
    _ = begin
            _n@1_1 = _n@1_0 + 1,
            case _n@1_1 =:= _stop_at@0_0 of
                true ->
                    '$patina_marker':return({return, _n@1_1 * 10, _n@1_1}),
                    {};
                false -> {}
            end,
            case _n@1_1 >= 10 of
                true ->
                    '$patina_marker':break({break, 0, {}, _n@1_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_n@1_1, _stop_at@0_0).

return_from_loop(_stop_at@0_0) ->
    _n@1_0 = 0,
    case '$patina_loop_16_42'(_n@1_0, _stop_at@0_0) of
        {break, 0, _loop_value@2_0, _n@1_1} -> _loop_value@2_0;
        {return, _loop_value@3_0, _n@1_1} ->
            '$patina_marker':return(_loop_value@3_0)
    end,
    -1.

'$patina_loop_17_62'(_inner@3_0, _outer@2_1,
                     _outer_stop@0_0, _inner_stop@1_0) ->
    _ = begin
            _inner@3_1 = _inner@3_0 + 1,
            case _outer@2_1 =:= _outer_stop@0_0 andalso
                     _inner@3_1 =:= _inner_stop@1_0
                of
                true ->
                    '$patina_marker':return({return,
                                             _outer@2_1 * 100 + _inner@3_1,
                                             _inner@3_1}),
                    {};
                false -> {}
            end,
            case _inner@3_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 1, {}, _inner@3_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_inner@3_1,
                              _outer@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0).

'$patina_loop_17_78'(_outer@2_0, _outer_stop@0_0,
                     _inner_stop@1_0) ->
    _ = begin
            _outer@2_1 = _outer@2_0 + 1,
            _inner@3_0 = 0,
            case '$patina_loop_17_62'(_inner@3_0,
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
                    '$patina_marker':break({break, 0, {}, _outer@2_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_outer@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0).

return_from_nested(_outer_stop@0_0, _inner_stop@1_0) ->
    _outer@2_0 = 0,
    case '$patina_loop_17_78'(_outer@2_0,
                              _outer_stop@0_0,
                              _inner_stop@1_0)
        of
        {break, 0, _loop_value@6_0, _outer@2_1} ->
            _loop_value@6_0;
        {return, _loop_value@7_0, _outer@2_1} ->
            '$patina_marker':return(_loop_value@7_0)
    end,
    -1.

'$patina_loop_18_78'(_inner@4_0, _a@2_1,
                     _outer_stop@0_0, _inner_stop@1_0, _b@3_1) ->
    _ = begin
            _inner@4_1 = _inner@4_0 + 1,
            case _a@2_1 =:= _outer_stop@0_0 andalso
                     _inner@4_1 =:= _inner_stop@1_0
                of
                true ->
                    '$patina_marker':return({return,
                                             _a@2_1 * 1000 + _b@3_1 * 10 +
                                                 _inner@4_1,
                                             _inner@4_1}),
                    {};
                false -> {}
            end,
            case _inner@4_1 >= 3 of
                true ->
                    '$patina_marker':break({break, 1, {}, _inner@4_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_inner@4_1,
                              _a@2_1,
                              _outer_stop@0_0,
                              _inner_stop@1_0,
                              _b@3_1).

'$patina_loop_18_94'(_a@2_0, _b@3_0, _outer_stop@0_0,
                     _inner_stop@1_0) ->
    _ = begin
            _a@2_1 = _a@2_0 + 1,
            _b@3_1 = _b@3_0 + 10,
            _inner@4_0 = 0,
            case '$patina_loop_18_78'(_inner@4_0,
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
                    '$patina_marker':break({break, 0, {}, _a@2_1, _b@3_1}),
                    {};
                false -> {}
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
    case '$patina_loop_18_94'(_a@2_0,
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

'$patina_loop_19_34'(_visits@2_0, _outer@1_1,
                     _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1}),
                    {};
                false -> {}
            end,
            '$patina_marker':break({break, 0, {}, _visits@2_1}),
            {}
        end,
    '$patina_marker':continue(_visits@2_1,
                              _outer@1_1,
                              _limit@0_0).

'$patina_loop_19_37'(_outer@1_0, _visits@2_0,
                     _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case '$patina_loop_19_34'(_visits@2_0,
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
    case '$patina_loop_19_37'(_outer@1_0,
                              _visits@2_0,
                              _limit@0_0)
        of
        {break, 0, _loop_value@5_0, _outer@1_1, _visits@2_1} ->
            _loop_value@5_0
    end,
    _visits@2_1 * 100 + _outer@1_1.

'$patina_loop_20_34'(_visits@2_0, _outer@1_1,
                     _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1}),
                    {};
                false -> {}
            end,
            '$patina_marker':break({break, 0, {}, _visits@2_1}),
            {}
        end,
    '$patina_marker':continue(_visits@2_1,
                              _outer@1_1,
                              _limit@0_0).

'$patina_loop_20_38'(_visits@2_0, _outer@1_1,
                     _limit@0_0) ->
    _ = begin
            case '$patina_loop_20_34'(_visits@2_0,
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

'$patina_loop_20_41'(_outer@1_0, _visits@2_0,
                     _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case '$patina_loop_20_38'(_visits@2_0,
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
    case '$patina_loop_20_41'(_outer@1_0,
                              _visits@2_0,
                              _limit@0_0)
        of
        {break, 0, _loop_value@8_0, _outer@1_1, _visits@2_1} ->
            _loop_value@8_0
    end,
    _visits@2_1 * 100 + _outer@1_1.

'$patina_loop_21_8'() ->
    _ = begin '$patina_marker':break({break, 0, 1}), {} end,
    '$patina_marker':continue().

cast_loop_value() ->
    _value@1_0 = case '$patina_loop_21_8'() of
                     {break, 0, _loop_value@0_0} -> _loop_value@0_0
                 end,
    _value@1_0.
