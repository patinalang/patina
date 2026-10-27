-file("tests/ui/src/flow_control/loop_outer_continue.pt",
      3).

-module('Elixir.Patina.LoopOuterContinue').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         continue_outer_loop/1,
         continue_two_loops/1]).

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

continue_outer_loop(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    begin
        {_jump_value@6_0, _outer@1_1, _visits@2_1} = fun
                                                         _$patina_loop(_outer@1_0,
                                                                       _visits@2_0) ->
                                                             case [] of
                                                                 {'$patina_label_1',
                                                                  _jump_value@5_0,
                                                                  _outer@1_2,
                                                                  _visits@2_2} ->
                                                                     {_jump_value@5_0,
                                                                      _outer@1_2,
                                                                      _visits@2_2};
                                                                 _ ->
                                                                     _ = case []
                                                                             of
                                                                             {'$patina_label_0',
                                                                              _outer@1_1,
                                                                              _visits@2_1} ->
                                                                                 {};
                                                                             _ ->
                                                                                 _branch_value@4_0 =
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
                                                                                                             _jump_value@3_0 =
                                                                                                                 _visits@2_1
                                                                                                                     *
                                                                                                                     100
                                                                                                                     +
                                                                                                                     _outer@1_1,
                                                                                                             begin
                                                                                                                 '$patina_marker':jump('$patina_label_1',
                                                                                                                                       _jump_value@3_0,
                                                                                                                                       _outer@1_1,
                                                                                                                                       _visits@2_1),
                                                                                                                 []
                                                                                                             end
                                                                                                         end
                                                                                                     end,
                                                                                                 _$patina_loop(_visits@2_1)
                                                                                         end(_visits@2_0)
                                                                                     end,
                                                                                 _visits@2_1 =
                                                                                     _visits@2_0,
                                                                                 _branch_value@4_0
                                                                         end,
                                                                     _$patina_loop(_outer@1_1,
                                                                                   _visits@2_1)
                                                             end
                                                     end(_outer@1_0,
                                                         _visits@2_0),
        _jump_value@6_0
    end.

continue_two_loops(_limit@0_0) ->
    _outer@1_0 = 0,
    _visits@2_0 = 0,
    begin
        {_jump_value@6_0, _outer@1_1, _visits@2_1} = fun
                                                         _$patina_loop(_outer@1_0,
                                                                       _visits@2_0) ->
                                                             case [] of
                                                                 {'$patina_label_1',
                                                                  _jump_value@5_0,
                                                                  _outer@1_2,
                                                                  _visits@2_2} ->
                                                                     {_jump_value@5_0,
                                                                      _outer@1_2,
                                                                      _visits@2_2};
                                                                 _ ->
                                                                     _ = case []
                                                                             of
                                                                             {'$patina_label_0',
                                                                              _outer@1_1,
                                                                              _visits@2_1} ->
                                                                                 {};
                                                                             _ ->
                                                                                 _branch_value@4_0 =
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
                                                                                                                             _jump_value@3_0 =
                                                                                                                                 _visits@2_1
                                                                                                                                     *
                                                                                                                                     100
                                                                                                                                     +
                                                                                                                                     _outer@1_1,
                                                                                                                             begin
                                                                                                                                 '$patina_marker':jump('$patina_label_1',
                                                                                                                                                       _jump_value@3_0,
                                                                                                                                                       _outer@1_1,
                                                                                                                                                       _visits@2_1),
                                                                                                                                 []
                                                                                                                             end
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
                                                                                 _branch_value@4_0
                                                                         end,
                                                                     _$patina_loop(_outer@1_1,
                                                                                   _visits@2_1)
                                                             end
                                                     end(_outer@1_0,
                                                         _visits@2_0),
        _jump_value@6_0
    end.
