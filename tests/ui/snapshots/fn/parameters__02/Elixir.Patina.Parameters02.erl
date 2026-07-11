-file("tests/ui/src/fn/parameters__02.pt", 4).
-module('Elixir.Patina.Parameters02').
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
lazy_add(_var_0, _var_1) ->
    fun() ->
           _var_0() + _var_1()
    end.
add(_var_0, _var_1) ->
    _var_0 + _var_1.
mul(_var_0, _var_1) ->
    _var_0 * _var_1.
choose_op(_var_0) ->
    case _var_0 of
        'Elixir.Patina.Parameters02.MathOp.Add' ->
            fun 'Elixir.Patina.Parameters02':add/2;
        'Elixir.Patina.Parameters02.MathOp.Mul' ->
            fun 'Elixir.Patina.Parameters02':mul/2
    end.
main() ->
    _var_0 =
        fun() ->
               begin
                   io:put_chars([[<<"fx evaluated!">>]]),
                   io:nl()
               end,
               3
        end,
    _var_1 =
        fun() ->
               begin
                   io:put_chars([[<<"fy evaluated!">>]]),
                   io:nl()
               end,
               5
        end,
    _var_2 = lazy_add(_var_0, _var_1),
    _var_3 = choose_op('Elixir.Patina.Parameters02.MathOp.Add'),
    begin
        io:put_chars([[io_lib:format("~tp", [3]),
                       <<" op ">>,
                       io_lib:format("~tp", [5]),
                       <<" = ">>,
                       io_lib:format("~tp", [_var_3(3, 5)])]]),
        io:nl()
    end.
