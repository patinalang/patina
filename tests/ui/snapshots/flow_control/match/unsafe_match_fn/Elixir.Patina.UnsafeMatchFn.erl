-file("tests/ui/src/flow_control/match/unsafe_match_"
      "fn.pt",
      1).

-module('Elixir.Patina.UnsafeMatchFn').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, match_point/1]).

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

match_point(_value@0_0) ->
    case _value@0_0 of
        #'Elixir.Patina.UnsafeMatchFn.Point':'Point'{x = _x@1_0,
                                                     y = _y@2_0}
            when erlang:is_integer(_x@1_0),
                 erlang:is_integer(_y@2_0) ->
            _x@1_0 + _y@2_0;
        _ -> 0
    end.
