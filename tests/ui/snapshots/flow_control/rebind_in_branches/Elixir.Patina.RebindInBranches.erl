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
         match_result/1]).

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
                           false ->
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
        false ->
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
        false ->
            _value@1_1 = 20,
            {}
    end,
    _value@1_1.

initialize_or_return(_take_then@0_0) ->
    begin _value@1_0 = {} end,
    case _take_then@0_0 of
        true ->
            _value@1_1 = 30,
            {};
        false ->
            _branch_value@2_0 = begin
                                    '$patina_marker':return(40), {}
                                end,
            _value@1_1 = _value@1_0,
            _branch_value@2_0
    end,
    _value@1_1.

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
                                                      false ->
                                                          _value@2_1 = 2,
                                                          20
                                                  end,
                               _inner_value@3_0 + 100;
                           false ->
                               _value@2_1 = 3,
                               300
                       end,
    _block_value@4_0 + _value@2_1.

nested_diverging_rebind(_take_outer@0_0,
                        _take_inner@1_0) ->
    _value@2_0 = 0,
    case _take_outer@0_0 of
        true ->
            case _take_inner@1_0 of
                true ->
                    _value@2_1 = 1,
                    '$patina_marker':return(_value@2_1),
                    {};
                false ->
                    _branch_value@3_0 = begin {} end,
                    _value@2_1 = _value@2_0,
                    _branch_value@3_0
            end;
        false ->
            _branch_value@4_0 = begin {} end,
            _value@2_1 = _value@2_0,
            _branch_value@4_0
    end,
    10.

short_circuit_result(_run_block@0_0) ->
    _value@1_0 = 10,
    __result@3_0 = case _run_block@0_0 of
                       true ->
                           _value@1_1 = _value@1_0 + 5,
                           true;
                       false ->
                           _branch_value@2_0 = false,
                           _value@1_1 = _value@1_0,
                           _branch_value@2_0
                   end,
    _value@1_1.

match_result(_take_first@0_0) ->
    _value@1_0 = 10,
    _branch@2_0 = case _take_first@0_0 of
                      true -> 'Elixir.Patina.RebindInBranches.Branch.First';
                      false -> 'Elixir.Patina.RebindInBranches.Branch.Second'
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
