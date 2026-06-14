-file("tests/ui/src/custom_types/structs__01.pt", 5).
-module('Elixir.Patina.Structs01').
-compile([no_auto_import]).
-export([main/0]).
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

-file("tests/ui/src/custom_types/structs__01.pt", 9).
-module('Elixir.Patina.Structs01.Person').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Structs01.Person',
      name => nil,
      age => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).

-file("tests/ui/src/custom_types/structs__01.pt", 15).
-module('Elixir.Patina.Structs01.Unit').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Structs01.Unit'}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).

-file("tests/ui/src/custom_types/structs__01.pt", 18).
-module('Elixir.Patina.Structs01.Pair').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Structs01.Pair',
      '0' => nil,
      '1' => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).

-file("tests/ui/src/custom_types/structs__01.pt", 21).
-module('Elixir.Patina.Structs01.Point').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Structs01.Point',
      x => nil,
      y => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).

-file("tests/ui/src/custom_types/structs__01.pt", 27).
-module('Elixir.Patina.Structs01.Rectangle').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Structs01.Rectangle',
      top_left => nil,
      bottom_right => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
