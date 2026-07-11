-file("tests/ui/src/flow_control/match/destructuring/destructure_enum__"
      "01.pt",
      6).
-module('Elixir.Patina.DestructureEnum01').
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
    _var_0 =
        #'Elixir.Patina.DestructureEnum01.Color':'RGB'{'0' = 122,
                                                       '1' = 17,
                                                       '2' = 40},
    begin
        io:put_chars([[<<"What color is it?">>]]),
        io:nl()
    end,
    case _var_0 of
        'Elixir.Patina.DestructureEnum01.Color.Red' ->
            io:put_chars([[<<"The color is Red!">>]]),
            io:nl();
        'Elixir.Patina.DestructureEnum01.Color.Blue' ->
            io:put_chars([[<<"The color is Blue!">>]]),
            io:nl();
        'Elixir.Patina.DestructureEnum01.Color.Green' ->
            io:put_chars([[<<"The color is Green!">>]]),
            io:nl();
        #'Elixir.Patina.DestructureEnum01.Color':'RGB'{'0' = _var_1,
                                                       '1' = _var_2,
                                                       '2' = _var_3} ->
            io:put_chars([[<<"Red: ">>,
                           io_lib:format("~tp", [_var_1]),
                           <<", green: ">>,
                           io_lib:format("~tp", [_var_2]),
                           <<", and blue: ">>,
                           io_lib:format("~tp", [_var_3]),
                           <<"!">>]]),
            io:nl();
        #'Elixir.Patina.DestructureEnum01.Color':'HSV'{'0' = _var_4,
                                                       '1' = _var_5,
                                                       '2' = _var_6} ->
            io:put_chars([[<<"Hue: ">>,
                           io_lib:format("~tp", [_var_4]),
                           <<", saturation: ">>,
                           io_lib:format("~tp", [_var_5]),
                           <<", value: ">>,
                           io_lib:format("~tp", [_var_6]),
                           <<"!">>]]),
            io:nl();
        #'Elixir.Patina.DestructureEnum01.Color':'HSL'{'0' = _var_7,
                                                       '1' = _var_8,
                                                       '2' = _var_9} ->
            io:put_chars([[<<"Hue: ">>,
                           io_lib:format("~tp", [_var_7]),
                           <<", saturation: ">>,
                           io_lib:format("~tp", [_var_8]),
                           <<", lightness: ">>,
                           io_lib:format("~tp", [_var_9]),
                           <<"!">>]]),
            io:nl();
        #'Elixir.Patina.DestructureEnum01.Color':'CMY'{'0' = _var_10,
                                                       '1' = _var_11,
                                                       '2' = _var_12} ->
            io:put_chars([[<<"Cyan: ">>,
                           io_lib:format("~tp", [_var_10]),
                           <<", magenta: ">>,
                           io_lib:format("~tp", [_var_11]),
                           <<", yellow: ">>,
                           io_lib:format("~tp", [_var_12]),
                           <<"!">>]]),
            io:nl();
        #'Elixir.Patina.DestructureEnum01.Color':'CMYK'{'0' = _var_13,
                                                        '1' = _var_14,
                                                        '2' = _var_15,
                                                        '3' = _var_16} ->
            io:put_chars([[<<"Cyan: ">>,
                           io_lib:format("~tp", [_var_13]),
                           <<", magenta: ">>,
                           io_lib:format("~tp", [_var_14]),
                           <<", yellow: ">>,
                           io_lib:format("~tp", [_var_15]),
                           <<", key (black): ">>,
                           io_lib:format("~tp", [_var_16]),
                           <<"!">>]]),
            io:nl()
    end.
