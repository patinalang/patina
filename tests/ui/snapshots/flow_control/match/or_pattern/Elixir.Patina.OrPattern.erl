-file("tests/ui/src/flow_control/match/or_pattern.pt",
      1).

-module('Elixir.Patina.OrPattern').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         classify/1,
         classify_nested/1,
         classify_nested_map/1,
         classify_if_let/1,
         unwrap_zero_pair_if_let/1,
         classify_let_else/1]).

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

classify(_number@0_0) ->
    case _number@0_0 of
        1 -> 10;
        3 -> 10;
        5 -> 10;
        2 -> 20;
        4 -> 20;
        _ -> 0
    end.

classify_nested(_pair@0_0) ->
    case _pair@0_0 of
        {0, 2} -> 10;
        {0, 3} -> 10;
        {1, 2} -> 10;
        {1, 3} -> 10;
        _ -> 0
    end.

classify_nested_map(_map@0_0) ->
    case _map@0_0 of
        #{<<102, 105, 114, 115, 116>> := 0,
          <<115, 101, 99, 111, 110, 100>> := 2} ->
            10;
        #{<<102, 105, 114, 115, 116>> := 0,
          <<115, 101, 99, 111, 110, 100>> := 3} ->
            10;
        #{<<102, 105, 114, 115, 116>> := 1,
          <<115, 101, 99, 111, 110, 100>> := 2} ->
            10;
        #{<<102, 105, 114, 115, 116>> := 1,
          <<115, 101, 99, 111, 110, 100>> := 3} ->
            10;
        _ -> 0
    end.

classify_if_let(_number@0_0) ->
    case _number@0_0 of
        1 -> 10;
        3 -> 10;
        5 -> 10;
        _ -> 0
    end.

unwrap_zero_pair_if_let(_pair@0_0) ->
    case _pair@0_0 of
        {0, _value@1_0} -> _value@1_0;
        {_value@1_0, 0} -> _value@1_0;
        _ -> -1
    end.

classify_let_else(_number@0_0) ->
    case _number@0_0 of
        1 -> {};
        3 -> {};
        5 -> {};
        _ ->
            '$patina_marker':return(0),
            {}
    end,
    10.
