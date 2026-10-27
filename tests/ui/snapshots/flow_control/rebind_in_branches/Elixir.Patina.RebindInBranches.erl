-file("tests/ui/src/flow_control/rebind_in_branches.pt",
      1).

-module('Elixir.Patina.RebindInBranches').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         branch_result/1,
         latest_version_wins/1,
         initialize_in_branches/1,
         initialize_or_return/1,
         nested_result/0,
         nested_branches/2,
         nested_diverging_rebind/2,
         short_circuit_result/1,
         short_circuit_or_result/1,
         nested_short_circuit_result/2,
         short_circuit_local_binding/2,
         match_result/1,
         or_assign/2,
         not_and_assign/2,
         and_or_right/3,
         or_then_and/3,
         and_then_or/3,
         while_or/1]).

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

branch_result(_take_then@0_0) ->
    _value@1_0 = 10,
    _block_value@2_0 = case _take_then@0_0 of
                           true ->
                               _value@1_1 = _value@1_0 + 1,
                               100;
                           _ ->
                               _value@1_1 = _value@1_0 + 2,
                               200
                       end,
    _block_value@2_0 + _value@1_1.

latest_version_wins(_take_then@0_0) ->
    _value@1_0 = 1,
    case _take_then@0_0 of
        true ->
            _value@1_1 = 2,
            _value@1_2 = 3,
            {};
        _ ->
            _branch_value@2_0 = begin _value@1_1 = 4, {} end,
            _value@1_2 = _value@1_1,
            _branch_value@2_0
    end,
    _value@1_2.

initialize_in_branches(_take_then@0_0) ->
    begin _value@1_0 = {} end,
    case _take_then@0_0 of
        true ->
            _value@1_1 = 10,
            {};
        _ ->
            _value@1_1 = 20,
            {}
    end,
    _value@1_1.

initialize_or_return(_take_then@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@3_0} -> _jump_value@3_0;
        _ ->
            begin _value@1_0 = {} end,
            case _take_then@0_0 of
                true ->
                    _value@1_1 = 30,
                    {};
                _ ->
                    _branch_value@2_0 = begin
                                            begin
                                                '$patina_marker':jump('$patina_label_0',
                                                                      40),
                                                []
                                            end
                                        end,
                    _value@1_1 = _value@1_0,
                    _branch_value@2_0
            end,
            _value@1_1
    end.

nested_result() ->
    _value@0_0 = 10,
    _block_value@2_0 = begin
                           _inner_value@1_0 = begin
                                                  _value@0_1 = _value@0_0 + 1,
                                                  100
                                              end,
                           _value@0_2 = _value@0_1 + 2,
                           _inner_value@1_0 + 1000
                       end,
    _block_value@2_0 + _value@0_2.

nested_branches(_take_outer@0_0, _take_inner@1_0) ->
    _value@2_0 = 0,
    _block_value@4_0 = case _take_outer@0_0 of
                           true ->
                               _inner_value@3_0 = case _take_inner@1_0 of
                                                      true ->
                                                          _value@2_1 = 1,
                                                          10;
                                                      _ ->
                                                          _value@2_1 = 2,
                                                          20
                                                  end,
                               _inner_value@3_0 + 100;
                           _ ->
                               _value@2_1 = 3,
                               300
                       end,
    _block_value@4_0 + _value@2_1.

nested_diverging_rebind(_take_outer@0_0,
                        _take_inner@1_0) ->
    case [] of
        {'$patina_label_0', _jump_value@5_0} -> _jump_value@5_0;
        _ ->
            _value@2_0 = 0,
            case _take_outer@0_0 of
                true ->
                    case _take_inner@1_0 of
                        true ->
                            _value@2_1 = 1,
                            begin
                                '$patina_marker':jump('$patina_label_0',
                                                      _value@2_1),
                                []
                            end;
                        _ ->
                            _branch_value@3_0 = begin {} end,
                            _value@2_1 = _value@2_0,
                            _branch_value@3_0
                    end;
                _ ->
                    _branch_value@4_0 = begin {} end,
                    _value@2_1 = _value@2_0,
                    _branch_value@4_0
            end,
            10
    end.

short_circuit_result(_run_block@0_0) ->
    _value@1_0 = 10,
    __result@3_0 = case _run_block@0_0 of
                       true ->
                           _value@1_1 = _value@1_0 + 5,
                           true;
                       _ ->
                           _branch_value@2_0 = false,
                           _value@1_1 = _value@1_0,
                           _branch_value@2_0
                   end,
    _value@1_1.

short_circuit_or_result(_skip_block@0_0) ->
    _value@1_0 = 10,
    _result@3_0 = case _skip_block@0_0 of
                      true ->
                          _branch_value@2_0 = true,
                          _value@1_1 = _value@1_0,
                          _branch_value@2_0;
                      _ ->
                          _value@1_1 = _value@1_0 + 5,
                          false
                  end,
    case _result@3_0 of
        true -> _value@1_1 + 100;
        _ -> _value@1_1
    end.

nested_short_circuit_result(_outer@0_0, _inner@1_0) ->
    _value@2_0 = 10,
    _result@5_0 = case _outer@0_0 of
                      true ->
                          case _inner@1_0 of
                              true ->
                                  _branch_value@3_0 = true,
                                  _value@2_1 = _value@2_0,
                                  _branch_value@3_0;
                              _ ->
                                  _value@2_1 = _value@2_0 + 1,
                                  true
                          end;
                      _ ->
                          _branch_value@4_0 = false,
                          _value@2_1 = _value@2_0,
                          _branch_value@4_0
                  end,
    case _result@5_0 of
        true -> _value@2_1 + 100;
        _ -> _value@2_1
    end.

short_circuit_local_binding(_run_block@0_0,
                            _value@1_0) ->
    _run_block@0_0 andalso
        begin
            _doubled@2_0 = _value@1_0 * 2, _doubled@2_0 > 10
        end.

match_result(_take_first@0_0) ->
    _value@1_0 = 10,
    _branch@2_0 = case _take_first@0_0 of
                      true -> 'Elixir.Patina.RebindInBranches.Branch.First';
                      _ -> 'Elixir.Patina.RebindInBranches.Branch.Second'
                  end,
    _block_value@3_0 = case _branch@2_0 of
                           'Elixir.Patina.RebindInBranches.Branch.First' ->
                               _value@1_1 = _value@1_0 + 1,
                               100;
                           'Elixir.Patina.RebindInBranches.Branch.Second' ->
                               _value@1_1 = _value@1_0 + 2,
                               200
                       end,
    _block_value@3_0 + _value@1_1.

or_assign(_a@0_0, _b@1_0) ->
    _y@2_0 = 0,
    _r@4_0 = case [] of
                 {'$patina_label_0', _y@2_1} -> _y@2_1 + 10;
                 _ ->
                     case _a@0_0 of
                         true ->
                             _branch_value@3_0 = begin
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _y@2_0),
                                                     []
                                                 end,
                             _y@2_1 = _y@2_0,
                             _branch_value@3_0;
                         _ ->
                             case begin _y@2_1 = _y@2_0 + 1, _b@1_0 end of
                                 true ->
                                     '$patina_marker':jump('$patina_label_0',
                                                           _y@2_1),
                                     [];
                                 _ -> _y@2_1 + 20
                             end
                     end
             end,
    _r@4_0 * 100 + _y@2_1.

not_and_assign(_a@0_0, _b@1_0) ->
    _y@2_0 = 0,
    _r@4_0 = case [] of
                 {'$patina_label_0', _y@2_1} -> _y@2_1 + 10;
                 _ ->
                     case _a@0_0 of
                         true ->
                             case begin _y@2_1 = _y@2_0 + 1, _b@1_0 end of
                                 true -> _y@2_1 + 20;
                                 _ ->
                                     '$patina_marker':jump('$patina_label_0',
                                                           _y@2_1),
                                     []
                             end;
                         _ ->
                             _branch_value@3_0 = begin
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _y@2_0),
                                                     []
                                                 end,
                             _y@2_1 = _y@2_0,
                             _branch_value@3_0
                     end
             end,
    _r@4_0 * 100 + _y@2_1.

and_or_right(_a@0_0, _b@1_0, _c@2_0) ->
    _y@3_0 = 0,
    _r@6_0 = case [] of
                 {'$patina_label_0', _y@3_1} -> _y@3_1 + 20;
                 _ ->
                     case _a@0_0 of
                         true ->
                             case [] of
                                 {'$patina_label_1', _y@3_1} -> _y@3_1 + 10;
                                 _ ->
                                     case _b@1_0 of
                                         true ->
                                             _branch_value@4_0 = begin
                                                                     '$patina_marker':jump('$patina_label_1',
                                                                                           _y@3_0),
                                                                     []
                                                                 end,
                                             _y@3_1 = _y@3_0,
                                             _branch_value@4_0;
                                         _ ->
                                             case begin
                                                      _y@3_1 = _y@3_0 + 1,
                                                      _c@2_0
                                                  end
                                                 of
                                                 true ->
                                                     '$patina_marker':jump('$patina_label_1',
                                                                           _y@3_1),
                                                     [];
                                                 _ ->
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _y@3_1),
                                                     []
                                             end
                                     end
                             end;
                         _ ->
                             _branch_value@5_0 = begin
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _y@3_0),
                                                     []
                                                 end,
                             _y@3_1 = _y@3_0,
                             _branch_value@5_0
                     end
             end,
    _r@6_0 * 100 + _y@3_1.

or_then_and(_a@0_0, _b@1_0, _c@2_0) ->
    _y@3_0 = 0,
    _r@5_0 = case [] of
                 {'$patina_label_0', _y@3_1} -> _y@3_1 + 20;
                 _ ->
                     case [] of
                         {'$patina_label_1', _y@3_1} ->
                             case _c@2_0 of
                                 true -> _y@3_1 + 10;
                                 _ ->
                                     '$patina_marker':jump('$patina_label_0',
                                                           _y@3_1),
                                     []
                             end;
                         _ ->
                             case _a@0_0 of
                                 true ->
                                     _branch_value@4_0 = begin
                                                             '$patina_marker':jump('$patina_label_1',
                                                                                   _y@3_0),
                                                             []
                                                         end,
                                     _y@3_1 = _y@3_0,
                                     _branch_value@4_0;
                                 _ ->
                                     case begin _y@3_1 = _y@3_0 + 1, _b@1_0 end
                                         of
                                         true ->
                                             '$patina_marker':jump('$patina_label_1',
                                                                   _y@3_1),
                                             [];
                                         _ ->
                                             '$patina_marker':jump('$patina_label_0',
                                                                   _y@3_1),
                                             []
                                     end
                             end
                     end
             end,
    _r@5_0 * 100 + _y@3_1.

and_then_or(_a@0_0, _b@1_0, _c@2_0) ->
    _y@3_0 = 0,
    _r@5_0 = case [] of
                 {'$patina_label_0', _y@3_1} -> _y@3_1 + 10;
                 _ ->
                     case [] of
                         {'$patina_label_1', _y@3_1} ->
                             case _c@2_0 of
                                 true ->
                                     '$patina_marker':jump('$patina_label_0',
                                                           _y@3_1),
                                     [];
                                 _ -> _y@3_1 + 20
                             end;
                         _ ->
                             case _a@0_0 of
                                 true ->
                                     case begin _y@3_1 = _y@3_0 + 1, _b@1_0 end
                                         of
                                         true ->
                                             '$patina_marker':jump('$patina_label_0',
                                                                   _y@3_1),
                                             [];
                                         _ ->
                                             '$patina_marker':jump('$patina_label_1',
                                                                   _y@3_1),
                                             []
                                     end;
                                 _ ->
                                     _branch_value@4_0 = begin
                                                             '$patina_marker':jump('$patina_label_1',
                                                                                   _y@3_0),
                                                             []
                                                         end,
                                     _y@3_1 = _y@3_0,
                                     _branch_value@4_0
                             end
                     end
             end,
    _r@5_0 * 100 + _y@3_1.

while_or(_n@0_0) ->
    _i@1_0 = 0,
    _steps@2_0 = 0,
    begin
        {_i@1_1, _steps@2_1} = fun _$patina_loop(_i@1_0,
                                                 _steps@2_0) ->
                                       case [] of
                                           {'$patina_label_0',
                                            _i@1_2,
                                            _steps@2_2} ->
                                               {_i@1_2, _steps@2_2};
                                           _ ->
                                               _ = begin
                                                       case [] of
                                                           {'$patina_label_1',
                                                            _steps@2_1} ->
                                                               _i@1_1 = _i@1_0 +
                                                                            1,
                                                               {};
                                                           _ ->
                                                               _branch_value@4_0 =
                                                                   case _i@1_0 <
                                                                            3
                                                                       of
                                                                       true ->
                                                                           _branch_value@3_0 =
                                                                               begin
                                                                                   '$patina_marker':jump('$patina_label_1',
                                                                                                         _steps@2_0),
                                                                                   []
                                                                               end,
                                                                           _steps@2_1 =
                                                                               _steps@2_0,
                                                                           _branch_value@3_0;
                                                                       _ ->
                                                                           case
                                                                               begin
                                                                                   _steps@2_1 =
                                                                                       _steps@2_0
                                                                                           +
                                                                                           1,
                                                                                   _i@1_0
                                                                                       <
                                                                                       _n@0_0
                                                                               end
                                                                               of
                                                                               true ->
                                                                                   '$patina_marker':jump('$patina_label_1',
                                                                                                         _steps@2_1),
                                                                                   [];
                                                                               _ ->
                                                                                   begin
                                                                                       '$patina_marker':jump('$patina_label_0',
                                                                                                             _i@1_0,
                                                                                                             _steps@2_1),
                                                                                       []
                                                                                   end
                                                                           end
                                                                   end,
                                                               _i@1_1 = _i@1_0,
                                                               _branch_value@4_0
                                                       end
                                                   end,
                                               _$patina_loop(_i@1_1, _steps@2_1)
                                       end
                               end(_i@1_0, _steps@2_0),
        {}
    end,
    _i@1_1 * 100 + _steps@2_1.
