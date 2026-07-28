-file("tests/ui/src/fn/closure_loop.pt", 1).
-module('Elixir.Patina.ClosureLoop').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,'$patina_loop_11_23'/1,closure_loop/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
'$patina_loop_11_23'(_i@0_0) ->
    begin
        _i@0_1 = _i@0_0 + 1,
        case _i@0_1 > 3 of
            true ->
                '$patina_marker':break({break, 0, {}, _i@0_1});
            false ->
                {}
        end
    end,
    '$patina_marker':continue(_i@0_1).
closure_loop() ->
    _count@2_0 =
        fun() ->
               _i@0_0 = 0,
               case '$patina_loop_11_23'(_i@0_0) of
                   {break, 0, _loop_value@1_0, _i@0_1} ->
                       _loop_value@1_0
               end,
               _i@0_1
        end,
    call_once(_count@2_0, {}).
