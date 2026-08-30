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
lazy_add(_fx@0_0, _fy@1_0) ->
    fun() ->
           _fx@0_0() + _fy@1_0()
    end.
add(_x@0_0, _y@1_0) ->
    _x@0_0 + _y@1_0.
mul(_x@0_0, _y@1_0) ->
    _x@0_0 * _y@1_0.
choose_op(_op@0_0) ->
    case _op@0_0 of
        'Elixir.Patina.Parameters02.MathOp.Add' ->
            fun add/2;
        'Elixir.Patina.Parameters02.MathOp.Mul' ->
            fun mul/2
    end.
main() ->
    _fx@0_0 =
        fun() ->
               begin
                   io:put_chars([[<<"fx evaluated!">>]]),
                   io:nl()
               end,
               3
        end,
    _fy@1_0 =
        fun() ->
               begin
                   io:put_chars([[<<"fy evaluated!">>]]),
                   io:nl()
               end,
               5
        end,
    _lazy_result@2_0 = lazy_add(_fx@0_0, _fy@1_0),
    _op@3_0 = choose_op('Elixir.Patina.Parameters02.MathOp.Add'),
    begin
        io:put_chars([[io_lib:format("~ktp", [3]),
                       <<" op ">>,
                       io_lib:format("~ktp", [5]),
                       <<" = ">>,
                       io_lib:format("~ktp", [_op@3_0(3, 5)])]]),
        io:nl()
    end.
