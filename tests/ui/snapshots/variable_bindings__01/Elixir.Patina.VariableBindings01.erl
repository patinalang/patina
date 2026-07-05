-file("tests/ui/src/variable_bindings__01.pt", 4).
-module('Elixir.Patina.VariableBindings01').
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
    _var_0 = 1,
    _var_1 = true,
    _var_2 = {},
    _var_3 = _var_0,
    begin
        io:put_chars([[<<"An integer: ">>,
                       io_lib:format("~tp", [_var_3])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"A boolean: ">>,
                       io_lib:format("~tp", [_var_1])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"Meet the unit value: ">>,
                       io_lib:format("~tp", [_var_2])]]),
        io:nl()
    end,
    _var_4 = 3,
    _var_5 = 2.
