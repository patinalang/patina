-file("tests/ui/src/flow_control/let_else.pt", 1).

-module('Elixir.Patina.LetElse').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         tuple_pattern/0,
         list_pattern/1]).

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

tuple_pattern() ->
    case {20, 22} of
        {20, _second@0_1} -> {};
        _ ->
            _branch_value@1_0 = begin
                                    '$patina_marker':return(0)
                                end,
            _second@0_1 = {},
            _branch_value@1_0
    end,
    20 + _second@0_1.

list_pattern(_values@0_0) ->
    case _values@0_0 of
        [_first@1_1, _second@2_1 | _] -> {};
        _ ->
            _branch_value@3_0 = begin
                                    '$patina_marker':return(0)
                                end,
            _first@1_1 = {},
            _second@2_1 = {},
            _branch_value@3_0
    end,
    _first@1_1 + _second@2_1.
