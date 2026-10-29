-file("tests/ui/src/flow_control/loop_outer_label.pt",
      3).

-module('Elixir.Patina.LoopOuterLabel').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         break_outer_loop/0,
         break_two_loops/0,
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

break_outer_loop() ->
    _value@0_0 = 0,
    {_jump_value@2_0, _value@0_1} = fun
                                        _$patina_loop(_value@0_0) ->
                                            case [] of
                                                {'$patina_label_0',
                                                 _jump_value@1_0,
                                                 _value@0_1} ->
                                                    {_jump_value@1_0,
                                                     _value@0_1};
                                                _ ->
                                                    fun
                                                        _$patina_loop(_value@0_0) ->
                                                            _value@0_1 =
                                                                _value@0_0 + 1,
                                                            '$patina_marker':jump('$patina_label_0',
                                                                                  _value@0_1,
                                                                                  _value@0_1),
                                                            [],
                                                            _$patina_loop(_value@0_1)
                                                    end(_value@0_0),
                                                    _$patina_loop(_value@0_0)
                                            end
                                    end(_value@0_0),
    _jump_value@2_0.

break_two_loops() ->
    _value@0_0 = 0,
    {_jump_value@3_0, _value@0_1} = fun
                                        _$patina_loop(_value@0_0) ->
                                            case [] of
                                                {'$patina_label_0',
                                                 _jump_value@2_0,
                                                 _value@0_1} ->
                                                    {_jump_value@2_0,
                                                     _value@0_1};
                                                _ ->
                                                    fun
                                                        _$patina_loop(_value@0_0) ->
                                                            fun
                                                                _$patina_loop(_value@0_0) ->
                                                                    _value@0_1 =
                                                                        _value@0_0
                                                                            + 1,
                                                                    case
                                                                        _value@0_1
                                                                            =:=
                                                                            3
                                                                        of
                                                                        true ->
                                                                            _jump_value@1_0 =
                                                                                _value@0_1
                                                                                    *
                                                                                    10,
                                                                            '$patina_marker':jump('$patina_label_0',
                                                                                                  _jump_value@1_0,
                                                                                                  _value@0_1),
                                                                            [];
                                                                        _ -> {}
                                                                    end,
                                                                    _$patina_loop(_value@0_1)
                                                            end(_value@0_0),
                                                            _$patina_loop(_value@0_0)
                                                    end(_value@0_0),
                                                    _$patina_loop(_value@0_0)
                                            end
                                    end(_value@0_0),
    _jump_value@3_0.

outer_break_initializes() ->
    _value@0_0 = {},
    _value@0_1 = fun _$patina_loop(_value@0_0) ->
                         case [] of
                             {'$patina_label_0', _value@0_1} -> _value@0_1;
                             _ ->
                                 fun _$patina_loop(_value@0_0) ->
                                         _value@0_1 = 7,
                                         '$patina_marker':jump('$patina_label_0',
                                                               _value@0_1),
                                         [],
                                         _$patina_loop(_value@0_1)
                                 end(_value@0_0),
                                 _$patina_loop(_value@0_0)
                         end
                 end(_value@0_0),
    _value@0_1.
