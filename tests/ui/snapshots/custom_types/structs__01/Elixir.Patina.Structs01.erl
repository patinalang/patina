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
        'Elixir.Patina.Structs01.Person':'$patina_constructor__Person'(_name@0_0,
                                                                       _age@1_0),
    begin
        io:put_chars(begin
                         _element@4_0 = begin
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [_peter@2_0]),
                                            [_element@3_0]
                                        end,
                         [_element@4_0]
                     end),
        io:nl(),
        {}
    end,
    _point@5_0 =
        'Elixir.Patina.Structs01.Point':'$patina_constructor__Point'(5.2,
                                                                     4.0e-1),
    _another_point@6_0 =
        'Elixir.Patina.Structs01.Point':'$patina_constructor__Point'(1.03e+1,
                                                                     2.0e-1),
    begin
        io:put_chars(begin
                         _element@12_0 = begin
                                             _element@7_0 =
                                                 <<"point coordinates: ("/utf8>>,
                                             _element@8_0 =
                                                 io_lib:format("~ktp",
                                                               [_point@5_0#'Elixir.Patina.Structs01.Point':'Point'.x]),
                                             _element@9_0 = <<", "/utf8>>,
                                             _element@10_0 =
                                                 io_lib:format("~ktp",
                                                               [_point@5_0#'Elixir.Patina.Structs01.Point':'Point'.y]),
                                             _element@11_0 = <<")"/utf8>>,
                                             [_element@7_0,
                                              _element@8_0,
                                              _element@9_0,
                                              _element@10_0,
                                              _element@11_0]
                                         end,
                         [_element@12_0]
                     end),
        io:nl(),
        {}
    end,
    _bottom_right@13_0 =
        _another_point@6_0#'Elixir.Patina.Structs01.Point':'Point'{x
                                                                       =
                                                                       1.03e+1},
    begin
        io:put_chars(begin
                         _element@19_0 = begin
                                             _element@14_0 =
                                                 <<"second point: ("/utf8>>,
                                             _element@15_0 =
                                                 io_lib:format("~ktp",
                                                               [_bottom_right@13_0#'Elixir.Patina.Structs01.Point':'Point'.x]),
                                             _element@16_0 = <<", "/utf8>>,
                                             _element@17_0 =
                                                 io_lib:format("~ktp",
                                                               [_bottom_right@13_0#'Elixir.Patina.Structs01.Point':'Point'.y]),
                                             _element@18_0 = <<")"/utf8>>,
                                             [_element@14_0,
                                              _element@15_0,
                                              _element@16_0,
                                              _element@17_0,
                                              _element@18_0]
                                         end,
                         [_element@19_0]
                     end),
        io:nl(),
        {}
    end,
    #'Elixir.Patina.Structs01.Point':'Point'{x =
                                                 _left_edge@20_0,
                                             y = _top_edge@21_0} =
        _point@5_0,
    __rectangle@23_0 = begin
                           _top_left@22_0 =
                               'Elixir.Patina.Structs01.Point':'$patina_constructor__Point'(_left_edge@20_0,
                                                                                            _top_edge@21_0),
                           'Elixir.Patina.Structs01.Rectangle':'$patina_constructor__Rectangle'(_top_left@22_0,
                                                                                                _bottom_right@13_0)
                       end,
    __unit@24_0 = 'Elixir.Patina.Structs01.Unit',
    _pair@25_0 =
        'Elixir.Patina.Structs01.Pair':'$patina_constructor__Pair'(1,
                                                                   1.0e-1),
    begin
        io:put_chars(begin
                         _element@30_0 = begin
                                             _element@26_0 =
                                                 <<"pair contains "/utf8>>,
                                             _element@27_0 =
                                                 io_lib:format("~ktp",
                                                               [_pair@25_0#'Elixir.Patina.Structs01.Pair':'Pair'.'0']),
                                             _element@28_0 = <<" and "/utf8>>,
                                             _element@29_0 =
                                                 io_lib:format("~ktp",
                                                               [_pair@25_0#'Elixir.Patina.Structs01.Pair':'Pair'.'1']),
                                             [_element@26_0,
                                              _element@27_0,
                                              _element@28_0,
                                              _element@29_0]
                                         end,
                         [_element@30_0]
                     end),
        io:nl(),
        {}
    end,
    #'Elixir.Patina.Structs01.Pair':'Pair'{'0' =
                                               _integer@31_0,
                                           '1' = _decimal@32_0} =
        _pair@25_0,
    begin
        io:put_chars(begin
                         _element@37_0 = begin
                                             _element@33_0 =
                                                 <<"pair contains "/utf8>>,
                                             _element@34_0 =
                                                 io_lib:format("~ktp",
                                                               [_integer@31_0]),
                                             _element@35_0 = <<" and "/utf8>>,
                                             _element@36_0 =
                                                 io_lib:format("~ktp",
                                                               [_decimal@32_0]),
                                             [_element@33_0,
                                              _element@34_0,
                                              _element@35_0,
                                              _element@36_0]
                                         end,
                         [_element@37_0]
                     end),
        io:nl(),
        {}
    end.
