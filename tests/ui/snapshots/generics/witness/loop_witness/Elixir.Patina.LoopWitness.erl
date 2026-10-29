-file("tests/ui/src/generics/witness/loop_witness.pt",
      1).

-module('Elixir.Patina.LoopWitness').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'LoopWitness::Score::loop_in_default'/3,
         loop_in_default/2,
         pair/2,
         pair_repeated/3,
         nested_rounds/2,
         closure_loop/2]).

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

'LoopWitness::Score::loop_in_default'(_witness_Score@0_0,
                                      _self@1_0, _times@2_0) ->
    _vtable_Score@3_0 = erlang:element(1,
                                       _witness_Score@0_0),
    _fn_Score_score@4_0 = erlang:element(1,
                                         _vtable_Score@3_0),
    _total@5_0 = 0,
    _n@6_0 = 0,
    {_n@6_1, _total@5_1} = fun _$patina_loop(_n@6_0,
                                             _total@5_0) ->
                                   case [] of
                                       {'$patina_label_0',
                                        _n@6_2,
                                        _total@5_2} ->
                                           {_n@6_2, _total@5_2};
                                       _ ->
                                           case _n@6_0 < _times@2_0 of
                                               true ->
                                                   _total@5_1 = _total@5_0 +
                                                                    _fn_Score_score@4_0(_witness_Score@0_0,
                                                                                        _self@1_0),
                                                   _n@6_1 = _n@6_0 + 1,
                                                   {};
                                               _ ->
                                                   _branch_value@7_0 = begin
                                                                           '$patina_marker':jump('$patina_label_0',
                                                                                                 _n@6_0,
                                                                                                 _total@5_0),
                                                                           []
                                                                       end,
                                                   _total@5_1 = _total@5_0,
                                                   _n@6_1 = _n@6_0,
                                                   _branch_value@7_0
                                           end,
                                           _$patina_loop(_n@6_1, _total@5_1)
                                   end
                           end(_n@6_0, _total@5_0),
    _total@5_1.

loop_continuing_outer(_witnesses_where_loop_continuing_outer@0_0,
                      _t@1_0, _rounds@2_0) ->
    _witness_Score@3_0 = erlang:element(1,
                                        _witnesses_where_loop_continuing_outer@0_0),
    _vtable_Score@4_0 = erlang:element(1,
                                       _witness_Score@3_0),
    _fn_Score_score@5_0 = erlang:element(1,
                                         _vtable_Score@4_0),
    _round@6_0 = 0,
    _total@7_0 = 0,
    {_round@6_1, _total@7_1} = fun _$patina_loop(_round@6_0,
                                                 _total@7_0) ->
                                       case [] of
                                           {'$patina_label_1',
                                            _round@6_2,
                                            _total@7_2} ->
                                               {_round@6_2, _total@7_2};
                                           _ ->
                                               case [] of
                                                   {'$patina_label_0',
                                                    _round@6_1,
                                                    _total@7_1} ->
                                                       {};
                                                   _ ->
                                                       _branch_value@8_0 = begin
                                                                               _round@6_1 =
                                                                                   _round@6_0
                                                                                       +
                                                                                       1,
                                                                               fun
                                                                                   _$patina_loop(_total@7_0) ->
                                                                                       _total@7_1 =
                                                                                           _total@7_0
                                                                                               +
                                                                                               _fn_Score_score@5_0(_witness_Score@3_0,
                                                                                                                   _t@1_0),
                                                                                       case
                                                                                           _round@6_1
                                                                                               <
                                                                                               _rounds@2_0
                                                                                           of
                                                                                           true ->
                                                                                               '$patina_marker':jump('$patina_label_0',
                                                                                                                     _round@6_1,
                                                                                                                     _total@7_1),
                                                                                               [];
                                                                                           _ ->
                                                                                               {}
                                                                                       end,
                                                                                       '$patina_marker':jump('$patina_label_1',
                                                                                                             _round@6_1,
                                                                                                             _total@7_1),
                                                                                       [],
                                                                                       _$patina_loop(_total@7_1)
                                                                               end(_total@7_0)
                                                                           end,
                                                       _total@7_1 = _total@7_0,
                                                       _branch_value@8_0
                                               end,
                                               _$patina_loop(_round@6_1,
                                                             _total@7_1)
                                       end
                               end(_round@6_0, _total@7_0),
    _total@7_1.

loop_in_closure(_witnesses_where_loop_in_closure@0_0,
                _t@1_0, _times@2_0) ->
    _witness_Score@3_0 = erlang:element(1,
                                        _witnesses_where_loop_in_closure@0_0),
    _vtable_Score@4_0 = erlang:element(1,
                                       _witness_Score@3_0),
    _fn_Score_score@5_0 = erlang:element(1,
                                         _vtable_Score@4_0),
    _f@10_0 = fun (_k@6_0) ->
                      _total@7_0 = 0,
                      _n@8_0 = 0,
                      {_n@8_1, _total@7_1} = fun _$patina_loop(_n@8_0,
                                                               _total@7_0) ->
                                                     case [] of
                                                         {'$patina_label_0',
                                                          _n@8_2,
                                                          _total@7_2} ->
                                                             {_n@8_2,
                                                              _total@7_2};
                                                         _ ->
                                                             case _n@8_0 <
                                                                      _k@6_0
                                                                 of
                                                                 true ->
                                                                     _total@7_1 =
                                                                         _total@7_0
                                                                             +
                                                                             _fn_Score_score@5_0(_witness_Score@3_0,
                                                                                                 _t@1_0),
                                                                     _n@8_1 =
                                                                         _n@8_0
                                                                             +
                                                                             1,
                                                                     {};
                                                                 _ ->
                                                                     _branch_value@9_0 =
                                                                         begin
                                                                             '$patina_marker':jump('$patina_label_0',
                                                                                                   _n@8_0,
                                                                                                   _total@7_0),
                                                                             []
                                                                         end,
                                                                     _total@7_1 =
                                                                         _total@7_0,
                                                                     _n@8_1 =
                                                                         _n@8_0,
                                                                     _branch_value@9_0
                                                             end,
                                                             _$patina_loop(_n@8_1,
                                                                           _total@7_1)
                                                     end
                                             end(_n@8_0, _total@7_0),
                      _total@7_1
              end,
    _f@10_0(_times@2_0).

loop_in_default(_n@0_0, _times@1_0) ->
    _witness_Score@2_0 =
        'Elixir.Patina.LoopWitness.Points':'LoopWitness::Score::$patina_witness'(),
    'LoopWitness::Score::loop_in_default'(_witness_Score@2_0,
                                          _n@0_0,
                                          _times@1_0).

pair(_a@0_0, _b@1_0) ->
    _witness_Score@2_0 =
        'Elixir.Patina.LoopWitness.Points':'LoopWitness::Score::$patina_witness'(),
    _witnesses_where_Score@3_0 = {_witness_Score@2_0},
    _witness_Score@4_0 =
        'Elixir.Patina.LoopWitness.Pair':'LoopWitness::Score::$patina_witness'(_witnesses_where_Score@3_0),
    'Elixir.Patina.LoopWitness.Pair':'LoopWitness::Score::score'(_witness_Score@4_0,
                                                                 begin
                                                                     _a@5_0 =
                                                                         _a@0_0,
                                                                     _b@6_0 =
                                                                         _b@1_0,
                                                                     'Elixir.Patina.LoopWitness.Pair':'$patina_constructor__Pair'(_a@5_0,
                                                                                                                                  _b@6_0)
                                                                 end).

pair_repeated(_a@0_0, _b@1_0, _times@2_0) ->
    _witness_Score@3_0 =
        'Elixir.Patina.LoopWitness.Points':'LoopWitness::Score::$patina_witness'(),
    _witnesses_where_Score@4_0 = {_witness_Score@3_0},
    _witness_Score@5_0 =
        'Elixir.Patina.LoopWitness.Pair':'LoopWitness::Score::$patina_witness'(_witnesses_where_Score@4_0),
    'LoopWitness::Score::loop_in_default'(_witness_Score@5_0,
                                          begin
                                              _a@6_0 = _a@0_0,
                                              _b@7_0 = _b@1_0,
                                              'Elixir.Patina.LoopWitness.Pair':'$patina_constructor__Pair'(_a@6_0,
                                                                                                           _b@7_0)
                                          end,
                                          _times@2_0).

nested_rounds(_n@0_0, _rounds@1_0) ->
    _witness_Score@2_0 =
        'Elixir.Patina.LoopWitness.Points':'LoopWitness::Score::$patina_witness'(),
    _witnesses_where_loop_continuing_outer@3_0 =
        {_witness_Score@2_0},
    loop_continuing_outer(_witnesses_where_loop_continuing_outer@3_0,
                          _n@0_0,
                          _rounds@1_0).

closure_loop(_n@0_0, _times@1_0) ->
    _witness_Score@2_0 =
        'Elixir.Patina.LoopWitness.Points':'LoopWitness::Score::$patina_witness'(),
    _witnesses_where_loop_in_closure@3_0 =
        {_witness_Score@2_0},
    loop_in_closure(_witnesses_where_loop_in_closure@3_0,
                    _n@0_0,
                    _times@1_0).
