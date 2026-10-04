-file("tests/ui/src/flow_control/loop_outer_label.pt",
      1).

-module('Elixir.Patina.LoopOuterLabel').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '$patina_loop_4_15'/1,
         '$patina_loop_4_18'/1,
         break_outer_loop/0,
         '$patina_loop_5_30'/1,
         '$patina_loop_5_34'/1,
         '$patina_loop_5_37'/1,
         break_two_loops/0,
         '$patina_loop_6_11'/1,
         '$patina_loop_6_14'/1,
         outer_break_initializes/0]).

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

'$patina_loop_4_15'(_value@0_0) ->
    _ = begin
            _value@0_1 = _value@0_0 + 1,
            '$patina_marker':break({break,
                                    0,
                                    _value@0_1,
                                    _value@0_1}),
            {}
        end,
    '$patina_marker':continue(_value@0_1).

'$patina_loop_4_18'(_value@0_0) ->
    _ = begin
            case '$patina_loop_4_15'(_value@0_0) of
                {break,
                 _target_loop_depth@1_0,
                 _loop_value@2_0,
                 _value@0_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@1_0,
                                            _loop_value@2_0,
                                            _value@0_1})
            end
        end,
    '$patina_marker':continue(_value@0_1).

break_outer_loop() ->
    _value@0_0 = 0,
    case '$patina_loop_4_18'(_value@0_0) of
        {break, 0, _loop_value@3_0, _value@0_1} ->
            _loop_value@3_0
    end.

'$patina_loop_5_30'(_value@0_0) ->
    _ = begin
            _value@0_1 = _value@0_0 + 1,
            case _value@0_1 =:= 3 of
                true ->
                    '$patina_marker':break({break,
                                            0,
                                            _value@0_1 * 10,
                                            _value@0_1}),
                    {};
                false -> {}
            end
        end,
    '$patina_marker':continue(_value@0_1).

'$patina_loop_5_34'(_value@0_0) ->
    _ = begin
            case '$patina_loop_5_30'(_value@0_0) of
                {break,
                 _target_loop_depth@1_0,
                 _loop_value@2_0,
                 _value@0_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@1_0,
                                            _loop_value@2_0,
                                            _value@0_1})
            end
        end,
    '$patina_marker':continue(_value@0_1).

'$patina_loop_5_37'(_value@0_0) ->
    _ = begin
            case '$patina_loop_5_34'(_value@0_0) of
                {break,
                 _target_loop_depth@3_0,
                 _loop_value@4_0,
                 _value@0_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@3_0,
                                            _loop_value@4_0,
                                            _value@0_1})
            end
        end,
    '$patina_marker':continue(_value@0_1).

break_two_loops() ->
    _value@0_0 = 0,
    case '$patina_loop_5_37'(_value@0_0) of
        {break, 0, _loop_value@5_0, _value@0_1} ->
            _loop_value@5_0
    end.

'$patina_loop_6_11'(_value@0_0) ->
    _ = begin
            _value@0_1 = 7,
            '$patina_marker':break({break, 0, {}, _value@0_1}),
            {}
        end,
    '$patina_marker':continue(_value@0_1).

'$patina_loop_6_14'(_value@0_0) ->
    _ = begin
            case '$patina_loop_6_11'(_value@0_0) of
                {break,
                 _target_loop_depth@1_0,
                 _loop_value@2_0,
                 _value@0_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@1_0,
                                            _loop_value@2_0,
                                            _value@0_1})
            end
        end,
    '$patina_marker':continue(_value@0_1).

outer_break_initializes() ->
    begin _value@0_0 = {} end,
    case '$patina_loop_6_14'(_value@0_0) of
        {break, 0, _loop_value@3_0, _value@0_1} ->
            _loop_value@3_0
    end,
    _value@0_1.
