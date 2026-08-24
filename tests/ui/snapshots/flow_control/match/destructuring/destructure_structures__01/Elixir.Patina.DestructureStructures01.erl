-file("tests/ui/src/flow_control/match/destructuring/destructure_struct"
      "ures__01.pt",
      4).
-module('Elixir.Patina.DestructureStructures01').
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
    _foo@0_0 =
        ('Elixir.Patina.DestructureStructures01.main.Foo':'__struct__'())#{x :=
                                                                               {1,
                                                                                2},
                                                                           y :=
                                                                               3},
    case _foo@0_0 of
        #{'__struct__' :=
              'Elixir.Patina.DestructureStructures01.main.Foo',
          x := {1, _b@1_0},
          y := _y@2_0} ->
            io:put_chars([[<<"First of x is 1, b = ">>,
                           io_lib:format("~ktp", [_b@1_0]),
                           <<",  y = ">>,
                           io_lib:format("~ktp", [_y@2_0]),
                           <<" ">>]]),
            io:nl();
        #{'__struct__' :=
              'Elixir.Patina.DestructureStructures01.main.Foo',
          y := 2,
          x := _i@3_0} ->
            io:put_chars([[<<"y is 2, i = ">>,
                           io_lib:format("~ktp", [_i@3_0])]]),
            io:nl();
        #{'__struct__' :=
              'Elixir.Patina.DestructureStructures01.main.Foo',
          y := _y@4_0} ->
            io:put_chars([[<<"y = ">>,
                           io_lib:format("~ktp", [_y@4_0]),
                           <<", we don't care about x">>]]),
            io:nl()
    end,
    _faa@5_0 =
        ('Elixir.Patina.DestructureStructures01.main.Foo':'__struct__'())#{x :=
                                                                               {1,
                                                                                2},
                                                                           y :=
                                                                               3},
    #{'__struct__' := 'Elixir.Patina.DestructureStructures01.main.Foo',
      x := _x0@6_0,
      y := _y0@7_0} =
        _faa@5_0,
    begin
        io:put_chars([[<<"Outside: x0 = ">>,
                       io_lib:format("~ktp", [_x0@6_0]),
                       <<", y0 = ">>,
                       io_lib:format("~ktp", [_y0@7_0])]]),
        io:nl()
    end,
    _bar@8_0 =
        ('Elixir.Patina.DestructureStructures01.main.Bar':'__struct__'())#{foo :=
                                                                               _faa@5_0},
    #{'__struct__' := 'Elixir.Patina.DestructureStructures01.main.Bar',
      foo :=
          #{'__struct__' :=
                'Elixir.Patina.DestructureStructures01.main.Foo',
            x := _nested_x@9_0,
            y := _nested_y@10_0}} =
        _bar@8_0,
    begin
        io:put_chars([[<<"Nested: nested_x = ">>,
                       io_lib:format("~ktp", [_nested_x@9_0]),
                       <<", nested_y = ">>,
                       io_lib:format("~ktp", [_nested_y@10_0])]]),
        io:nl()
    end.
