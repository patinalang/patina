-file("tests/ui/src/generics/witness/loop_witness.pt",
      1).

-module('Elixir.Patina.LoopWitness').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'LoopWitness::Score::loop_in_default$patina_loop_1'/5,
         'LoopWitness::Score::loop_in_default'/3,
         'loop_continuing_outer$patina_loop_2'/5,
         'loop_continuing_outer$patina_loop_1'/5,
         'loop_in_closure$patina_loop_1'/5,
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

'LoopWitness::Score::loop_in_default$patina_loop_1'(_witness@0_0,
                                                    _n@4_0, _times@2_0,
                                                    _total@3_0, _self@1_0) ->
    _ = begin
            case _n@4_0 < _times@2_0 of
                true ->
                    _total@3_1 = _total@3_0 +
                                     (erlang:element(2,
                                                     _witness@0_0)):'LoopWitness::Score::score'(_witness@0_0,
                                                                                                _self@1_0),
                    _n@4_1 = _n@4_0 + 1,
                    {};
                false ->
                    _branch_value@5_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@4_0,
                                                                    _total@3_0})
                                        end,
                    _total@3_1 = _total@3_0,
                    _n@4_1 = _n@4_0,
                    _branch_value@5_0
            end
        end,
    '$patina_marker':continue(_witness@0_0,
                              _n@4_1,
                              _times@2_0,
                              _total@3_1,
                              _self@1_0).

'LoopWitness::Score::loop_in_default'(_witness@0_0,
                                      _self@1_0, _times@2_0) ->
    _total@3_0 = 0,
    _n@4_0 = 0,
    case
        'LoopWitness::Score::loop_in_default$patina_loop_1'(_witness@0_0,
                                                            _n@4_0,
                                                            _times@2_0,
                                                            _total@3_0,
                                                            _self@1_0)
        of
        {break, 0, _loop_value@6_0, _n@4_1, _total@3_1} ->
            _loop_value@6_0
    end,
    _total@3_1.

'loop_continuing_outer$patina_loop_2'(_witness@1_0 =
                                          {_witness@0_0},
                                      _total@5_0, _t@2_0, _round@4_1,
                                      _rounds@3_0) ->
    _ = begin
            _total@5_1 = _total@5_0 +
                             (erlang:element(2,
                                             _witness@0_0)):'LoopWitness::Score::score'(_witness@0_0,
                                                                                        _t@2_0),
            case _round@4_1 < _rounds@3_0 of
                true ->
                    '$patina_marker':break({continue, 0, _total@5_1});
                false -> {}
            end,
            '$patina_marker':break({break, 0, {}, _total@5_1})
        end,
    '$patina_marker':continue(_witness@1_0,
                              _total@5_1,
                              _t@2_0,
                              _round@4_1,
                              _rounds@3_0).

'loop_continuing_outer$patina_loop_1'(_witness@1_0 =
                                          {_witness@0_0},
                                      _round@4_0, _total@5_0, _t@2_0,
                                      _rounds@3_0) ->
    _ = begin
            _round@4_1 = _round@4_0 + 1,
            case 'loop_continuing_outer$patina_loop_2'(_witness@1_0,
                                                       _total@5_0,
                                                       _t@2_0,
                                                       _round@4_1,
                                                       _rounds@3_0)
                of
                {break,
                 _target_loop_depth@6_0,
                 _loop_value@7_0,
                 _total@5_1} ->
                    '$patina_marker':break({break,
                                            _target_loop_depth@6_0,
                                            _loop_value@7_0,
                                            _round@4_1,
                                            _total@5_1});
                {continue, 0, _total@5_1} ->
                    '$patina_marker':continue(_witness@1_0,
                                              _round@4_1,
                                              _total@5_1,
                                              _t@2_0,
                                              _rounds@3_0)
            end
        end,
    '$patina_marker':continue(_witness@1_0,
                              _round@4_1,
                              _total@5_1,
                              _t@2_0,
                              _rounds@3_0).

loop_continuing_outer(_witness@1_0 = {_witness@0_0},
                      _t@2_0, _rounds@3_0) ->
    _round@4_0 = 0,
    _total@5_0 = 0,
    case 'loop_continuing_outer$patina_loop_1'(_witness@1_0,
                                               _round@4_0,
                                               _total@5_0,
                                               _t@2_0,
                                               _rounds@3_0)
        of
        {break, 0, _loop_value@8_0, _round@4_1, _total@5_1} ->
            _loop_value@8_0
    end,
    _total@5_1.

'loop_in_closure$patina_loop_1'(_witness@1_0 =
                                    {_witness@0_0},
                                _n@6_0, _k@4_0, _total@5_0, _t@2_0) ->
    _ = begin
            case _n@6_0 < _k@4_0 of
                true ->
                    _total@5_1 = _total@5_0 +
                                     (erlang:element(2,
                                                     _witness@0_0)):'LoopWitness::Score::score'(_witness@0_0,
                                                                                                _t@2_0),
                    _n@6_1 = _n@6_0 + 1,
                    {};
                false ->
                    _branch_value@7_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@6_0,
                                                                    _total@5_0})
                                        end,
                    _total@5_1 = _total@5_0,
                    _n@6_1 = _n@6_0,
                    _branch_value@7_0
            end
        end,
    '$patina_marker':continue(_witness@1_0,
                              _n@6_1,
                              _k@4_0,
                              _total@5_1,
                              _t@2_0).

loop_in_closure(_witness@1_0 = {_witness@0_0}, _t@2_0,
                _times@3_0) ->
    _f@9_0 = fun (_k@4_0) ->
                     _total@5_0 = 0,
                     _n@6_0 = 0,
                     case 'loop_in_closure$patina_loop_1'(_witness@1_0,
                                                          _n@6_0,
                                                          _k@4_0,
                                                          _total@5_0,
                                                          _t@2_0)
                         of
                         {break, 0, _loop_value@8_0, _n@6_1, _total@5_1} ->
                             _loop_value@8_0
                     end,
                     _total@5_1
             end,
    _f@9_0(_times@3_0).

loop_in_default(_n@0_0, _times@1_0) ->
    'LoopWitness::Score::loop_in_default'({'impl LoopWitness::Score for LoopWitness::Points',
                                           'Elixir.Patina.LoopWitness.Points'},
                                          _n@0_0,
                                          _times@1_0).

pair(_a@0_0, _b@1_0) ->
    'Elixir.Patina.LoopWitness.Pair':'LoopWitness::Score::score'({'impl LoopWitness::Score for LoopWitness::Pair<_>',
                                                                  'Elixir.Patina.LoopWitness.Pair',
                                                                  {'impl LoopWitness::Score for LoopWitness::Points',
                                                                   'Elixir.Patina.LoopWitness.Points'}},
                                                                 'Elixir.Patina.LoopWitness.Pair':'$patina_constructor__Pair'(_a@0_0,
                                                                                                                              _b@1_0)).

pair_repeated(_a@0_0, _b@1_0, _times@2_0) ->
    'LoopWitness::Score::loop_in_default'({'impl LoopWitness::Score for LoopWitness::Pair<_>',
                                           'Elixir.Patina.LoopWitness.Pair',
                                           {'impl LoopWitness::Score for LoopWitness::Points',
                                            'Elixir.Patina.LoopWitness.Points'}},
                                          'Elixir.Patina.LoopWitness.Pair':'$patina_constructor__Pair'(_a@0_0,
                                                                                                       _b@1_0),
                                          _times@2_0).

nested_rounds(_n@0_0, _rounds@1_0) ->
    loop_continuing_outer({{'impl LoopWitness::Score for LoopWitness::Points',
                            'Elixir.Patina.LoopWitness.Points'}},
                          _n@0_0,
                          _rounds@1_0).

closure_loop(_n@0_0, _times@1_0) ->
    loop_in_closure({{'impl LoopWitness::Score for LoopWitness::Points',
                      'Elixir.Patina.LoopWitness.Points'}},
                    _n@0_0,
                    _times@1_0).
