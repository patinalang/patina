-file("tests/ui/src/expression__03.pt", 4).
-module('Elixir.Patina.Expression03').
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
    _var_3 =
        begin
            _var_1 = _var_0 * _var_0,
            _var_2 = _var_1 * _var_0,
            _var_2 + _var_1 + _var_0
        end,
    _var_4 = begin 2 * _var_0 end,
    begin
        io:put_chars([[<<"x is ">>, io_lib:format("~tp", [_var_0])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"y is ">>, io_lib:format("~tp", [_var_3])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"z is ">>, io_lib:format("~tp", [_var_4])]]),
        io:nl()
    end.
