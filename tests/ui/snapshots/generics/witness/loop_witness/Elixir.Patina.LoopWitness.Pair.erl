-file("tests/ui/src/generics/witness/loop_witness.pt",
      25).

-module('Elixir.Patina.LoopWitness.Pair').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export(['__info__'/1,
         '$patina_constructor__Pair'/2,
         'LoopWitness::Score::score'/2,
         'LoopWitness::Score::$patina_witness'/1]).

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

'LoopWitness::Score::score'(_witness_Score@0_0,
                            _self@1_0) ->
    _where_Score@2_0 = erlang:element(3,
                                      _witness_Score@0_0),
    _witness_Score@3_0 = erlang:element(1,
                                        _where_Score@2_0),
    _vtable_Score@4_0 = erlang:element(1,
                                       _witness_Score@3_0),
    _fn_Score_score@5_0 = erlang:element(1,
                                         _vtable_Score@4_0),
    begin
        _total@6_0 = 0,
        begin
            _total@6_1 = fun _$patina_loop(_total@6_0) ->
                                 case [] of
                                     {'$patina_label_0', _total@6_2} ->
                                         _total@6_2;
                                     _ ->
                                         _ = begin
                                                 _total@6_1 = _total@6_0 +
                                                                  (_fn_Score_score@5_0(_witness_Score@3_0,
                                                                                       _self@1_0#'Pair'.a)
                                                                       +
                                                                       _fn_Score_score@5_0(_witness_Score@3_0,
                                                                                           _self@1_0#'Pair'.b)),
                                                 begin
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _total@6_1),
                                                     []
                                                 end
                                             end,
                                         _$patina_loop(_total@6_1)
                                 end
                         end(_total@6_0),
            {}
        end,
        _total@6_1
    end.

'LoopWitness::Score::$patina_witness'(_witnesses_where_Score@0_0) ->
    {{fun 'Elixir.Patina.LoopWitness.Pair':'LoopWitness::Score::score'/2,
      fun 'Elixir.Patina.LoopWitness':'LoopWitness::Score::loop_in_default'/3},
     {},
     _witnesses_where_Score@0_0}.
