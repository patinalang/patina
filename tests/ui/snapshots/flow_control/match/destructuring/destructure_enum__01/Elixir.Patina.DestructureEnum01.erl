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
        io:put_chars([[<<"What color is it?"/utf8>>]]),
        io:nl(),
        {}
    end,
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
            io:put_chars([[<<"Red: "/utf8>>,
                           io_lib:format("~ktp", [_r@1_0]),
                           <<", green: "/utf8>>,
                           io_lib:format("~ktp", [_g@2_0]),
                           <<", and blue: "/utf8>>,
                           io_lib:format("~ktp", [_b@3_0]),
                           <<"!"/utf8>>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSV'{'0' =
                                                           _h@4_0,
                                                       '1' = _s@5_0,
                                                       '2' = _v@6_0} ->
            io:put_chars([[<<"Hue: "/utf8>>,
                           io_lib:format("~ktp", [_h@4_0]),
                           <<", saturation: "/utf8>>,
                           io_lib:format("~ktp", [_s@5_0]),
                           <<", value: "/utf8>>,
                           io_lib:format("~ktp", [_v@6_0]),
                           <<"!"/utf8>>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'HSL'{'0' =
                                                           _h@7_0,
                                                       '1' = _s@8_0,
                                                       '2' = _l@9_0} ->
            io:put_chars([[<<"Hue: "/utf8>>,
                           io_lib:format("~ktp", [_h@7_0]),
                           <<", saturation: "/utf8>>,
                           io_lib:format("~ktp", [_s@8_0]),
                           <<", lightness: "/utf8>>,
                           io_lib:format("~ktp", [_l@9_0]),
                           <<"!"/utf8>>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMY'{'0' =
                                                           _c@10_0,
                                                       '1' = _m@11_0,
                                                       '2' = _y@12_0} ->
            io:put_chars([[<<"Cyan: "/utf8>>,
                           io_lib:format("~ktp", [_c@10_0]),
                           <<", magenta: "/utf8>>,
                           io_lib:format("~ktp", [_m@11_0]),
                           <<", yellow: "/utf8>>,
                           io_lib:format("~ktp", [_y@12_0]),
                           <<"!"/utf8>>]]),
            io:nl(),
            {};
        #'Elixir.Patina.DestructureEnum01.Color':'CMYK'{'0' =
                                                            _c@13_0,
                                                        '1' = _m@14_0,
                                                        '2' = _y@15_0,
                                                        '3' = _k@16_0} ->
            io:put_chars([[<<"Cyan: "/utf8>>,
                           io_lib:format("~ktp", [_c@13_0]),
                           <<", magenta: "/utf8>>,
                           io_lib:format("~ktp", [_m@14_0]),
                           <<", yellow: "/utf8>>,
                           io_lib:format("~ktp", [_y@15_0]),
                           <<", key (black): "/utf8>>,
                           io_lib:format("~ktp", [_k@16_0]),
                           <<"!"/utf8>>]]),
            io:nl(),
            {}
    end.
