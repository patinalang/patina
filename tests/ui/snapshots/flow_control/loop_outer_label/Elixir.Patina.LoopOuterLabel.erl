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
    begin
        {_jump_value@2_0, _value@0_1} = fun
                                            _$patina_loop(_value@0_0) ->
                                                case [] of
                                                    {'$patina_label_0',
                                                     _jump_value@1_0,
                                                     _value@0_1} ->
                                                        {_jump_value@1_0,
                                                         _value@0_1};
                                                    _ ->
                                                        _ = begin
                                                                fun
                                                                    _$patina_loop(_value@0_0) ->
                                                                        _ =
                                                                            begin
                                                                                _value@0_1 =
                                                                                    _value@0_0
                                                                                        +
                                                                                        1,
                                                                                begin
                                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                                          _value@0_1,
                                                                                                          _value@0_1),
                                                                                    []
                                                                                end
                                                                            end,
                                                                        _$patina_loop(_value@0_1)
                                                                end(_value@0_0)
                                                            end,
                                                        _$patina_loop(_value@0_0)
                                                end
                                        end(_value@0_0),
        _jump_value@2_0
    end.

break_two_loops() ->
    _value@0_0 = 0,
    begin
        {_jump_value@3_0, _value@0_1} = fun
                                            _$patina_loop(_value@0_0) ->
                                                case [] of
                                                    {'$patina_label_0',
                                                     _jump_value@2_0,
                                                     _value@0_1} ->
                                                        {_jump_value@2_0,
                                                         _value@0_1};
                                                    _ ->
                                                        _ = begin
                                                                fun
                                                                    _$patina_loop(_value@0_0) ->
                                                                        _ =
                                                                            begin
                                                                                fun
                                                                                    _$patina_loop(_value@0_0) ->
                                                                                        _ =
                                                                                            begin
                                                                                                _value@0_1 =
                                                                                                    _value@0_0
                                                                                                        +
                                                                                                        1,
                                                                                                case
                                                                                                    _value@0_1
                                                                                                        =:=
                                                                                                        3
                                                                                                    of
                                                                                                    true ->
                                                                                                        begin
                                                                                                            _jump_value@1_0 =
                                                                                                                _value@0_1
                                                                                                                    *
                                                                                                                    10,
                                                                                                            begin
                                                                                                                '$patina_marker':jump('$patina_label_0',
                                                                                                                                      _jump_value@1_0,
                                                                                                                                      _value@0_1),
                                                                                                                []
                                                                                                            end
                                                                                                        end;
                                                                                                    _ ->
                                                                                                        {}
                                                                                                end
                                                                                            end,
                                                                                        _$patina_loop(_value@0_1)
                                                                                end(_value@0_0)
                                                                            end,
                                                                        _$patina_loop(_value@0_0)
                                                                end(_value@0_0)
                                                            end,
                                                        _$patina_loop(_value@0_0)
                                                end
                                        end(_value@0_0),
        _jump_value@3_0
    end.

outer_break_initializes() ->
    begin _value@0_0 = {} end,
    begin
        _value@0_1 = fun _$patina_loop(_value@0_0) ->
                             case [] of
                                 {'$patina_label_0', _value@0_1} -> _value@0_1;
                                 _ ->
                                     _ = begin
                                             fun _$patina_loop(_value@0_0) ->
                                                     _ = begin
                                                             _value@0_1 = 7,
                                                             begin
                                                                 '$patina_marker':jump('$patina_label_0',
                                                                                       _value@0_1),
                                                                 []
                                                             end
                                                         end,
                                                     _$patina_loop(_value@0_1)
                                             end(_value@0_0)
                                         end,
                                     _$patina_loop(_value@0_0)
                             end
                     end(_value@0_0),
        {}
    end,
    _value@0_1.
