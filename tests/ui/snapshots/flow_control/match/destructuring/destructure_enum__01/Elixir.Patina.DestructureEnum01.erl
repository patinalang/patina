-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_enum__01.pt",
      6).

-module('Elixir.Patina.DestructureEnum01').

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
    _color@0_0 =
        'Elixir.Patina.DestructureEnum01.Color':'$patina_constructor__RGB'(122,
                                                                           17,
                                                                           40),
    io:put_chars([[<<"What color is it?"/utf8>>]]),
    io:nl(),
    case _color@0_0 of
        'Elixir.Patina.DestructureEnum01.Color.Red' ->
            io:put_chars([[<<"The color is Red!"/utf8>>]]),
            io:nl(),
            {};
        'Elixir.Patina.DestructureEnum01.Color.Blue' ->
            io:put_chars([[<<"The color is Blue!"/utf8>>]]),
            io:nl(),
            {};
        'Elixir.Patina.DestructureEnum01.Color.Green' ->
            io:put_chars([[<<"The color is Green!"/utf8>>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'RGB'{'0' =
                                                           _r@1_0,
                                                       '1' = _g@2_0,
                                                       '2' = _b@3_0} ->
            io:put_chars(begin
                             _element@7_0 = begin
                                                _element@4_0 =
                                                    io_lib:format("~ktp",
                                                                  [_r@1_0]),
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  [_g@2_0]),
                                                _element@6_0 =
                                                    io_lib:format("~ktp",
                                                                  [_b@3_0]),
                                                [<<"Red: "/utf8>>,
                                                 _element@4_0,
                                                 <<", green: "/utf8>>,
                                                 _element@5_0,
                                                 <<", and blue: "/utf8>>,
                                                 _element@6_0,
                                                 <<"!"/utf8>>]
                                            end,
                             [_element@7_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSV'{'0' =
                                                           _h@8_0,
                                                       '1' = _s@9_0,
                                                       '2' = _v@10_0} ->
            io:put_chars(begin
                             _element@14_0 = begin
                                                 _element@11_0 =
                                                     io_lib:format("~ktp",
                                                                   [_h@8_0]),
                                                 _element@12_0 =
                                                     io_lib:format("~ktp",
                                                                   [_s@9_0]),
                                                 _element@13_0 =
                                                     io_lib:format("~ktp",
                                                                   [_v@10_0]),
                                                 [<<"Hue: "/utf8>>,
                                                  _element@11_0,
                                                  <<", saturation: "/utf8>>,
                                                  _element@12_0,
                                                  <<", value: "/utf8>>,
                                                  _element@13_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@14_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSL'{'0' =
                                                           _h@15_0,
                                                       '1' = _s@16_0,
                                                       '2' = _l@17_0} ->
            io:put_chars(begin
                             _element@21_0 = begin
                                                 _element@18_0 =
                                                     io_lib:format("~ktp",
                                                                   [_h@15_0]),
                                                 _element@19_0 =
                                                     io_lib:format("~ktp",
                                                                   [_s@16_0]),
                                                 _element@20_0 =
                                                     io_lib:format("~ktp",
                                                                   [_l@17_0]),
                                                 [<<"Hue: "/utf8>>,
                                                  _element@18_0,
                                                  <<", saturation: "/utf8>>,
                                                  _element@19_0,
                                                  <<", lightness: "/utf8>>,
                                                  _element@20_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@21_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMY'{'0' =
                                                           _c@22_0,
                                                       '1' = _m@23_0,
                                                       '2' = _y@24_0} ->
            io:put_chars(begin
                             _element@28_0 = begin
                                                 _element@25_0 =
                                                     io_lib:format("~ktp",
                                                                   [_c@22_0]),
                                                 _element@26_0 =
                                                     io_lib:format("~ktp",
                                                                   [_m@23_0]),
                                                 _element@27_0 =
                                                     io_lib:format("~ktp",
                                                                   [_y@24_0]),
                                                 [<<"Cyan: "/utf8>>,
                                                  _element@25_0,
                                                  <<", magenta: "/utf8>>,
                                                  _element@26_0,
                                                  <<", yellow: "/utf8>>,
                                                  _element@27_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@28_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMYK'{'0' =
                                                            _c@29_0,
                                                        '1' = _m@30_0,
                                                        '2' = _y@31_0,
                                                        '3' = _k@32_0} ->
            io:put_chars(begin
                             _element@37_0 = begin
                                                 _element@33_0 =
                                                     io_lib:format("~ktp",
                                                                   [_c@29_0]),
                                                 _element@34_0 =
                                                     io_lib:format("~ktp",
                                                                   [_m@30_0]),
                                                 _element@35_0 =
                                                     io_lib:format("~ktp",
                                                                   [_y@31_0]),
                                                 _element@36_0 =
                                                     io_lib:format("~ktp",
                                                                   [_k@32_0]),
                                                 [<<"Cyan: "/utf8>>,
                                                  _element@33_0,
                                                  <<", magenta: "/utf8>>,
                                                  _element@34_0,
                                                  <<", yellow: "/utf8>>,
                                                  _element@35_0,
                                                  <<", key (black): "/utf8>>,
                                                  _element@36_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@37_0]
                         end),
            io:nl(),
            {}
    end.
