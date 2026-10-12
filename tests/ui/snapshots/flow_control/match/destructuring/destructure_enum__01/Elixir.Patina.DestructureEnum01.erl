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
    begin
        io:put_chars(begin
                         _element@1_0 = [<<"What color is it?"/utf8>>],
                         [_element@1_0]
                     end),
        io:nl(),
        {}
    end,
    case _color@0_0 of
        'Elixir.Patina.DestructureEnum01.Color.Red' ->
            io:put_chars(begin
                             _element@2_0 = [<<"The color is Red!"/utf8>>],
                             [_element@2_0]
                         end),
            io:nl(),
            {};
        'Elixir.Patina.DestructureEnum01.Color.Blue' ->
            io:put_chars(begin
                             _element@3_0 = [<<"The color is Blue!"/utf8>>],
                             [_element@3_0]
                         end),
            io:nl(),
            {};
        'Elixir.Patina.DestructureEnum01.Color.Green' ->
            io:put_chars(begin
                             _element@4_0 = [<<"The color is Green!"/utf8>>],
                             [_element@4_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'RGB'{'0' =
                                                           _r@5_0,
                                                       '1' = _g@6_0,
                                                       '2' = _b@7_0} ->
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@8_0 =
                                                     <<"Red: "/utf8>>,
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_r@5_0]),
                                                 _element@10_0 =
                                                     <<", green: "/utf8>>,
                                                 _element@11_0 =
                                                     io_lib:format("~ktp",
                                                                   [_g@6_0]),
                                                 _element@12_0 =
                                                     <<", and blue: "/utf8>>,
                                                 _element@13_0 =
                                                     io_lib:format("~ktp",
                                                                   [_b@7_0]),
                                                 _element@14_0 = <<"!"/utf8>>,
                                                 [_element@8_0,
                                                  _element@9_0,
                                                  _element@10_0,
                                                  _element@11_0,
                                                  _element@12_0,
                                                  _element@13_0,
                                                  _element@14_0]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSV'{'0' =
                                                           _h@16_0,
                                                       '1' = _s@17_0,
                                                       '2' = _v@18_0} ->
            io:put_chars(begin
                             _element@26_0 = begin
                                                 _element@19_0 =
                                                     <<"Hue: "/utf8>>,
                                                 _element@20_0 =
                                                     io_lib:format("~ktp",
                                                                   [_h@16_0]),
                                                 _element@21_0 =
                                                     <<", saturation: "/utf8>>,
                                                 _element@22_0 =
                                                     io_lib:format("~ktp",
                                                                   [_s@17_0]),
                                                 _element@23_0 =
                                                     <<", value: "/utf8>>,
                                                 _element@24_0 =
                                                     io_lib:format("~ktp",
                                                                   [_v@18_0]),
                                                 _element@25_0 = <<"!"/utf8>>,
                                                 [_element@19_0,
                                                  _element@20_0,
                                                  _element@21_0,
                                                  _element@22_0,
                                                  _element@23_0,
                                                  _element@24_0,
                                                  _element@25_0]
                                             end,
                             [_element@26_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSL'{'0' =
                                                           _h@27_0,
                                                       '1' = _s@28_0,
                                                       '2' = _l@29_0} ->
            io:put_chars(begin
                             _element@37_0 = begin
                                                 _element@30_0 =
                                                     <<"Hue: "/utf8>>,
                                                 _element@31_0 =
                                                     io_lib:format("~ktp",
                                                                   [_h@27_0]),
                                                 _element@32_0 =
                                                     <<", saturation: "/utf8>>,
                                                 _element@33_0 =
                                                     io_lib:format("~ktp",
                                                                   [_s@28_0]),
                                                 _element@34_0 =
                                                     <<", lightness: "/utf8>>,
                                                 _element@35_0 =
                                                     io_lib:format("~ktp",
                                                                   [_l@29_0]),
                                                 _element@36_0 = <<"!"/utf8>>,
                                                 [_element@30_0,
                                                  _element@31_0,
                                                  _element@32_0,
                                                  _element@33_0,
                                                  _element@34_0,
                                                  _element@35_0,
                                                  _element@36_0]
                                             end,
                             [_element@37_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMY'{'0' =
                                                           _c@38_0,
                                                       '1' = _m@39_0,
                                                       '2' = _y@40_0} ->
            io:put_chars(begin
                             _element@48_0 = begin
                                                 _element@41_0 =
                                                     <<"Cyan: "/utf8>>,
                                                 _element@42_0 =
                                                     io_lib:format("~ktp",
                                                                   [_c@38_0]),
                                                 _element@43_0 =
                                                     <<", magenta: "/utf8>>,
                                                 _element@44_0 =
                                                     io_lib:format("~ktp",
                                                                   [_m@39_0]),
                                                 _element@45_0 =
                                                     <<", yellow: "/utf8>>,
                                                 _element@46_0 =
                                                     io_lib:format("~ktp",
                                                                   [_y@40_0]),
                                                 _element@47_0 = <<"!"/utf8>>,
                                                 [_element@41_0,
                                                  _element@42_0,
                                                  _element@43_0,
                                                  _element@44_0,
                                                  _element@45_0,
                                                  _element@46_0,
                                                  _element@47_0]
                                             end,
                             [_element@48_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMYK'{'0' =
                                                            _c@49_0,
                                                        '1' = _m@50_0,
                                                        '2' = _y@51_0,
                                                        '3' = _k@52_0} ->
            io:put_chars(begin
                             _element@62_0 = begin
                                                 _element@53_0 =
                                                     <<"Cyan: "/utf8>>,
                                                 _element@54_0 =
                                                     io_lib:format("~ktp",
                                                                   [_c@49_0]),
                                                 _element@55_0 =
                                                     <<", magenta: "/utf8>>,
                                                 _element@56_0 =
                                                     io_lib:format("~ktp",
                                                                   [_m@50_0]),
                                                 _element@57_0 =
                                                     <<", yellow: "/utf8>>,
                                                 _element@58_0 =
                                                     io_lib:format("~ktp",
                                                                   [_y@51_0]),
                                                 _element@59_0 =
                                                     <<", key (black): "/utf8>>,
                                                 _element@60_0 =
                                                     io_lib:format("~ktp",
                                                                   [_k@52_0]),
                                                 _element@61_0 = <<"!"/utf8>>,
                                                 [_element@53_0,
                                                  _element@54_0,
                                                  _element@55_0,
                                                  _element@56_0,
                                                  _element@57_0,
                                                  _element@58_0,
                                                  _element@59_0,
                                                  _element@60_0,
                                                  _element@61_0]
                                             end,
                             [_element@62_0]
                         end),
            io:nl(),
            {}
    end.
