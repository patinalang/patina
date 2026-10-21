-file("tests/ui/src/flow_control/early_return.pt", 1).

-module('Elixir.Patina.EarlyReturn').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         is_negative/1,
         is_negative_implicit/1,
         closure_value/1,
         inlined_closure_value/1,
         passed_closure_value/1,
         expression_value/1,
         match_stopped/0,
         match_continued/0,
         match_scrutinee_value/1,
         nested_value/1,
         if_condition_value/1,
         return_in_try/2,
         return_in_catch/2,
         return_in_after/2,
         return_in_try_and_after/2,
         return_through_nested_afters/1,
         return_from_inner_after/1,
         return_in_try_without_after/2,
         return_in_large_after/2]).

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

mark(_value@0_0) ->
    _marks@1_0 = begin erlang:get(marks) end,
    erlang:put(marks, [_value@0_0 | _marks@1_0]),
    _value@0_0.

match_value(_choice@0_0) ->
    _value@1_0 = case _choice@0_0 of
                     'Elixir.Patina.EarlyReturn.Choice.Stop' ->
                         '$patina_marker':return(10),
                         [];
                     'Elixir.Patina.EarlyReturn.Choice.Continue' -> 20
                 end,
    _value@1_0 + 1.

is_negative(_x@0_0) ->
    case _x@0_0 >= 0 of
        true -> begin '$patina_marker':return(false), [] end;
        _ -> {}
    end,
    begin '$patina_marker':return(true), [] end.

is_negative_implicit(_x@0_0) ->
    case _x@0_0 >= 0 of
        true -> begin '$patina_marker':return(false), [] end;
        _ -> {}
    end,
    true.

closure_value(_stop@0_0) ->
    __closure@1_0 = fun () ->
                            case _stop@0_0 of
                                true ->
                                    begin '$patina_marker':return(4), [] end;
                                _ -> {}
                            end,
                            5
                    end,
    10.

inlined_closure_value(_stop@0_0) ->
    _closure@1_0 = fun () ->
                           case _stop@0_0 of
                               true -> begin '$patina_marker':return(4), [] end;
                               _ -> {}
                           end,
                           5
                   end,
    _closure@1_0() * 10.

call_with(_f@0_0, _x@1_0) -> _f@0_0(_x@1_0) * 10.

passed_closure_value(_x@0_0) ->
    call_with(fun (_y@1_0) ->
                      case _y@1_0 < 0 of
                          true -> begin '$patina_marker':return(0), [] end;
                          _ -> {}
                      end,
                      _y@1_0 + 1
              end,
              _x@0_0).

expression_value(_stop@0_0) ->
    mark(1) +
        begin
            case _stop@0_0 of
                true -> begin '$patina_marker':return(mark(2)), [] end;
                _ -> {}
            end,
            mark(3)
        end.

match_stopped() ->
    match_value('Elixir.Patina.EarlyReturn.Choice.Stop').

match_continued() ->
    match_value('Elixir.Patina.EarlyReturn.Choice.Continue').

match_scrutinee_value(_stop@0_0) ->
    case begin
             case _stop@0_0 of
                 true -> begin '$patina_marker':return(40), [] end;
                 _ -> {}
             end,
             'Elixir.Patina.EarlyReturn.Choice.Continue'
         end
        of
        'Elixir.Patina.EarlyReturn.Choice.Stop' -> 50;
        'Elixir.Patina.EarlyReturn.Choice.Continue' -> 60
    end.

nested_value(_x@0_0) ->
    _result@1_0 = begin
                      case _x@0_0 =:= 0 of
                          true -> begin '$patina_marker':return(10), [] end;
                          _ -> {}
                      end,
                      begin
                          case _x@0_0 =:= 1 of
                              true -> begin '$patina_marker':return(20), [] end;
                              _ -> {}
                          end,
                          30
                      end
                  end,
    _result@1_0 + 1.

if_condition_value(_stop@0_0) ->
    case begin
             case _stop@0_0 of
                 true -> begin '$patina_marker':return(10), [] end;
                 _ -> {}
             end,
             true
         end
        of
        true -> 20;
        _ -> 30
    end.

return_in_try(_stop@0_0, _f@1_0) ->
    _value@2_0 = try case _stop@0_0 of
                         true -> begin '$patina_marker':return(10), [] end;
                         _ -> {}
                     end,
                     _f@1_0(),
                     20
                 catch
                     error:_ -> 30
                 after
                     mark(0), {}
                 end,
    _value@2_0 + 1.

return_in_catch(_stop@0_0, _f@1_0) ->
    _value@2_0 = try _f@1_0(), 20 catch
                     error:_ ->
                         case _stop@0_0 of
                             true -> begin '$patina_marker':return(10), [] end;
                             _ -> {}
                         end,
                         30
                 after
                     mark(0), {}
                 end,
    _value@2_0 + 1.

return_in_after(_stop@0_0, _f@1_0) ->
    _value@2_0 = try _f@1_0(), 20 catch
                     error:_ -> 30
                 after
                     mark(0),
                     case _stop@0_0 of
                         true -> begin '$patina_marker':return(10), [] end;
                         _ -> {}
                     end
                 end,
    _value@2_0 + 1.

return_in_try_and_after(_stop_try@0_0,
                        _stop_after@1_0) ->
    try case _stop_try@0_0 of
            true -> begin '$patina_marker':return(mark(1)), [] end;
            _ -> {}
        end,
        2
    after
        case _stop_after@1_0 of
            true -> begin '$patina_marker':return(mark(3)), [] end;
            _ -> {}
        end
    end.

return_through_nested_afters(_stop@0_0) ->
    try try case _stop@0_0 of
                true -> begin '$patina_marker':return(mark(1)), [] end;
                _ -> {}
            end,
            2
        after
            mark(10), {}
        end
    after
        mark(20), {}
    end.

return_from_inner_after(_stop@0_0) ->
    try try 2 after
            case _stop@0_0 of
                true -> begin '$patina_marker':return(1), [] end;
                _ -> {}
            end,
            mark(10),
            {}
        end
    after
        mark(20), {}
    end.

return_in_try_without_after(_stop@0_0, _f@1_0) ->
    _value@2_0 = try case _stop@0_0 of
                         true -> begin '$patina_marker':return(10), [] end;
                         _ -> {}
                     end,
                     _f@1_0(),
                     20
                 catch
                     error:_ -> 30
                 end,
    _value@2_0 + 1.

return_in_large_after(_stop@0_0, _x@1_0) ->
    _value@27_0 = try mark(1) after
                      _v0@2_0 = _x@1_0,
                      _v1@3_0 = _v0@2_0 + _x@1_0,
                      _v2@4_0 = _v1@3_0 + _x@1_0,
                      _v3@5_0 = _v2@4_0 + _x@1_0,
                      _v4@6_0 = _v3@5_0 + _x@1_0,
                      _v5@7_0 = _v4@6_0 + _x@1_0,
                      _v6@8_0 = _v5@7_0 + _x@1_0,
                      _v7@9_0 = _v6@8_0 + _x@1_0,
                      _v8@10_0 = _v7@9_0 + _x@1_0,
                      _v9@11_0 = _v8@10_0 + _x@1_0,
                      _v10@12_0 = _v9@11_0 + _x@1_0,
                      _v11@13_0 = _v10@12_0 + _x@1_0,
                      _v12@14_0 = _v11@13_0 + _x@1_0,
                      _v13@15_0 = _v12@14_0 + _x@1_0,
                      _v14@16_0 = _v13@15_0 + _x@1_0,
                      _v15@17_0 = _v14@16_0 + _x@1_0,
                      _v16@18_0 = _v15@17_0 + _x@1_0,
                      _v17@19_0 = _v16@18_0 + _x@1_0,
                      _v18@20_0 = _v17@19_0 + _x@1_0,
                      _v19@21_0 = _v18@20_0 + _x@1_0,
                      _v20@22_0 = _v19@21_0 + _x@1_0,
                      _v21@23_0 = _v20@22_0 + _x@1_0,
                      _v22@24_0 = _v21@23_0 + _x@1_0,
                      _v23@25_0 = _v22@24_0 + _x@1_0,
                      _v24@26_0 = _v23@25_0 + _x@1_0,
                      case _stop@0_0 of
                          true ->
                              begin '$patina_marker':return(_v24@26_0), [] end;
                          _ -> {}
                      end,
                      mark(_v24@26_0),
                      {}
                  end,
    _value@27_0 + 1.
