-file("tests/ui/src/generics/witness/loop_witness.pt",
      15).

-module('Elixir.Patina.LoopWitness.Points').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'LoopWitness::Score::score'/2]).

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

'LoopWitness::Score::score'(_witness@0_0 =
                                {'impl LoopWitness::Score for LoopWitness::Points',
                                 _},
                            _self@1_0) ->
    _self@1_0.

'LoopWitness::Score::loop_in_default'(_witness@0_0 =
                                          {'impl LoopWitness::Score for LoopWitness::Points',
                                           _},
                                      _arg@1_0, _arg@2_0) ->
    'Elixir.Patina.LoopWitness':'LoopWitness::Score::loop_in_default'(_witness@0_0,
                                                                      _arg@1_0,
                                                                      _arg@2_0).
