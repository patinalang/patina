-file("tests/ui/src/primitives/lists__01.pt", 4).

-module('Elixir.Patina.Lists01').

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
    case [] of
        {'$patina_label_0'} -> {};
        _ ->
            _xs@0_0 = [1, 2, 3, 4, 5],
            case 'Elixir.Patina.Core.List':get(_xs@0_0, 0) of
                _value@2_0 = _xval@1_0
                    when erlang:is_integer(_value@2_0) ->
                    io:put_chars(begin
                                     _element@4_0 = begin
                                                        _element@3_0 =
                                                            io_lib:format("~ktp",
                                                                          [_xval@1_0]),
                                                        [<<"elem 0: "/utf8>>,
                                                         _element@3_0]
                                                    end,
                                     [_element@4_0]
                                 end),
                    io:nl(),
                    {};
                nil ->
                    io:put_chars([[<<"the list is empty!"/utf8>>]]),
                    io:nl(),
                    {}
            end,
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  ['Elixir.Patina.Core.List':len(_xs@0_0)]),
                                                [<<"Number of elements in array: "/utf8>>,
                                                 _element@5_0]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@8_0 = begin
                                                _element@7_0 =
                                                    io_lib:format("~ktp",
                                                                  ['Elixir.Patina.Core.List':head(_xs@0_0)]),
                                                [<<"First element of the list: "/utf8>>,
                                                 _element@7_0]
                                            end,
                             [_element@8_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   ['Elixir.Patina.Core.List':tail(_xs@0_0)]),
                                                 [<<"Rest of the list: "/utf8>>,
                                                  _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            _more_xs@11_0 = [-1, 0 | _xs@0_0],
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@14_0 =
                                                     io_lib:format("~ktp",
                                                                   [begin
                                                                        _self@12_0 =
                                                                            'Elixir.Patina.Core.List':tail('Elixir.Patina.Core.List':tail(_more_xs@11_0)),
                                                                        _other@13_0 =
                                                                            _xs@0_0,
                                                                        _self@12_0
                                                                            =:=
                                                                            _other@13_0
                                                                    end]),
                                                 [<<"more_xs.tail().tail() is equal to xs: "/utf8>>,
                                                  _element@14_0]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            _rest@16_0 = {},
            _first@17_0 = {},
            _second@18_0 = {},
            _third@19_0 = {},
            case _more_xs@11_0 of
                [_first@17_1, _second@18_1, _third@19_1 | _rest@16_1 =
                                                              _] ->
                    {};
                _ ->
                    _branch_value@20_0 = begin
                                             '$patina_marker':jump('$patina_label_0'),
                                             []
                                         end,
                    _rest@16_1 = _rest@16_0,
                    _first@17_1 = _first@17_0,
                    _second@18_1 = _second@18_0,
                    _third@19_1 = _third@19_0,
                    _branch_value@20_0
            end,
            io:put_chars(begin
                             _element@22_0 = begin
                                                 _element@21_0 =
                                                     io_lib:format("~ktp",
                                                                   [_first@17_1
                                                                        +
                                                                        _second@18_1
                                                                        +
                                                                        _third@19_1]),
                                                 [<<"Sum of first three elements is "/utf8>>,
                                                  _element@21_0]
                                             end,
                             [_element@22_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@24_0 = begin
                                                 _element@23_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@16_1]),
                                                 [<<"...and the rest: "/utf8>>,
                                                  _element@23_0]
                                             end,
                             [_element@24_0]
                         end),
            io:nl(),
            {}
    end.
