-file("tests/ui/src/flow_control/loop_outer_continue.pt", 1).
-module('Elixir.Patina.LoopOuterContinue').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         '$patina_loop_10_44'/3,
         '$patina_loop_10_47'/3,
         continue_outer_loop/1,
         '$patina_loop_11_44'/3,
         '$patina_loop_11_48'/3,
         '$patina_loop_11_51'/3,
         continue_two_loops/1]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
'$patina_loop_10_44'(_visits@2_0, _outer@1_1, _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1});
                false ->
                    {}
            end,
            '$patina_marker':break({break, 0,
                                    _visits@2_1 * 100 + _outer@1_1,
                                    _visits@2_1})
        end,
    '$patina_marker':continue(_visits@2_1, _outer@1_1, _limit@0_0).
'$patina_loop_10_47'(_outer@1_0, _visits@2_0, _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case
                '$patina_loop_10_44'(_visits@2_0, _outer@1_1,
                                     _limit@0_0)
            of
                {break, _target_loop_depth@3_0, _loop_value@4_0,
                 _visits@2_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@3_0,
                                            _loop_value@4_0, _outer@1_1,
                                            _visits@2_1});
                {continue, 0, _visits@2_1} ->
                    '$patina_marker':continue(_outer@1_1, _visits@2_1,
                                              _limit@0_0)
            end
        end,
    '$patina_marker':continue(_outer@1_1, _visits@2_1, _limit@0_0).
continue_outer_loop(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    case '$patina_loop_10_47'(_outer@1_0, _visits@2_0, _limit@0_0) of
        {break, 0, _loop_value@5_0, _outer@1_1, _visits@2_1} ->
            _loop_value@5_0
    end.
'$patina_loop_11_44'(_visits@2_0, _outer@1_1, _limit@0_0) ->
    _ = begin
            _visits@2_1 = _visits@2_0 + 1,
            case _outer@1_1 < _limit@0_0 of
                true ->
                    '$patina_marker':break({continue, 0, _visits@2_1});
                false ->
                    {}
            end,
            '$patina_marker':break({break, 0,
                                    _visits@2_1 * 100 + _outer@1_1,
                                    _visits@2_1})
        end,
    '$patina_marker':continue(_visits@2_1, _outer@1_1, _limit@0_0).
'$patina_loop_11_48'(_visits@2_0, _outer@1_1, _limit@0_0) ->
    _ = begin
            case
                '$patina_loop_11_44'(_visits@2_0, _outer@1_1,
                                     _limit@0_0)
            of
                {break, _target_loop_depth@3_0, _loop_value@4_0,
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
    '$patina_marker':continue(_visits@2_1, _outer@1_1, _limit@0_0).
'$patina_loop_11_51'(_outer@1_0, _visits@2_0, _limit@0_0) ->
    _ = begin
            _outer@1_1 = _outer@1_0 + 1,
            case
                '$patina_loop_11_48'(_visits@2_0, _outer@1_1,
                                     _limit@0_0)
            of
                {break, _target_loop_depth@6_0, _loop_value@7_0,
                 _visits@2_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@6_0,
                                            _loop_value@7_0, _outer@1_1,
                                            _visits@2_1});
                {continue, 0, _visits@2_1} ->
                    '$patina_marker':continue(_outer@1_1, _visits@2_1,
                                              _limit@0_0)
            end
        end,
    '$patina_marker':continue(_outer@1_1, _visits@2_1, _limit@0_0).
continue_two_loops(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    case '$patina_loop_11_51'(_outer@1_0, _visits@2_0, _limit@0_0) of
        {break, 0, _loop_value@8_0, _outer@1_1, _visits@2_1} ->
            _loop_value@8_0
    end.
