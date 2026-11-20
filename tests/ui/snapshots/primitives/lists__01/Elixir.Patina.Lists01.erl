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
            case begin
                     _self@1_0 = _xs@0_0,
                     _index@2_0 = 0,
                     case _self@1_0 of
                         [_head@3_0 | _] when _index@2_0 =:= 0 -> _head@3_0;
                         [_ | _tail@4_0 = _] when _index@2_0 > 0 ->
                             'Elixir.Patina.Core.List':get(_tail@4_0,
                                                           _index@2_0 - 1);
                         _ -> nil
                     end
                 end
                of
                _value@6_0 = _xval@5_0
                    when erlang:is_integer(_value@6_0) ->
                    io:put_chars(begin
                                     _element@8_0 = begin
                                                        _element@7_0 =
                                                            io_lib:format("~ktp",
                                                                          [_xval@5_0]),
                                                        [<<"elem 0: "/utf8>>,
                                                         _element@7_0]
                                                    end,
                                     [_element@8_0]
                                 end),
                    io:nl(),
                    {};
                nil ->
                    io:put_chars([[<<"the list is empty!"/utf8>>]]),
                    io:nl(),
                    {}
            end,
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [begin
                                                                        _self@9_0 =
                                                                            _xs@0_0,
                                                                        erlang:length(_self@9_0)
                                                                    end]),
                                                 [<<"Number of elements in array: "/utf8>>,
                                                  _element@10_0]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@14_0 =
                                                     io_lib:format("~ktp",
                                                                   [begin
                                                                        _self@12_0 =
                                                                            _xs@0_0,
                                                                        case
                                                                            _self@12_0
                                                                            of
                                                                            [_head@13_0
                                                                             | _] ->
                                                                                _head@13_0;
                                                                            [] ->
                                                                                nil
                                                                        end
                                                                    end]),
                                                 [<<"First element of the list: "/utf8>>,
                                                  _element@14_0]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@19_0 = begin
                                                 _element@18_0 =
                                                     io_lib:format("~ktp",
                                                                   [begin
                                                                        _self@16_0 =
                                                                            _xs@0_0,
                                                                        case
                                                                            _self@16_0
                                                                            of
                                                                            [_
                                                                             | _tail@17_0 =
                                                                                   _] ->
                                                                                _tail@17_0;
                                                                            [] ->
                                                                                []
                                                                        end
                                                                    end]),
                                                 [<<"Rest of the list: "/utf8>>,
                                                  _element@18_0]
                                             end,
                             [_element@19_0]
                         end),
            io:nl(),
            _more_xs@20_0 = [-1, 0 | _xs@0_0],
            io:put_chars(begin
                             _element@28_0 = begin
                                                 _element@27_0 =
                                                     io_lib:format("~ktp",
                                                                   [begin
                                                                        _self@25_0 =
                                                                            begin
                                                                                _self@23_0 =
                                                                                    begin
                                                                                        _self@21_0 =
                                                                                            _more_xs@20_0,
                                                                                        case
                                                                                            _self@21_0
                                                                                            of
                                                                                            [_
                                                                                             | _tail@22_0 =
                                                                                                   _] ->
                                                                                                _tail@22_0;
                                                                                            [] ->
                                                                                                []
                                                                                        end
                                                                                    end,
                                                                                case
                                                                                    _self@23_0
                                                                                    of
                                                                                    [_
                                                                                     | _tail@24_0 =
                                                                                           _] ->
                                                                                        _tail@24_0;
                                                                                    [] ->
                                                                                        []
                                                                                end
                                                                            end,
                                                                        _other@26_0 =
                                                                            _xs@0_0,
                                                                        _self@25_0
                                                                            =:=
                                                                            _other@26_0
                                                                    end]),
                                                 [<<"more_xs.tail().tail() is equal to xs: "/utf8>>,
                                                  _element@27_0]
                                             end,
                             [_element@28_0]
                         end),
            io:nl(),
            _rest@29_0 = {},
            _first@30_0 = {},
            _second@31_0 = {},
            _third@32_0 = {},
            case _more_xs@20_0 of
                [_first@30_1, _second@31_1, _third@32_1 | _rest@29_1 =
                                                              _] ->
                    {};
                _ ->
                    _branch_value@33_0 = begin
                                             '$patina_marker':jump('$patina_label_0'),
                                             []
                                         end,
                    _rest@29_1 = _rest@29_0,
                    _first@30_1 = _first@30_0,
                    _second@31_1 = _second@31_0,
                    _third@32_1 = _third@32_0,
                    _branch_value@33_0
            end,
            io:put_chars(begin
                             _element@35_0 = begin
                                                 _element@34_0 =
                                                     io_lib:format("~ktp",
                                                                   [_first@30_1
                                                                        +
                                                                        _second@31_1
                                                                        +
                                                                        _third@32_1]),
                                                 [<<"Sum of first three elements is "/utf8>>,
                                                  _element@34_0]
                                             end,
                             [_element@35_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@37_0 = begin
                                                 _element@36_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@29_1]),
                                                 [<<"...and the rest: "/utf8>>,
                                                  _element@36_0]
                                             end,
                             [_element@37_0]
                         end),
            io:nl(),
            {}
    end.
