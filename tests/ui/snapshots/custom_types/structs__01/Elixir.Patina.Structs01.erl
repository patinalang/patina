-file("tests/ui/src/custom_types/structs__01.pt", 5).

-module('Elixir.Patina.Structs01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

main() ->
    _name@0_0 = <<"Peter"/utf8>>,
    _age@1_0 = 27,
    _peter@2_0 =
        #'Elixir.Patina.Structs01.Person':'Person'{name =
                                                       _name@0_0,
                                                   age = _age@1_0},
    begin
        io:put_chars([[io_lib:format("~ktp", [_peter@2_0])]]),
        io:nl(),
        {}
    end,
    _point@3_0 = #'Elixir.Patina.Structs01.Point':'Point'{x
                                                              = 5.2,
                                                          y = 4.0e-1},
    _another_point@4_0 =
        #'Elixir.Patina.Structs01.Point':'Point'{x = 1.03e+1,
                                                 y = 2.0e-1},
    begin
        io:put_chars([[<<"point coordinates: ("/utf8>>,
                       io_lib:format("~ktp",
                                     [_point@3_0#'Elixir.Patina.Structs01.Point':'Point'.x]),
                       <<", "/utf8>>,
                       io_lib:format("~ktp",
                                     [_point@3_0#'Elixir.Patina.Structs01.Point':'Point'.y]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    _bottom_right@5_0 =
        _another_point@4_0#'Elixir.Patina.Structs01.Point':'Point'{x
                                                                       =
                                                                       1.03e+1},
    begin
        io:put_chars([[<<"second point: ("/utf8>>,
                       io_lib:format("~ktp",
                                     [_bottom_right@5_0#'Elixir.Patina.Structs01.Point':'Point'.x]),
                       <<", "/utf8>>,
                       io_lib:format("~ktp",
                                     [_bottom_right@5_0#'Elixir.Patina.Structs01.Point':'Point'.y]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    #'Elixir.Patina.Structs01.Point':'Point'{x =
                                                 _left_edge@6_0,
                                             y = _top_edge@7_0} =
        _point@3_0,
    __rectangle@8_0 =
        #'Elixir.Patina.Structs01.Rectangle':'Rectangle'{top_left
                                                             =
                                                             #'Elixir.Patina.Structs01.Point':'Point'{x
                                                                                                          =
                                                                                                          _left_edge@6_0,
                                                                                                      y
                                                                                                          =
                                                                                                          _top_edge@7_0},
                                                         bottom_right =
                                                             _bottom_right@5_0},
    __unit@9_0 = 'Elixir.Patina.Structs01.Unit',
    _pair@10_0 = #'Elixir.Patina.Structs01.Pair':'Pair'{'0'
                                                            = 1,
                                                        '1' = 1.0e-1},
    begin
        io:put_chars([[<<"pair contains "/utf8>>,
                       io_lib:format("~ktp",
                                     [_pair@10_0#'Elixir.Patina.Structs01.Pair':'Pair'.'0']),
                       <<" and "/utf8>>,
                       io_lib:format("~ktp",
                                     [_pair@10_0#'Elixir.Patina.Structs01.Pair':'Pair'.'1'])]]),
        io:nl(),
        {}
    end,
    #'Elixir.Patina.Structs01.Pair':'Pair'{'0' =
                                               _integer@11_0,
                                           '1' = _decimal@12_0} =
        _pair@10_0,
    begin
        io:put_chars([[<<"pair contains "/utf8>>,
                       io_lib:format("~ktp", [_integer@11_0]),
                       <<" and "/utf8>>,
                       io_lib:format("~ktp", [_decimal@12_0])]]),
        io:nl(),
        {}
    end.
