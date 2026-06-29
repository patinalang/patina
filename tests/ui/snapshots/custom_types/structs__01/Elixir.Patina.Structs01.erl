-file("tests/ui/src/custom_types/structs__01.pt", 5).
-module('Elixir.Patina.Structs01').
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
    _var_0 = <<"Peter">>,
    _var_1 = 27,
    _var_2 =
        ('Elixir.Patina.Structs01.Person':'__struct__'())#{name :=
                                                               _var_0,
                                                           age := _var_1},
    begin
        io:put_chars([[io_lib:format("~tp", [_var_2])]]),
        io:nl()
    end,
    _var_3 =
        ('Elixir.Patina.Structs01.Point':'__struct__'())#{x := 5.2,
                                                          y := 0.4},
    _var_4 =
        ('Elixir.Patina.Structs01.Point':'__struct__'())#{x := 10.3,
                                                          y := 0.2},
    begin
        io:put_chars([[<<"point coordinates: (">>,
                       io_lib:format("~tp", [map_get(x, _var_3)]),
                       <<", ">>,
                       io_lib:format("~tp", [map_get(y, _var_3)]),
                       <<")">>]]),
        io:nl()
    end,
    _var_5 = _var_4#{x := 10.3},
    begin
        io:put_chars([[<<"second point: (">>,
                       io_lib:format("~tp", [map_get(x, _var_5)]),
                       <<", ">>,
                       io_lib:format("~tp", [map_get(y, _var_5)]),
                       <<")">>]]),
        io:nl()
    end,
    #{x := _var_6, y := _var_7} = _var_3,
    _var_8 =
        ('Elixir.Patina.Structs01.Rectangle':'__struct__'())#{top_left :=
                                                                  ('Elixir.Patina.Structs01.Point':'__struct__'())#{x :=
                                                                                                                       _var_6,
                                                                                                                   y :=
                                                                                                                       _var_7},
                                                              bottom_right :=
                                                                  _var_5},
    _var_9 = 'Elixir.Patina.Structs01.Unit',
    _var_10 =
        ('Elixir.Patina.Structs01.Pair':'__struct__'())#{'0' := 1,
                                                         '1' := 0.1},
    begin
        io:put_chars([[<<"pair contains ">>,
                       io_lib:format("~tp", [map_get('0', _var_10)]),
                       <<" and ">>,
                       io_lib:format("~tp", [map_get('1', _var_10)])]]),
        io:nl()
    end,
    #{'0' := _var_11, '1' := _var_12} = _var_10,
    begin
        io:put_chars([[<<"pair contains ">>,
                       io_lib:format("~tp", [_var_11]),
                       <<" and ">>,
                       io_lib:format("~tp", [_var_12])]]),
        io:nl()
    end.
