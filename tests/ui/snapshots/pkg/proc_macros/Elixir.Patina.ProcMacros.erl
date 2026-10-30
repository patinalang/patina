-file("tests/ui/src/pkg/proc_macros.pt", 1).

-module('Elixir.Patina.ProcMacros').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         answer/0,
         from_attr/0,
         hello_from_derive/0,
         type_name/0,
         bang/0,
         attribute/0,
         derive/0,
         documented/0,
         round_trip/0,
         token_zoo/0,
         forwarded/0,
         rebuilt/0,
         built_streams/0,
         rendered/0,
         constructors/0,
         escaped/0,
         call_site/0,
         raw_identifier/0,
         set_span/0]).

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

identity() -> 10.

identity_alias() -> 20.

nested_identity() -> 30.

answer() -> 42.

from_attr() -> 40 + 2.

hello_from_derive() -> 7.

type_name() -> <<"Point"/utf8>>.

bang() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [1 + 2]),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [4 + 1]),
                                        [_element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [5 + 2]),
                                        [_element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@9_0 = begin
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [identity()]),
                                        _element@7_0 = io_lib:format("~ktp",
                                                                     [identity_alias()]),
                                        _element@8_0 = io_lib:format("~ktp",
                                                                     [nested_identity()]),
                                        [_element@6_0,
                                         <<" "/utf8>>,
                                         _element@7_0,
                                         <<" "/utf8>>,
                                         _element@8_0]
                                    end,
                     [_element@9_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@11_0 = begin
                                         _element@10_0 = io_lib:format("~ktp",
                                                                       [42]),
                                         [_element@10_0]
                                     end,
                     [_element@11_0]
                 end),
    io:nl(),
    {}.

attribute() ->
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [answer()]),
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [from_attr()]),
                                        [_element@0_0,
                                         <<" "/utf8>>,
                                         _element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.

derive() ->
    _point@0_0 = {point, 1, 2},
    io:put_chars(begin
                     _element@4_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [erlang:element(2,
                                                                                     _point@0_0)
                                                                          +
                                                                          erlang:element(3,
                                                                                         _point@0_0)]),
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [hello_from_derive()]),
                                        _element@3_0 = type_name(),
                                        [_element@1_0,
                                         <<" "/utf8>>,
                                         _element@2_0,
                                         <<" "/utf8>>,
                                         _element@3_0]
                                    end,
                     [_element@4_0]
                 end),
    io:nl(),
    {}.

documented() -> 1.

round_trip() ->
    _a@0_0 = 1 + 2 * 3,
    _b@2_0 = begin _inner@1_0 = 10, _inner@1_0 * 2 end,
    _c@3_0 = 1 + 2,
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [_a@0_0]),
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [_b@2_0]),
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_c@3_0]),
                                        [_element@4_0,
                                         <<" "/utf8>>,
                                         _element@5_0,
                                         <<" "/utf8>>,
                                         _element@6_0]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    {}.

token_zoo() ->
    _shifted@1_0 = begin
                       _x@0_0 = 1,
                       _x@0_1 = _x@0_0 bsl 3,
                       _x@0_2 = _x@0_1 bsr 1,
                       _x@0_2
                   end,
    _nested@9_0 = begin
                      _deep@2_0 = {1, {2, {3, {4, 5}}}},
                      _lists@3_0 = [[1, 2], [3, 4]],
                      case _deep@2_0 of
                          {_a@4_0, {_b@5_0, {_c@6_0, {_d@7_0, _e@8_0}}}} ->
                              _a@4_0 + _b@5_0 + _c@6_0 + _d@7_0 + _e@8_0
                      end
                  end,
    _float@10_0 = 2.5e-1,
    _text@11_0 = <<"a \"quoted\" string"/utf8>>,
    _raw@12_0 = <<"no \\escapes"/utf8>>,
    _atom@13_0 = atom,
    _raw_atom@14_0 = 'raw \\atom',
    _bits@15_0 = 3 band 15,
    _ranged@17_0 = case 3 of
                       _range@16_0 when _range@16_0 >= 1, _range@16_0 =< 5 ->
                           true;
                       _ -> false
                   end,
    _neg@18_0 = -5,
    io:put_chars(begin
                     _element@29_0 = begin
                                         _element@19_0 = io_lib:format("~ktp",
                                                                       [_shifted@1_0]),
                                         _element@20_0 = io_lib:format("~ktp",
                                                                       [_nested@9_0]),
                                         _element@21_0 = io_lib:format("~ktp",
                                                                       [_float@10_0]),
                                         _element@22_0 = _text@11_0,
                                         _element@23_0 = _raw@12_0,
                                         _element@24_0 = io_lib:format("~ktp",
                                                                       [_atom@13_0]),
                                         _element@25_0 = io_lib:format("~ktp",
                                                                       [_raw_atom@14_0]),
                                         _element@26_0 = io_lib:format("~ktp",
                                                                       [_bits@15_0]),
                                         _element@27_0 = io_lib:format("~ktp",
                                                                       [_ranged@17_0]),
                                         _element@28_0 = io_lib:format("~ktp",
                                                                       [_neg@18_0]),
                                         [_element@19_0,
                                          <<" "/utf8>>,
                                          _element@20_0,
                                          <<" "/utf8>>,
                                          _element@21_0,
                                          <<" "/utf8>>,
                                          _element@22_0,
                                          <<" "/utf8>>,
                                          _element@23_0,
                                          <<" "/utf8>>,
                                          _element@24_0,
                                          <<" "/utf8>>,
                                          _element@25_0,
                                          <<" "/utf8>>,
                                          _element@26_0,
                                          <<" "/utf8>>,
                                          _element@27_0,
                                          <<" "/utf8>>,
                                          _element@28_0]
                                     end,
                     [_element@29_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@32_0 = begin
                                         _element@30_0 = io_lib:format("~ktp",
                                                                       [documented()]),
                                         _element@31_0 = io_lib:format("~ktp",
                                                                       ['Elixir.Patina.ProcMacros.Documented':value()]),
                                         [_element@30_0,
                                          <<" "/utf8>>,
                                          _element@31_0]
                                     end,
                     [_element@32_0]
                 end),
    io:nl(),
    {}.

forwarded() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [1 + 2]),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {}.

rebuilt() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [(1 + 2) *
                                                                          3]),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {}.

built_streams() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [1 + 2]),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {_list@3_0, _product@4_0} = begin
                                    _element@2_0 = 1 + 2 * 3,
                                    {[1, 2], _element@2_0}
                                end,
    case _list@3_0 of
        [_a@5_0, _b@6_0] ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@7_0 =
                                                     io_lib:format("~ktp",
                                                                   [_a@5_0]),
                                                 _element@8_0 =
                                                     io_lib:format("~ktp",
                                                                   [_b@6_0]),
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_product@4_0]),
                                                 [_element@7_0,
                                                  <<" "/utf8>>,
                                                  _element@8_0,
                                                  <<" "/utf8>>,
                                                  _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"unexpected"/utf8>>]]),
            io:nl(),
            {}
    end.

rendered() ->
    io:put_chars([[<<"`x` += f(1, \"q\\n\") r#\"raw\"# a\"at\" "
                     "'c' #[z] a.b if (y) m! (1) if ! w receive "
                     "(v) `if`(u) fn(t)"/utf8>>]]),
    io:nl(),
    {}.

constructors() ->
    {_c@0_0, _i@1_0, _big@2_0, _f@3_0, _s@4_0, _a@5_0} =
        {$a,
         1,
         123456789012345678901234567890,
         2.5e-1,
         <<"s"/utf8>>,
         a},
    io:put_chars(begin
                     _element@12_0 = begin
                                         _element@6_0 = io_lib:format("~ktp",
                                                                      [_c@0_0]),
                                         _element@7_0 = io_lib:format("~ktp",
                                                                      [_i@1_0]),
                                         _element@8_0 = io_lib:format("~ktp",
                                                                      [_big@2_0]),
                                         _element@9_0 = io_lib:format("~ktp",
                                                                      [_f@3_0]),
                                         _element@10_0 = _s@4_0,
                                         _element@11_0 = io_lib:format("~ktp",
                                                                       [_a@5_0]),
                                         [_element@6_0,
                                          <<" "/utf8>>,
                                          _element@7_0,
                                          <<" "/utf8>>,
                                          _element@8_0,
                                          <<" "/utf8>>,
                                          _element@9_0,
                                          <<" "/utf8>>,
                                          _element@10_0,
                                          <<" "/utf8>>,
                                          _element@11_0]
                                     end,
                     [_element@12_0]
                 end),
    io:nl(),
    {}.

escaped() ->
    {_s@0_0,
     _quote@1_0,
     _backslash@2_0,
     _double@3_0,
     _non_ascii@4_0,
     _a@5_0} =
        {<<"a\"b\\c\nd'e"/utf8>>, $', $\\, $", $ż, 'x "y"'},
    io:put_chars(begin
                     _element@12_0 = begin
                                         _element@6_0 = _s@0_0,
                                         _element@7_0 = io_lib:format("~ktp",
                                                                      [_quote@1_0]),
                                         _element@8_0 = io_lib:format("~ktp",
                                                                      [_backslash@2_0]),
                                         _element@9_0 = io_lib:format("~ktp",
                                                                      [_double@3_0]),
                                         _element@10_0 = io_lib:format("~ktp",
                                                                       [_non_ascii@4_0]),
                                         _element@11_0 = io_lib:format("~ktp",
                                                                       [_a@5_0]),
                                         [_element@6_0,
                                          <<" "/utf8>>,
                                          _element@7_0,
                                          <<" "/utf8>>,
                                          _element@8_0,
                                          <<" "/utf8>>,
                                          _element@9_0,
                                          <<" "/utf8>>,
                                          _element@10_0,
                                          <<" "/utf8>>,
                                          _element@11_0]
                                     end,
                     [_element@12_0]
                 end),
    io:nl(),
    {}.

call_site() ->
    _local@0_0 = 5,
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_local@0_0]),
                                        [_element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.

raw_identifier() ->
    _x@0_0 = 7,
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_x@0_0]),
                                        [_element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.

set_span() ->
    _local@0_0 = 5,
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_local@0_0]),
                                        [_element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@4_0 = begin
                                        _element@3_0 = io_lib:format("~ktp",
                                                                     [_local@0_0
                                                                          + 1
                                                                          -
                                                                          _local@0_0
                                                                              *
                                                                              2]),
                                        [_element@3_0]
                                    end,
                     [_element@4_0]
                 end),
    io:nl(),
    {}.
