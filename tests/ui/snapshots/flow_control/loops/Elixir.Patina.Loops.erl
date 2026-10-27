-file("tests/ui/src/flow_control/loops.pt", 1).

-module('Elixir.Patina.Loops').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         normal_continue/1,
         loop_value/1,
         zero_parameter_loop/0,
         initialize_in_loop/0,
         while_sum/2,
         nested/2,
         return_from_loop/1,
         return_from_nested/2,
         return_from_nested_uneven_rebinds/2,
         continue_outer/1,
         continue_through_middle/1,
         cast_loop_value/0,
         helper_names/1,
         trait_helper_names/1,
         continue_and_break_in_try/1,
         break_in_catch/2,
         continue_in_catch/2,
         break_and_continue_in_after/1,
         return_in_try_in_loop/1,
         return_in_after_in_loop/1,
         continue_outer_from_try/1,
         loop_in_try/1]).

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

normal_continue(_limit@0_0) ->
    _count@1_0 = 0,
    _total@2_0 = 0,
    begin
        {_count@1_1, _total@2_1} = fun _$patina_loop(_count@1_0,
                                                     _total@2_0) ->
                                           case [] of
                                               {'$patina_label_1',
                                                _count@1_2,
                                                _total@2_2} ->
                                                   {_count@1_2, _total@2_2};
                                               _ ->
                                                   _ = case [] of
                                                           {'$patina_label_0',
                                                            _count@1_1,
                                                            _total@2_1} ->
                                                               {};
                                                           _ ->
                                                               _count@1_1 =
                                                                   _count@1_0 +
                                                                       1,
                                                               case _count@1_1
                                                                        rem 2
                                                                        =:= 0
                                                                   of
                                                                   true ->
                                                                       begin
                                                                           '$patina_marker':jump('$patina_label_0',
                                                                                                 _count@1_1,
                                                                                                 _total@2_0),
                                                                           []
                                                                       end;
                                                                   _ -> {}
                                                               end,
                                                               _total@2_1 =
                                                                   _total@2_0 +
                                                                       _count@1_1,
                                                               case _count@1_1
                                                                        >=
                                                                        _limit@0_0
                                                                   of
                                                                   true ->
                                                                       begin
                                                                           '$patina_marker':jump('$patina_label_1',
                                                                                                 _count@1_1,
                                                                                                 _total@2_1),
                                                                           []
                                                                       end;
                                                                   _ -> {}
                                                               end
                                                       end,
                                                   _$patina_loop(_count@1_1,
                                                                 _total@2_1)
                                           end
                                   end(_count@1_0, _total@2_0),
        {}
    end,
    _total@2_1 * 100 + _count@1_1.

loop_value(_target@0_0) ->
    _counter@1_0 = 0,
    _result@5_0 = begin
                      {_jump_value@4_0, _counter@1_1} = fun
                                                            _$patina_loop(_counter@1_0) ->
                                                                case [] of
                                                                    {'$patina_label_0',
                                                                     _jump_value@3_0,
                                                                     _counter@1_2} ->
                                                                        {_jump_value@3_0,
                                                                         _counter@1_2};
                                                                    _ ->
                                                                        _ =
                                                                            begin
                                                                                _counter@1_1 =
                                                                                    _counter@1_0
                                                                                        +
                                                                                        1,
                                                                                case
                                                                                    _counter@1_1
                                                                                        =:=
                                                                                        _target@0_0
                                                                                    of
                                                                                    true ->
                                                                                        begin
                                                                                            _jump_value@2_0 =
                                                                                                _counter@1_1
                                                                                                    *
                                                                                                    2,
                                                                                            begin
                                                                                                '$patina_marker':jump('$patina_label_0',
                                                                                                                      _jump_value@2_0,
                                                                                                                      _counter@1_1),
                                                                                                []
                                                                                            end
                                                                                        end;
                                                                                    _ ->
                                                                                        {}
                                                                                end
                                                                            end,
                                                                        _$patina_loop(_counter@1_1)
                                                                end
                                                        end(_counter@1_0),
                      _jump_value@4_0
                  end,
    _result@5_0 * 10 + _counter@1_1.

zero_parameter_loop() ->
    begin
        _jump_value@1_0 = fun _$patina_loop() ->
                                  case [] of
                                      {'$patina_label_0', _jump_value@0_0} ->
                                          _jump_value@0_0;
                                      _ ->
                                          _ = begin
                                                  begin
                                                      '$patina_marker':jump('$patina_label_0',
                                                                            7),
                                                      []
                                                  end
                                              end,
                                          _$patina_loop()
                                  end
                          end(),
        _jump_value@1_0
    end.

initialize_in_loop() ->
    begin _value@0_0 = {} end,
    begin
        _value@0_1 = fun _$patina_loop(_value@0_0) ->
                             case [] of
                                 {'$patina_label_0', _value@0_2} -> _value@0_2;
                                 _ ->
                                     _ = begin
                                             _value@0_1 = 7,
                                             begin
                                                 '$patina_marker':jump('$patina_label_0',
                                                                       _value@0_1),
                                                 []
                                             end
                                         end,
                                     _$patina_loop(_value@0_1)
                             end
                     end(_value@0_0),
        {}
    end,
    _value@0_1.

while_sum(_start@0_0, _limit@1_0) ->
    _n@2_0 = _start@0_0,
    _total@3_0 = 0,
    begin
        {_n@2_1, _total@3_1} = fun _$patina_loop(_n@2_0,
                                                 _total@3_0) ->
                                       case [] of
                                           {'$patina_label_0',
                                            _n@2_2,
                                            _total@3_2} ->
                                               {_n@2_2, _total@3_2};
                                           _ ->
                                               _ = begin
                                                       case _n@2_0 < _limit@1_0
                                                           of
                                                           true ->
                                                               _total@3_1 =
                                                                   _total@3_0 +
                                                                       _n@2_0,
                                                               _n@2_1 = _n@2_0 +
                                                                            1,
                                                               {};
                                                           _ ->
                                                               _branch_value@4_0 =
                                                                   begin
                                                                       begin
                                                                           '$patina_marker':jump('$patina_label_0',
                                                                                                 _n@2_0,
                                                                                                 _total@3_0),
                                                                           []
                                                                       end
                                                                   end,
                                                               _total@3_1 =
                                                                   _total@3_0,
                                                               _n@2_1 = _n@2_0,
                                                               _branch_value@4_0
                                                       end
                                                   end,
                                               _$patina_loop(_n@2_1, _total@3_1)
                                       end
                               end(_n@2_0, _total@3_0),
        {}
    end,
    _total@3_1 * 100 + _n@2_1.

nested(_outer_limit@0_0, _inner_limit@1_0) ->
    _outer@2_0 = 0,
    _visits@3_0 = 0,
    begin
        {_outer@2_1, _visits@3_1} = fun
                                        _$patina_loop(_outer@2_0,
                                                      _visits@3_0) ->
                                            case [] of
                                                {'$patina_label_2',
                                                 _outer@2_2,
                                                 _visits@3_2} ->
                                                    {_outer@2_2, _visits@3_2};
                                                _ ->
                                                    _ = begin
                                                            _outer@2_1 =
                                                                _outer@2_0 + 1,
                                                            _inner@4_0 = 0,
                                                            begin
                                                                {_inner@4_1,
                                                                 _visits@3_1} =
                                                                    fun
                                                                        _$patina_loop(_inner@4_0,
                                                                                      _visits@3_0) ->
                                                                            case
                                                                                []
                                                                                of
                                                                                {'$patina_label_1',
                                                                                 _inner@4_2,
                                                                                 _visits@3_2} ->
                                                                                    {_inner@4_2,
                                                                                     _visits@3_2};
                                                                                _ ->
                                                                                    _ =
                                                                                        case
                                                                                            []
                                                                                            of
                                                                                            {'$patina_label_0',
                                                                                             _inner@4_1,
                                                                                             _visits@3_1} ->
                                                                                                {};
                                                                                            _ ->
                                                                                                _inner@4_1 =
                                                                                                    _inner@4_0
                                                                                                        +
                                                                                                        1,
                                                                                                case
                                                                                                    _inner@4_1
                                                                                                        <
                                                                                                        _inner_limit@1_0
                                                                                                    of
                                                                                                    true ->
                                                                                                        begin
                                                                                                            '$patina_marker':jump('$patina_label_0',
                                                                                                                                  _inner@4_1,
                                                                                                                                  _visits@3_0),
                                                                                                            []
                                                                                                        end;
                                                                                                    _ ->
                                                                                                        {}
                                                                                                end,
                                                                                                _visits@3_1 =
                                                                                                    _visits@3_0
                                                                                                        +
                                                                                                        (_outer@2_1
                                                                                                             *
                                                                                                             10
                                                                                                             +
                                                                                                             _inner@4_1),
                                                                                                begin
                                                                                                    '$patina_marker':jump('$patina_label_1',
                                                                                                                          _inner@4_1,
                                                                                                                          _visits@3_1),
                                                                                                    []
                                                                                                end
                                                                                        end,
                                                                                    _$patina_loop(_inner@4_1,
                                                                                                  _visits@3_1)
                                                                            end
                                                                    end(_inner@4_0,
                                                                        _visits@3_0),
                                                                {}
                                                            end,
                                                            case _outer@2_1 >=
                                                                     _outer_limit@0_0
                                                                of
                                                                true ->
                                                                    begin
                                                                        '$patina_marker':jump('$patina_label_2',
                                                                                              _outer@2_1,
                                                                                              _visits@3_1),
                                                                        []
                                                                    end;
                                                                _ -> {}
                                                            end
                                                        end,
                                                    _$patina_loop(_outer@2_1,
                                                                  _visits@3_1)
                                            end
                                    end(_outer@2_0, _visits@3_0),
        {}
    end,
    _visits@3_1 * 100 + _outer@2_1.

return_from_loop(_stop_at@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@3_0} -> _jump_value@3_0;
        _ ->
            _n@1_0 = 0,
            begin
                _n@1_1 = fun _$patina_loop(_n@1_0) ->
                                 case [] of
                                     {'$patina_label_1', _n@1_2} -> _n@1_2;
                                     _ ->
                                         _ = begin
                                                 _n@1_1 = _n@1_0 + 1,
                                                 case _n@1_1 =:= _stop_at@0_0 of
                                                     true ->
                                                         begin
                                                             _jump_value@2_0 =
                                                                 _n@1_1 * 10,
                                                             begin
                                                                 '$patina_marker':jump('$patina_label_0',
                                                                                       _jump_value@2_0),
                                                                 []
                                                             end
                                                         end;
                                                     _ -> {}
                                                 end,
                                                 case _n@1_1 >= 10 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':jump('$patina_label_1',
                                                                                   _n@1_1),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end
                                             end,
                                         _$patina_loop(_n@1_1)
                                 end
                         end(_n@1_0),
                {}
            end,
            -1
    end.

return_from_nested(_outer_stop@0_0, _inner_stop@1_0) ->
    case [] of
        {'$patina_label_0', _jump_value@5_0} -> _jump_value@5_0;
        _ ->
            _outer@2_0 = 0,
            begin
                _outer@2_1 = fun _$patina_loop(_outer@2_0) ->
                                     case [] of
                                         {'$patina_label_2', _outer@2_2} ->
                                             _outer@2_2;
                                         _ ->
                                             _ = begin
                                                     _outer@2_1 = _outer@2_0 +
                                                                      1,
                                                     _inner@3_0 = 0,
                                                     begin
                                                         _inner@3_1 = fun
                                                                          _$patina_loop(_inner@3_0) ->
                                                                              case
                                                                                  []
                                                                                  of
                                                                                  {'$patina_label_1',
                                                                                   _inner@3_2} ->
                                                                                      _inner@3_2;
                                                                                  _ ->
                                                                                      _ =
                                                                                          begin
                                                                                              _inner@3_1 =
                                                                                                  _inner@3_0
                                                                                                      +
                                                                                                      1,
                                                                                              case
                                                                                                  _outer@2_1
                                                                                                      =:=
                                                                                                      _outer_stop@0_0
                                                                                                      andalso
                                                                                                      _inner@3_1
                                                                                                          =:=
                                                                                                          _inner_stop@1_0
                                                                                                  of
                                                                                                  true ->
                                                                                                      begin
                                                                                                          _jump_value@4_0 =
                                                                                                              _outer@2_1
                                                                                                                  *
                                                                                                                  100
                                                                                                                  +
                                                                                                                  _inner@3_1,
                                                                                                          begin
                                                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                                                    _jump_value@4_0),
                                                                                                              []
                                                                                                          end
                                                                                                      end;
                                                                                                  _ ->
                                                                                                      {}
                                                                                              end,
                                                                                              case
                                                                                                  _inner@3_1
                                                                                                      >=
                                                                                                      3
                                                                                                  of
                                                                                                  true ->
                                                                                                      begin
                                                                                                          '$patina_marker':jump('$patina_label_1',
                                                                                                                                _inner@3_1),
                                                                                                          []
                                                                                                      end;
                                                                                                  _ ->
                                                                                                      {}
                                                                                              end
                                                                                          end,
                                                                                      _$patina_loop(_inner@3_1)
                                                                              end
                                                                      end(_inner@3_0),
                                                         {}
                                                     end,
                                                     case _outer@2_1 >= 3 of
                                                         true ->
                                                             begin
                                                                 '$patina_marker':jump('$patina_label_2',
                                                                                       _outer@2_1),
                                                                 []
                                                             end;
                                                         _ -> {}
                                                     end
                                                 end,
                                             _$patina_loop(_outer@2_1)
                                     end
                             end(_outer@2_0),
                {}
            end,
            -1
    end.

return_from_nested_uneven_rebinds(_outer_stop@0_0,
                                  _inner_stop@1_0) ->
    case [] of
        {'$patina_label_0', _jump_value@6_0} -> _jump_value@6_0;
        _ ->
            _a@2_0 = 0,
            _b@3_0 = 0,
            begin
                {_a@2_1, _b@3_1} = fun _$patina_loop(_a@2_0, _b@3_0) ->
                                           case [] of
                                               {'$patina_label_2',
                                                _a@2_2,
                                                _b@3_2} ->
                                                   {_a@2_2, _b@3_2};
                                               _ ->
                                                   _ = begin
                                                           _a@2_1 = _a@2_0 + 1,
                                                           _b@3_1 = _b@3_0 + 10,
                                                           _inner@4_0 = 0,
                                                           begin
                                                               _inner@4_1 = fun
                                                                                _$patina_loop(_inner@4_0) ->
                                                                                    case
                                                                                        []
                                                                                        of
                                                                                        {'$patina_label_1',
                                                                                         _inner@4_2} ->
                                                                                            _inner@4_2;
                                                                                        _ ->
                                                                                            _ =
                                                                                                begin
                                                                                                    _inner@4_1 =
                                                                                                        _inner@4_0
                                                                                                            +
                                                                                                            1,
                                                                                                    case
                                                                                                        _a@2_1
                                                                                                            =:=
                                                                                                            _outer_stop@0_0
                                                                                                            andalso
                                                                                                            _inner@4_1
                                                                                                                =:=
                                                                                                                _inner_stop@1_0
                                                                                                        of
                                                                                                        true ->
                                                                                                            begin
                                                                                                                _jump_value@5_0 =
                                                                                                                    _a@2_1
                                                                                                                        *
                                                                                                                        1000
                                                                                                                        +
                                                                                                                        _b@3_1
                                                                                                                            *
                                                                                                                            10
                                                                                                                        +
                                                                                                                        _inner@4_1,
                                                                                                                begin
                                                                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                                                                          _jump_value@5_0),
                                                                                                                    []
                                                                                                                end
                                                                                                            end;
                                                                                                        _ ->
                                                                                                            {}
                                                                                                    end,
                                                                                                    case
                                                                                                        _inner@4_1
                                                                                                            >=
                                                                                                            3
                                                                                                        of
                                                                                                        true ->
                                                                                                            begin
                                                                                                                '$patina_marker':jump('$patina_label_1',
                                                                                                                                      _inner@4_1),
                                                                                                                []
                                                                                                            end;
                                                                                                        _ ->
                                                                                                            {}
                                                                                                    end
                                                                                                end,
                                                                                            _$patina_loop(_inner@4_1)
                                                                                    end
                                                                            end(_inner@4_0),
                                                               {}
                                                           end,
                                                           case _a@2_1 >= 3 of
                                                               true ->
                                                                   begin
                                                                       '$patina_marker':jump('$patina_label_2',
                                                                                             _a@2_1,
                                                                                             _b@3_1),
                                                                       []
                                                                   end;
                                                               _ -> {}
                                                           end
                                                       end,
                                                   _$patina_loop(_a@2_1, _b@3_1)
                                           end
                                   end(_a@2_0, _b@3_0),
                {}
            end,
            -1
    end.

continue_outer(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    begin
        {_outer@1_1, _visits@2_1} = fun
                                        _$patina_loop(_outer@1_0,
                                                      _visits@2_0) ->
                                            case [] of
                                                {'$patina_label_1',
                                                 _outer@1_2,
                                                 _visits@2_2} ->
                                                    {_outer@1_2, _visits@2_2};
                                                _ ->
                                                    _ = case [] of
                                                            {'$patina_label_0',
                                                             _outer@1_1,
                                                             _visits@2_1} ->
                                                                {};
                                                            _ ->
                                                                _branch_value@3_0 =
                                                                    begin
                                                                        _outer@1_1 =
                                                                            _outer@1_0
                                                                                +
                                                                                1,
                                                                        fun
                                                                            _$patina_loop(_visits@2_0) ->
                                                                                _ =
                                                                                    begin
                                                                                        _visits@2_1 =
                                                                                            _visits@2_0
                                                                                                +
                                                                                                1,
                                                                                        case
                                                                                            _outer@1_1
                                                                                                <
                                                                                                _limit@0_0
                                                                                            of
                                                                                            true ->
                                                                                                begin
                                                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                                                          _outer@1_1,
                                                                                                                          _visits@2_1),
                                                                                                    []
                                                                                                end;
                                                                                            _ ->
                                                                                                {}
                                                                                        end,
                                                                                        begin
                                                                                            '$patina_marker':jump('$patina_label_1',
                                                                                                                  _outer@1_1,
                                                                                                                  _visits@2_1),
                                                                                            []
                                                                                        end
                                                                                    end,
                                                                                _$patina_loop(_visits@2_1)
                                                                        end(_visits@2_0)
                                                                    end,
                                                                _visits@2_1 =
                                                                    _visits@2_0,
                                                                _branch_value@3_0
                                                        end,
                                                    _$patina_loop(_outer@1_1,
                                                                  _visits@2_1)
                                            end
                                    end(_outer@1_0, _visits@2_0),
        {}
    end,
    _visits@2_1 * 100 + _outer@1_1.

continue_through_middle(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    begin
        {_outer@1_1, _visits@2_1} = fun
                                        _$patina_loop(_outer@1_0,
                                                      _visits@2_0) ->
                                            case [] of
                                                {'$patina_label_1',
                                                 _outer@1_2,
                                                 _visits@2_2} ->
                                                    {_outer@1_2, _visits@2_2};
                                                _ ->
                                                    _ = case [] of
                                                            {'$patina_label_0',
                                                             _outer@1_1,
                                                             _visits@2_1} ->
                                                                {};
                                                            _ ->
                                                                _branch_value@3_0 =
                                                                    begin
                                                                        _outer@1_1 =
                                                                            _outer@1_0
                                                                                +
                                                                                1,
                                                                        fun
                                                                            _$patina_loop(_visits@2_0) ->
                                                                                _ =
                                                                                    begin
                                                                                        fun
                                                                                            _$patina_loop(_visits@2_0) ->
                                                                                                _ =
                                                                                                    begin
                                                                                                        _visits@2_1 =
                                                                                                            _visits@2_0
                                                                                                                +
                                                                                                                1,
                                                                                                        case
                                                                                                            _outer@1_1
                                                                                                                <
                                                                                                                _limit@0_0
                                                                                                            of
                                                                                                            true ->
                                                                                                                begin
                                                                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                                                                          _outer@1_1,
                                                                                                                                          _visits@2_1),
                                                                                                                    []
                                                                                                                end;
                                                                                                            _ ->
                                                                                                                {}
                                                                                                        end,
                                                                                                        begin
                                                                                                            '$patina_marker':jump('$patina_label_1',
                                                                                                                                  _outer@1_1,
                                                                                                                                  _visits@2_1),
                                                                                                            []
                                                                                                        end
                                                                                                    end,
                                                                                                _$patina_loop(_visits@2_1)
                                                                                        end(_visits@2_0)
                                                                                    end,
                                                                                _$patina_loop(_visits@2_0)
                                                                        end(_visits@2_0)
                                                                    end,
                                                                _visits@2_1 =
                                                                    _visits@2_0,
                                                                _branch_value@3_0
                                                        end,
                                                    _$patina_loop(_outer@1_1,
                                                                  _visits@2_1)
                                            end
                                    end(_outer@1_0, _visits@2_0),
        {}
    end,
    _visits@2_1 * 100 + _outer@1_1.

cast_loop_value() ->
    _value@2_0 = begin
                     _jump_value@1_0 = fun _$patina_loop() ->
                                               case [] of
                                                   {'$patina_label_0',
                                                    _jump_value@0_0} ->
                                                       _jump_value@0_0;
                                                   _ ->
                                                       _ = begin
                                                               begin
                                                                   '$patina_marker':jump('$patina_label_0',
                                                                                         1),
                                                                   []
                                                               end
                                                           end,
                                                       _$patina_loop()
                                               end
                                       end(),
                     _jump_value@1_0
                 end,
    _value@2_0.

helper_names(_limit@0_0) ->
    _total@1_0 = 0,
    _i@2_0 = 0,
    begin
        {_i@2_1, _total@1_1} = fun _$patina_loop(_i@2_0,
                                                 _total@1_0) ->
                                       case [] of
                                           {'$patina_label_0',
                                            _i@2_2,
                                            _total@1_2} ->
                                               {_i@2_2, _total@1_2};
                                           _ ->
                                               _ = begin
                                                       case _i@2_0 < _limit@0_0
                                                           of
                                                           true ->
                                                               _total@1_1 =
                                                                   _total@1_0 +
                                                                       1,
                                                               _i@2_1 = _i@2_0 +
                                                                            1,
                                                               {};
                                                           _ ->
                                                               _branch_value@3_0 =
                                                                   begin
                                                                       begin
                                                                           '$patina_marker':jump('$patina_label_0',
                                                                                                 _i@2_0,
                                                                                                 _total@1_0),
                                                                           []
                                                                       end
                                                                   end,
                                                               _total@1_1 =
                                                                   _total@1_0,
                                                               _i@2_1 = _i@2_0,
                                                               _branch_value@3_0
                                                       end
                                                   end,
                                               _$patina_loop(_i@2_1, _total@1_1)
                                       end
                               end(_i@2_0, _total@1_0),
        {}
    end,
    _j@4_0 = 0,
    begin
        {_j@4_1, _total@1_2} = fun _$patina_loop(_j@4_0,
                                                 _total@1_1) ->
                                       case [] of
                                           {'$patina_label_3',
                                            _j@4_2,
                                            _total@1_4} ->
                                               {_j@4_2, _total@1_4};
                                           _ ->
                                               _ = begin
                                                       case _j@4_0 < _limit@0_0
                                                           of
                                                           true ->
                                                               _k@5_0 = 0,
                                                               begin
                                                                   {_k@5_1,
                                                                    _total@1_2} =
                                                                       fun
                                                                           _$patina_loop(_k@5_0,
                                                                                         _total@1_1) ->
                                                                               case
                                                                                   []
                                                                                   of
                                                                                   {'$patina_label_1',
                                                                                    _k@5_2,
                                                                                    _total@1_3} ->
                                                                                       {_k@5_2,
                                                                                        _total@1_3};
                                                                                   _ ->
                                                                                       _ =
                                                                                           begin
                                                                                               case
                                                                                                   _k@5_0
                                                                                                       <
                                                                                                       _limit@0_0
                                                                                                   of
                                                                                                   true ->
                                                                                                       _total@1_2 =
                                                                                                           _total@1_1
                                                                                                               +
                                                                                                               10,
                                                                                                       _k@5_1 =
                                                                                                           _k@5_0
                                                                                                               +
                                                                                                               1,
                                                                                                       {};
                                                                                                   _ ->
                                                                                                       _branch_value@6_0 =
                                                                                                           begin
                                                                                                               begin
                                                                                                                   '$patina_marker':jump('$patina_label_1',
                                                                                                                                         _k@5_0,
                                                                                                                                         _total@1_1),
                                                                                                                   []
                                                                                                               end
                                                                                                           end,
                                                                                                       _total@1_2 =
                                                                                                           _total@1_1,
                                                                                                       _k@5_1 =
                                                                                                           _k@5_0,
                                                                                                       _branch_value@6_0
                                                                                               end
                                                                                           end,
                                                                                       _$patina_loop(_k@5_1,
                                                                                                     _total@1_2)
                                                                               end
                                                                       end(_k@5_0,
                                                                           _total@1_1),
                                                                   {}
                                                               end,
                                                               _l@7_0 = 0,
                                                               begin
                                                                   {_l@7_1,
                                                                    _total@1_3} =
                                                                       fun
                                                                           _$patina_loop(_l@7_0,
                                                                                         _total@1_2) ->
                                                                               case
                                                                                   []
                                                                                   of
                                                                                   {'$patina_label_2',
                                                                                    _l@7_2,
                                                                                    _total@1_4} ->
                                                                                       {_l@7_2,
                                                                                        _total@1_4};
                                                                                   _ ->
                                                                                       _ =
                                                                                           begin
                                                                                               case
                                                                                                   _l@7_0
                                                                                                       <
                                                                                                       _limit@0_0
                                                                                                   of
                                                                                                   true ->
                                                                                                       _total@1_3 =
                                                                                                           _total@1_2
                                                                                                               +
                                                                                                               100,
                                                                                                       _l@7_1 =
                                                                                                           _l@7_0
                                                                                                               +
                                                                                                               1,
                                                                                                       {};
                                                                                                   _ ->
                                                                                                       _branch_value@8_0 =
                                                                                                           begin
                                                                                                               begin
                                                                                                                   '$patina_marker':jump('$patina_label_2',
                                                                                                                                         _l@7_0,
                                                                                                                                         _total@1_2),
                                                                                                                   []
                                                                                                               end
                                                                                                           end,
                                                                                                       _total@1_3 =
                                                                                                           _total@1_2,
                                                                                                       _l@7_1 =
                                                                                                           _l@7_0,
                                                                                                       _branch_value@8_0
                                                                                               end
                                                                                           end,
                                                                                       _$patina_loop(_l@7_1,
                                                                                                     _total@1_3)
                                                                               end
                                                                       end(_l@7_0,
                                                                           _total@1_2),
                                                                   {}
                                                               end,
                                                               _j@4_1 = _j@4_0 +
                                                                            1,
                                                               {};
                                                           _ ->
                                                               _branch_value@9_0 =
                                                                   begin
                                                                       begin
                                                                           '$patina_marker':jump('$patina_label_3',
                                                                                                 _j@4_0,
                                                                                                 _total@1_1),
                                                                           []
                                                                       end
                                                                   end,
                                                               _total@1_3 =
                                                                   _total@1_1,
                                                               _j@4_1 = _j@4_0,
                                                               _branch_value@9_0
                                                       end
                                                   end,
                                               _$patina_loop(_j@4_1, _total@1_3)
                                       end
                               end(_j@4_0, _total@1_1),
        {}
    end,
    _f@13_0 = fun (_n@10_0) ->
                      _m@11_0 = 0,
                      begin
                          _m@11_1 = fun _$patina_loop(_m@11_0) ->
                                            case [] of
                                                {'$patina_label_4', _m@11_2} ->
                                                    _m@11_2;
                                                _ ->
                                                    _ = begin
                                                            case _m@11_0 <
                                                                     _n@10_0
                                                                of
                                                                true ->
                                                                    _m@11_1 =
                                                                        _m@11_0
                                                                            + 1,
                                                                    {};
                                                                _ ->
                                                                    _branch_value@12_0 =
                                                                        begin
                                                                            begin
                                                                                '$patina_marker':jump('$patina_label_4',
                                                                                                      _m@11_0),
                                                                                []
                                                                            end
                                                                        end,
                                                                    _m@11_1 =
                                                                        _m@11_0,
                                                                    _branch_value@12_0
                                                            end
                                                        end,
                                                    _$patina_loop(_m@11_1)
                                            end
                                    end(_m@11_0),
                          {}
                      end,
                      _m@11_1 * 1000
              end,
    _total@1_2 + _f@13_0(_limit@0_0).

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

note(_value@0_0) ->
    _notes@1_0 = begin erlang:get(notes) end,
    erlang:put(notes, [_value@0_0 | _notes@1_0]),
    {}.

continue_and_break_in_try(_limit@0_0) ->
    _n@1_0 = 0,
    begin
        {_jump_value@4_0, _n@1_1} = fun _$patina_loop(_n@1_0) ->
                                            case [] of
                                                {'$patina_label_1',
                                                 _jump_value@3_0,
                                                 _n@1_2} ->
                                                    {_jump_value@3_0, _n@1_2};
                                                _ ->
                                                    _ = case [] of
                                                            {'$patina_label_0',
                                                             _n@1_1} ->
                                                                {};
                                                            _ ->
                                                                _n@1_1 = _n@1_0
                                                                             +
                                                                             1,
                                                                try case _n@1_1
                                                                             <
                                                                             _limit@0_0
                                                                        of
                                                                        true ->
                                                                            begin
                                                                                '$patina_marker':jump('$patina_label_0',
                                                                                                      _n@1_1),
                                                                                []
                                                                            end;
                                                                        _ -> {}
                                                                    end,
                                                                    case _n@1_1
                                                                             >=
                                                                             _limit@0_0
                                                                        of
                                                                        true ->
                                                                            begin
                                                                                _jump_value@2_0 =
                                                                                    _n@1_1
                                                                                        *
                                                                                        10,
                                                                                begin
                                                                                    '$patina_marker':jump('$patina_label_1',
                                                                                                          _jump_value@2_0,
                                                                                                          _n@1_1),
                                                                                    []
                                                                                end
                                                                            end;
                                                                        _ -> {}
                                                                    end
                                                                after
                                                                    note(_n@1_1)
                                                                end
                                                        end,
                                                    _$patina_loop(_n@1_1)
                                            end
                                    end(_n@1_0),
        _jump_value@4_0
    end.

break_in_catch(_limit@0_0, _f@1_0) ->
    _n@2_0 = 0,
    begin
        {_jump_value@4_0, _n@2_1} = fun _$patina_loop(_n@2_0) ->
                                            case [] of
                                                {'$patina_label_0',
                                                 _jump_value@3_0,
                                                 _n@2_2} ->
                                                    {_jump_value@3_0, _n@2_2};
                                                _ ->
                                                    _ = begin
                                                            _n@2_1 = _n@2_0 + 1,
                                                            try _f@1_0(_n@2_1),
                                                                {}
                                                            catch
                                                                error:_ ->
                                                                    begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _n@2_1,
                                                                                              _n@2_1),
                                                                        []
                                                                    end
                                                            after
                                                                note(_n@2_1)
                                                            end,
                                                            case _n@2_1 >=
                                                                     _limit@0_0
                                                                of
                                                                true ->
                                                                    begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              -1,
                                                                                              _n@2_1),
                                                                        []
                                                                    end;
                                                                _ -> {}
                                                            end
                                                        end,
                                                    _$patina_loop(_n@2_1)
                                            end
                                    end(_n@2_0),
        _jump_value@4_0
    end.

continue_in_catch(_limit@0_0, _f@1_0) ->
    _n@2_0 = 0,
    _passed@3_0 = 0,
    begin
        {_jump_value@5_0, _n@2_1, _passed@3_1} = fun
                                                     _$patina_loop(_n@2_0,
                                                                   _passed@3_0) ->
                                                         case [] of
                                                             {'$patina_label_0',
                                                              _jump_value@4_0,
                                                              _n@2_2,
                                                              _passed@3_2} ->
                                                                 {_jump_value@4_0,
                                                                  _n@2_2,
                                                                  _passed@3_2};
                                                             _ ->
                                                                 _ = case [] of
                                                                         {'$patina_label_1',
                                                                          _n@2_1,
                                                                          _passed@3_1} ->
                                                                             {};
                                                                         _ ->
                                                                             _n@2_1 =
                                                                                 _n@2_0
                                                                                     +
                                                                                     1,
                                                                             case
                                                                                 _n@2_1
                                                                                     >
                                                                                     _limit@0_0
                                                                                 of
                                                                                 true ->
                                                                                     begin
                                                                                         '$patina_marker':jump('$patina_label_0',
                                                                                                               _passed@3_0,
                                                                                                               _n@2_1,
                                                                                                               _passed@3_0),
                                                                                         []
                                                                                     end;
                                                                                 _ ->
                                                                                     {}
                                                                             end,
                                                                             try
                                                                                 _f@1_0(_n@2_1),
                                                                                 {}
                                                                             catch
                                                                                 error:_ ->
                                                                                     begin
                                                                                         '$patina_marker':jump('$patina_label_1',
                                                                                                               _n@2_1,
                                                                                                               _passed@3_0),
                                                                                         []
                                                                                     end
                                                                             after
                                                                                 note(_n@2_1)
                                                                             end,
                                                                             _passed@3_1 =
                                                                                 _passed@3_0
                                                                                     +
                                                                                     1,
                                                                             {}
                                                                     end,
                                                                 _$patina_loop(_n@2_1,
                                                                               _passed@3_1)
                                                         end
                                                 end(_n@2_0, _passed@3_0),
        _jump_value@5_0
    end.

break_and_continue_in_after(_limit@0_0) ->
    _n@1_0 = 0,
    _odd@2_0 = 0,
    begin
        {_jump_value@5_0, _n@1_1, _odd@2_1} = fun
                                                  _$patina_loop(_n@1_0,
                                                                _odd@2_0) ->
                                                      case [] of
                                                          {'$patina_label_0',
                                                           _jump_value@4_0,
                                                           _n@1_2,
                                                           _odd@2_2} ->
                                                              {_jump_value@4_0,
                                                               _n@1_2,
                                                               _odd@2_2};
                                                          _ ->
                                                              _ = case [] of
                                                                      {'$patina_label_1',
                                                                       _n@1_1,
                                                                       _odd@2_1} ->
                                                                          {};
                                                                      _ ->
                                                                          _n@1_1 =
                                                                              _n@1_0
                                                                                  +
                                                                                  1,
                                                                          try
                                                                              note(_n@1_1)
                                                                          after
                                                                              case
                                                                                  _n@1_1
                                                                                      >=
                                                                                      _limit@0_0
                                                                                  of
                                                                                  true ->
                                                                                      begin
                                                                                          _jump_value@3_0 =
                                                                                              _n@1_1
                                                                                                  *
                                                                                                  10
                                                                                                  +
                                                                                                  _odd@2_0,
                                                                                          begin
                                                                                              '$patina_marker':jump('$patina_label_0',
                                                                                                                    _jump_value@3_0,
                                                                                                                    _n@1_1,
                                                                                                                    _odd@2_0),
                                                                                              []
                                                                                          end
                                                                                      end;
                                                                                  _ ->
                                                                                      {}
                                                                              end,
                                                                              case
                                                                                  _n@1_1
                                                                                      rem
                                                                                      2
                                                                                      =:=
                                                                                      0
                                                                                  of
                                                                                  true ->
                                                                                      begin
                                                                                          '$patina_marker':jump('$patina_label_1',
                                                                                                                _n@1_1,
                                                                                                                _odd@2_0),
                                                                                          []
                                                                                      end;
                                                                                  _ ->
                                                                                      {}
                                                                              end
                                                                          end,
                                                                          _odd@2_1 =
                                                                              _odd@2_0
                                                                                  +
                                                                                  1,
                                                                          {}
                                                                  end,
                                                              _$patina_loop(_n@1_1,
                                                                            _odd@2_1)
                                                      end
                                              end(_n@1_0, _odd@2_0),
        _jump_value@5_0
    end.

return_in_try_in_loop(_stop@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@3_0} -> _jump_value@3_0;
        _ ->
            _n@1_0 = 0,
            begin
                _n@1_1 = fun _$patina_loop(_n@1_0) ->
                                 case [] of
                                     {'$patina_label_1', _n@1_2} -> _n@1_2;
                                     _ ->
                                         _ = begin
                                                 _n@1_1 = _n@1_0 + 1,
                                                 try case _n@1_1 =:= _stop@0_0
                                                         of
                                                         true ->
                                                             begin
                                                                 _jump_value@2_0 =
                                                                     _n@1_1 *
                                                                         10,
                                                                 begin
                                                                     '$patina_marker':jump('$patina_label_0',
                                                                                           _jump_value@2_0),
                                                                     []
                                                                 end
                                                             end;
                                                         _ -> {}
                                                     end
                                                 after
                                                     note(_n@1_1)
                                                 end,
                                                 case _n@1_1 >= 5 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':jump('$patina_label_1',
                                                                                   _n@1_1),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end
                                             end,
                                         _$patina_loop(_n@1_1)
                                 end
                         end(_n@1_0),
                {}
            end,
            -1
    end.

return_in_after_in_loop(_stop@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@3_0} -> _jump_value@3_0;
        _ ->
            _n@1_0 = 0,
            begin
                _n@1_1 = fun _$patina_loop(_n@1_0) ->
                                 case [] of
                                     {'$patina_label_1', _n@1_2} -> _n@1_2;
                                     _ ->
                                         _ = begin
                                                 _n@1_1 = _n@1_0 + 1,
                                                 try note(_n@1_1) after
                                                     case _n@1_1 =:= _stop@0_0
                                                         of
                                                         true ->
                                                             begin
                                                                 _jump_value@2_0 =
                                                                     _n@1_1 *
                                                                         10,
                                                                 begin
                                                                     '$patina_marker':jump('$patina_label_0',
                                                                                           _jump_value@2_0),
                                                                     []
                                                                 end
                                                             end;
                                                         _ -> {}
                                                     end
                                                 end,
                                                 case _n@1_1 >= 5 of
                                                     true ->
                                                         begin
                                                             '$patina_marker':jump('$patina_label_1',
                                                                                   _n@1_1),
                                                             []
                                                         end;
                                                     _ -> {}
                                                 end
                                             end,
                                         _$patina_loop(_n@1_1)
                                 end
                         end(_n@1_0),
                {}
            end,
            -1
    end.

continue_outer_from_try(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    begin
        {_outer@1_1, _visits@2_1} = fun
                                        _$patina_loop(_outer@1_0,
                                                      _visits@2_0) ->
                                            case [] of
                                                {'$patina_label_1',
                                                 _outer@1_2,
                                                 _visits@2_2} ->
                                                    {_outer@1_2, _visits@2_2};
                                                _ ->
                                                    _ = case [] of
                                                            {'$patina_label_0',
                                                             _outer@1_1,
                                                             _visits@2_1} ->
                                                                {};
                                                            _ ->
                                                                _branch_value@3_0 =
                                                                    begin
                                                                        _outer@1_1 =
                                                                            _outer@1_0
                                                                                +
                                                                                1,
                                                                        fun
                                                                            _$patina_loop(_visits@2_0) ->
                                                                                _ =
                                                                                    begin
                                                                                        _visits@2_1 =
                                                                                            _visits@2_0
                                                                                                +
                                                                                                1,
                                                                                        try
                                                                                            case
                                                                                                _outer@1_1
                                                                                                    <
                                                                                                    _limit@0_0
                                                                                                of
                                                                                                true ->
                                                                                                    begin
                                                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                                                              _outer@1_1,
                                                                                                                              _visits@2_1),
                                                                                                        []
                                                                                                    end;
                                                                                                _ ->
                                                                                                    {}
                                                                                            end,
                                                                                            case
                                                                                                _outer@1_1
                                                                                                    >=
                                                                                                    _limit@0_0
                                                                                                of
                                                                                                true ->
                                                                                                    begin
                                                                                                        '$patina_marker':jump('$patina_label_1',
                                                                                                                              _outer@1_1,
                                                                                                                              _visits@2_1),
                                                                                                        []
                                                                                                    end;
                                                                                                _ ->
                                                                                                    {}
                                                                                            end
                                                                                        after
                                                                                            note(_outer@1_1)
                                                                                        end
                                                                                    end,
                                                                                _$patina_loop(_visits@2_1)
                                                                        end(_visits@2_0)
                                                                    end,
                                                                _visits@2_1 =
                                                                    _visits@2_0,
                                                                _branch_value@3_0
                                                        end,
                                                    _$patina_loop(_outer@1_1,
                                                                  _visits@2_1)
                                            end
                                    end(_outer@1_0, _visits@2_0),
        {}
    end,
    _visits@2_1 * 100 + _outer@1_1.

loop_in_try(_stop@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@6_0} -> _jump_value@6_0;
        _ ->
            _result@5_0 = try _n@1_0 = 0,
                              begin
                                  {_jump_value@4_0, _n@1_1} = fun
                                                                  _$patina_loop(_n@1_0) ->
                                                                      case [] of
                                                                          {'$patina_label_1',
                                                                           _jump_value@3_0,
                                                                           _n@1_2} ->
                                                                              {_jump_value@3_0,
                                                                               _n@1_2};
                                                                          _ ->
                                                                              _ =
                                                                                  begin
                                                                                      _n@1_1 =
                                                                                          _n@1_0
                                                                                              +
                                                                                              1,
                                                                                      case
                                                                                          _n@1_1
                                                                                              =:=
                                                                                              _stop@0_0
                                                                                          of
                                                                                          true ->
                                                                                              begin
                                                                                                  '$patina_marker':jump('$patina_label_0',
                                                                                                                        _n@1_1),
                                                                                                  []
                                                                                              end;
                                                                                          _ ->
                                                                                              {}
                                                                                      end,
                                                                                      case
                                                                                          _n@1_1
                                                                                              >=
                                                                                              5
                                                                                          of
                                                                                          true ->
                                                                                              begin
                                                                                                  _jump_value@2_0 =
                                                                                                      _n@1_1
                                                                                                          *
                                                                                                          10,
                                                                                                  begin
                                                                                                      '$patina_marker':jump('$patina_label_1',
                                                                                                                            _jump_value@2_0,
                                                                                                                            _n@1_1),
                                                                                                      []
                                                                                                  end
                                                                                              end;
                                                                                          _ ->
                                                                                              {}
                                                                                      end
                                                                                  end,
                                                                              _$patina_loop(_n@1_1)
                                                                      end
                                                              end(_n@1_0),
                                  _jump_value@4_0
                              end
                          after
                              note(0)
                          end,
            _result@5_0 + 1
    end.
