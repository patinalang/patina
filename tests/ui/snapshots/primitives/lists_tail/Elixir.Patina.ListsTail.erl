-file("tests/ui/src/primitives/lists_tail.pt", 1).
-module('Elixir.Patina.ListsTail').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,prepend/1,match_tail/1,let_else_tail/1]).
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
prepend(_xs@0_0) ->
    [-1, 0 | _xs@0_0].
match_tail(_xs@0_0) ->
    _more_xs@1_0 = [-1, 0 | _xs@0_0],
    case _more_xs@1_0 of
        [-1, 0 | _tail@2_0 = _] ->
            _tail@2_0;
        _ ->
            []
    end.
let_else_tail(_xs@0_0) ->
    _more_xs@1_0 = [-1, 0 | _xs@0_0],
    case _more_xs@1_0 of
        [-1, 0 | _tail@2_1 = _] ->
            {};
        _ ->
            _branch_value@3_0 = begin '$patina_marker':return([]) end,
            _tail@2_1 = {},
            _branch_value@3_0
    end,
    _tail@2_1.
