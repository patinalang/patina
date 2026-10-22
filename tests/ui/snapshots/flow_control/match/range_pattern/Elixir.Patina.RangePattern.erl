-file("tests/ui/src/flow_control/match/range_pattern.pt",
      1).

-module('Elixir.Patina.RangePattern').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         classify_int/1,
         classify_char/1,
         classify_float/1,
         matches_either_range/1,
         match_bound_range/1,
         match_nested_ranges/1,
         match_enum_range/1,
         match_map_range/1,
         if_let_range/1,
         let_else_range/1]).

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

classify_int(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 =< -11 -> 1;
        _range@2_0 when _range@2_0 >= -10, _range@2_0 < 0 -> 2;
        _range@3_0 when _range@3_0 >= 0, _range@3_0 =< 10 -> 3;
        _range@4_0 when _range@4_0 >= 11 -> 4
    end.

classify_char(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 =< $` -> 1;
        _range@2_0 when _range@2_0 >= $a, _range@2_0 =< $z -> 2;
        _range@3_0 when _range@3_0 >= ${ -> 3
    end.

classify_float(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 =< -1.5 -> 1;
        _range@2_0 when _range@2_0 >= -1.5, _range@2_0 < 1.5 ->
            2;
        _range@3_0 when _range@3_0 >= 1.5 -> 3;
        _ -> 4
    end.

matches_either_range(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 >= 1, _range@1_0 =< 3 ->
            '$patina_marker':jump('$patina_label_0'),
            [];
        _range@2_0 when _range@2_0 >= 7, _range@2_0 < 9 ->
            '$patina_marker':jump('$patina_label_0'),
            [];
        11 ->
            '$patina_marker':jump('$patina_label_0'),
            [];
        _ -> false;
        {'$patina_label_0'} -> true
    end.

match_bound_range(_value@0_0) ->
    case _value@0_0 of
        _bound@1_0 = _range@2_0
            when _range@2_0 >= -3, _range@2_0 =< 3,
                 _bound@1_0 rem 2 =:= 0 ->
            _bound@1_0 * 10;
        _ -> 0
    end.

match_nested_ranges(_value@0_0) ->
    case _value@0_0 of
        {_range@1_0, _range@2_0}
            when _range@1_0 >= -2, _range@1_0 =< 2,
                 _range@2_0 >= $m, _range@2_0 =< $p ->
            true;
        _ -> false
    end.

match_enum_range(_value@0_0) ->
    case _value@0_0 of
        _bound@1_0 = _range@2_0
            when _range@2_0 >= 40, _range@2_0 =< 44,
                 _bound@1_0 =/= 41 ->
            true;
        _ -> false
    end.

match_map_range(_value@0_0) ->
    case _value@0_0 of
        #{<<"score"/utf8>> := _range@1_0}
            when _range@1_0 >= 10, _range@1_0 < 20 ->
            true;
        _ -> false
    end.

if_let_range(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 >= 2, _range@1_0 =< 4 ->
            true;
        _ -> false
    end.

let_else_range(_value@0_0) ->
    case _value@0_0 of
        _range@1_0 when _range@1_0 >= -1, _range@1_0 =< 1 -> {};
        _ -> begin '$patina_marker':return(false), [] end
    end,
    true.
