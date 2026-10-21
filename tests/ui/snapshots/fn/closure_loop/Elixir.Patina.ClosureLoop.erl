-file("tests/ui/src/fn/closure_loop.pt", 1).

-module('Elixir.Patina.ClosureLoop').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, closure_loop/0]).

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

'closure_loop$patina_loop_1'(_i@2_0, _y@1_0) ->
    _ = begin
            _i@2_1 = _i@2_0 + _y@1_0,
            case _i@2_1 > 3 of
                true ->
                    begin
                        '$patina_marker':break({break, 0, {}, _i@2_1}), []
                    end;
                _ -> {}
            end
        end,
    begin '$patina_marker':continue(_i@2_1, _y@1_0), [] end.

closure_loop() ->
    _count@4_0 = fun (_x@0_0, _y@1_0) ->
                         _i@2_0 = _x@0_0,
                         case 'closure_loop$patina_loop_1'(_i@2_0, _y@1_0) of
                             {break, 0, _loop_value@3_0, _i@2_1} ->
                                 _loop_value@3_0
                         end,
                         _i@2_1
                 end,
    _count@4_0(1, 2).
