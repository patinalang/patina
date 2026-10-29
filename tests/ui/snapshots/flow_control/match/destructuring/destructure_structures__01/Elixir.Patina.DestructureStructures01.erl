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
        'Elixir.Patina.DestructureStructures01.main.Foo':'$patina_constructor__Foo'({1,
                                                                                     2},
                                                                                    3),
    case _foo@0_0 of
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    =
                                                                    {1, _b@1_0},
                                                                y = _y@2_0} ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_b@1_0]),
                                                _element@4_0 =
                                                    io_lib:format("~ktp",
                                                                  [_y@2_0]),
                                                [<<"First of x is 1, b = "/utf8>>,
                                                 _element@3_0,
                                                 <<",  y = "/utf8>>,
                                                 _element@4_0,
                                                 <<" "/utf8>>]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = _i@6_0,
                                                                y = 2} ->
            io:put_chars(begin
                             _element@8_0 = begin
                                                _element@7_0 =
                                                    io_lib:format("~ktp",
                                                                  [_i@6_0]),
                                                [<<"y is 2, i = "/utf8>>,
                                                 _element@7_0]
                                            end,
                             [_element@8_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                    = _,
                                                                y = _y@9_0} ->
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_y@9_0]),
                                                 [<<"y = "/utf8>>,
                                                  _element@10_0,
                                                  <<", we don't care about x"/utf8>>]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            {}
    end,
    _faa@12_0 =
        'Elixir.Patina.DestructureStructures01.main.Foo':'$patina_constructor__Foo'({1,
                                                                                     2},
                                                                                    3),
    #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                = _x0@13_0,
                                                            y = _y0@14_0} =
        _faa@12_0,
    io:put_chars(begin
                     _element@17_0 = begin
                                         _element@15_0 = io_lib:format("~ktp",
                                                                       [_x0@13_0]),
                                         _element@16_0 = io_lib:format("~ktp",
                                                                       [_y0@14_0]),
                                         [<<"Outside: x0 = "/utf8>>,
                                          _element@15_0,
                                          <<", y0 = "/utf8>>,
                                          _element@16_0]
                                     end,
                     [_element@17_0]
                 end),
    io:nl(),
    _bar@18_0 = _faa@12_0,
    #'Elixir.Patina.DestructureStructures01.main.Foo':'Foo'{x
                                                                =
                                                                _nested_x@19_0,
                                                            y =
                                                                _nested_y@20_0} =
        _bar@18_0,
    io:put_chars(begin
                     _element@23_0 = begin
                                         _element@21_0 = io_lib:format("~ktp",
                                                                       [_nested_x@19_0]),
                                         _element@22_0 = io_lib:format("~ktp",
                                                                       [_nested_y@20_0]),
                                         [<<"Nested: nested_x = "/utf8>>,
                                          _element@21_0,
                                          <<", nested_y = "/utf8>>,
                                          _element@22_0]
                                     end,
                     [_element@23_0]
                 end),
    io:nl(),
    {}.
