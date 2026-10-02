-file("tests/ui/src/primitives/errors/map_pattern_ke"
      "y_native_record.pt",
      1).

-module('Elixir.Patina.MapPatternKeyNativeRecord').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, record_key/1]).

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

record_key(_map@0_0) ->
    case _map@0_0 of
        #{'Elixir.Patina.MapPatternKeyNativeRecord.Point':'$patina_constructor__Point'(1,
                                                                                       2)
              := _value@1_0} ->
            _value@1_0;
        _ -> -1
    end.
