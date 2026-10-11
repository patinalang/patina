-file("tests/ui/src/generics/witness/loop_witness.pt",
      25).

-module('Elixir.Patina.LoopWitness.Pair').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export(['__info__'/1,
         '$patina_constructor__Pair'/2,
         '<impl LoopWitness::Score for LoopWitness::Pair<_>>::score$patina_loop_1'/3,
         'LoopWitness::Score::score'/2]).

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

-record #'Pair'{a, b}.

'$patina_constructor__Pair'(_a@0_0, _b@1_0) ->
    #'Pair'{a = _a@0_0, b = _b@1_0}.

'<impl LoopWitness::Score for LoopWitness::Pair<_>>::score$patina_loop_1'(_witness@0_0 =
                                                                              {'impl LoopWitness::Score for LoopWitness::Pair<_>',
                                                                               _,
                                                                               _witness@1_0},
                                                                          _total@3_0,
                                                                          _self@2_0) ->
    _ = begin
            _total@3_1 = _total@3_0 +
                             ((erlang:element(2,
                                              _witness@1_0)):'LoopWitness::Score::score'(_witness@1_0,
                                                                                         _self@2_0#'Pair'.a)
                                  +
                                  (erlang:element(2,
                                                  _witness@1_0)):'LoopWitness::Score::score'(_witness@1_0,
                                                                                             _self@2_0#'Pair'.b)),
            '$patina_marker':break({break, 0, {}, _total@3_1})
        end,
    '$patina_marker':continue(_witness@0_0,
                              _total@3_1,
                              _self@2_0).

'LoopWitness::Score::score'(_witness@0_0 =
                                {'impl LoopWitness::Score for LoopWitness::Pair<_>',
                                 _,
                                 _witness@1_0},
                            _self@2_0) ->
    _total@3_0 = 0,
    case
        '<impl LoopWitness::Score for LoopWitness::Pair<_>>::score$patina_loop_1'(_witness@0_0,
                                                                                  _total@3_0,
                                                                                  _self@2_0)
        of
        {break, 0, _loop_value@4_0, _total@3_1} ->
            _loop_value@4_0
    end,
    _total@3_1.

'LoopWitness::Score::loop_in_default'(_witness@0_0 =
                                          {'impl LoopWitness::Score for LoopWitness::Pair<_>',
                                           _,
                                           _},
                                      _arg@1_0, _arg@2_0) ->
    'Elixir.Patina.LoopWitness':'LoopWitness::Score::loop_in_default'(_witness@0_0,
                                                                      _arg@1_0,
                                                                      _arg@2_0).
