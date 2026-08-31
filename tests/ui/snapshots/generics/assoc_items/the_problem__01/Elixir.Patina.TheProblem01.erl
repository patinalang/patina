-file("tests/ui/src/generics/assoc_items/the_problem"
      "__01.pt",
      4).

-module('Elixir.Patina.TheProblem01').

-compile([no_auto_import]).

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

difference(_A@0_0, _B@1_0, _C@2_0, _container@3_0) ->
    (erlang:element(1,
                    _C@2_0)):'TheProblem01::Contains::last'(_C@2_0,
                                                            _A@0_0,
                                                            _B@1_0,
                                                            _container@3_0)
        -
        (erlang:element(1,
                        _C@2_0)):'TheProblem01::Contains::first'(_C@2_0,
                                                                 _A@0_0,
                                                                 _B@1_0,
                                                                 _container@3_0).

main() ->
    _number_1@0_0 = 3,
    _number_2@1_0 = 10,
    _container@2_0 =
        ('Elixir.Patina.TheProblem01.Container':'__struct__'())#{'0'
                                                                     :=
                                                                     _number_1@0_0,
                                                                 '1' :=
                                                                     _number_2@1_0},
    begin
        io:put_chars([[<<"Does container contain ">>,
                       io_lib:format("~ktp", [_number_1@0_0]),
                       <<" and ">>,
                       io_lib:format("~ktp", [_number_2@1_0]),
                       <<": ">>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains::contains'({'Elixir.Patina.TheProblem01.Container',
                                                                                                                 {}},
                                                                                                                {'Elixir.Patina.Core.int',
                                                                                                                 {}},
                                                                                                                {'Elixir.Patina.Core.int',
                                                                                                                 {}},
                                                                                                                _container@2_0,
                                                                                                                _number_1@0_0,
                                                                                                                _number_2@1_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"First number: ">>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains::first'({'Elixir.Patina.TheProblem01.Container',
                                                                                                              {}},
                                                                                                             {'Elixir.Patina.Core.int',
                                                                                                              {}},
                                                                                                             {'Elixir.Patina.Core.int',
                                                                                                              {}},
                                                                                                             _container@2_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"Last number: ">>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains::last'({'Elixir.Patina.TheProblem01.Container',
                                                                                                             {}},
                                                                                                            {'Elixir.Patina.Core.int',
                                                                                                             {}},
                                                                                                            {'Elixir.Patina.Core.int',
                                                                                                             {}},
                                                                                                            _container@2_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"The difference is: ">>,
                       io_lib:format("~ktp",
                                     [difference({'Elixir.Patina.Core.int', {}},
                                                 {'Elixir.Patina.Core.int', {}},
                                                 {'Elixir.Patina.TheProblem01.Container',
                                                  {}},
                                                 _container@2_0)])]]),
        io:nl(),
        {}
    end.
