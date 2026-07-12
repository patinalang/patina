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
    _var_0 = 5,
    case _var_0 > 4 of
        true ->
            _var_1 = 4;
        false ->
            {}
    end,
    begin
        io:put_chars([[io_lib:format("~tp", [_var_1])]]),
        io:nl()
    end.
