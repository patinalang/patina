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

closure_loop() ->
    _count@3_0 = fun (_x@0_0, _y@1_0) ->
                         _i@2_0 = _x@0_0,
                         begin
                             _i@2_1 = fun _$patina_loop(_i@2_0) ->
                                              case [] of
                                                  {'$patina_label_0', _i@2_2} ->
                                                      _i@2_2;
                                                  _ ->
                                                      _ = begin
                                                              _i@2_1 = _i@2_0 +
                                                                           _y@1_0,
                                                              case _i@2_1 > 3 of
                                                                  true ->
                                                                      begin
                                                                          '$patina_marker':jump('$patina_label_0',
                                                                                                _i@2_1),
                                                                          []
                                                                      end;
                                                                  _ -> {}
                                                              end
                                                          end,
                                                      _$patina_loop(_i@2_1)
                                              end
                                      end(_i@2_0),
                             {}
                         end,
                         _i@2_1
                 end,
    _count@3_0(1, 2).
