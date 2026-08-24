-file("tests/ui/src/flow_control__01.pt", 4).
-module('Elixir.Patina.FlowControl01').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,main/0]).
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
main() ->
    _n@0_0 = 5,
    case _n@0_0 > 4 of
        true ->
            _n@0_1 = 4;
        false ->
            _branch_value@1_0 = {},
            _n@0_1 = _n@0_0,
            _branch_value@1_0
    end,
    begin
        io:put_chars([[io_lib:format("~ktp", [_n@0_1])]]),
        io:nl()
    end.
