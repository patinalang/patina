-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_structures__01.pt",
      4).

-module('Elixir.Patina.DestructureStructures01').

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
    _foo@0_0 =
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = {1, 2},
                                                                y = 3},
    case _foo@0_0 of
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    =
                                                                    {1, _b@1_0},
                                                                y = _y@2_0} ->
            io:put_chars([[<<"First of x is 1, b = ">>,
                           io_lib:format("~ktp", [_b@1_0]),
                           <<",  y = ">>,
                           io_lib:format("~ktp", [_y@2_0]),
                           <<" ">>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = _i@3_0,
                                                                y = 2} ->
            io:put_chars([[<<"y is 2, i = ">>,
                           io_lib:format("~ktp", [_i@3_0])]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = _,
                                                                y = _y@4_0} ->
            io:put_chars([[<<"y = ">>,
                           io_lib:format("~ktp", [_y@4_0]),
                           <<", we don't care about x">>]]),
            io:nl(),
            {}
    end,
    _faa@5_0 =
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = {1, 2},
                                                                y = 3},
    #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                = _x0@6_0,
                                                            y = _y0@7_0} =
        _faa@5_0,
    begin
        io:put_chars([[<<"Outside: x0 = ">>,
                       io_lib:format("~ktp", [_x0@6_0]),
                       <<", y0 = ">>,
                       io_lib:format("~ktp", [_y0@7_0])]]),
        io:nl(),
        {}
    end,
    _bar@8_0 = _faa@5_0,
    #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                = _nested_x@9_0,
                                                            y =
                                                                _nested_y@10_0} =
        _bar@8_0,
    begin
        io:put_chars([[<<"Nested: nested_x = ">>,
                       io_lib:format("~ktp", [_nested_x@9_0]),
                       <<", nested_y = ">>,
                       io_lib:format("~ktp", [_nested_y@10_0])]]),
        io:nl(),
        {}
    end.
