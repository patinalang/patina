-file("tests/ui/src/generics/assoc_items/types__03.pt", 4).
-module('Elixir.Patina.Types03').
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
difference(_C@0_0, _container@1_0) ->
    (element(1, _C@0_0)):'Types03::Contains::last'(_C@0_0,
                                                   _container@1_0)
    -
    (element(1, _C@0_0)):'Types03::Contains::first'(_C@0_0,
                                                    _container@1_0).
main() ->
    _number_1@0_0 = 3,
    _number_2@1_0 = 10,
    _container@2_0 =
        ('Elixir.Patina.Types03.Container':'__struct__'())#{'0' :=
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
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::contains'({'Elixir.Patina.Types03.Container',
                                                                                                       {}},
                                                                                                      _container@2_0,
                                                                                                      _number_1@0_0,
                                                                                                      _number_2@1_0)])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"First number: ">>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::first'({'Elixir.Patina.Types03.Container',
                                                                                                    {}},
                                                                                                   _container@2_0)])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"Last number: ">>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::last'({'Elixir.Patina.Types03.Container',
                                                                                                   {}},
                                                                                                  _container@2_0)])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"The difference is: ">>,
                       io_lib:format("~ktp",
                                     [difference({'Elixir.Patina.Types03.Container',
                                                  {}},
                                                 _container@2_0)])]]),
        io:nl()
    end.
