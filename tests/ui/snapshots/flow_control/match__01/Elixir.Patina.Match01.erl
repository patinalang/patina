-file("tests/ui/src/flow_control/match__01.pt", 4).

-module('Elixir.Patina.Match01').

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
    _number@0_0 = 13,
    begin
        io:put_chars(begin
                         _element@3_0 = begin
                                            _element@1_0 =
                                                <<"Tell me about "/utf8>>,
                                            _element@2_0 = io_lib:format("~ktp",
                                                                         [_number@0_0]),
                                            [_element@1_0, _element@2_0]
                                        end,
                         [_element@3_0]
                     end),
        io:nl(),
        {}
    end,
    case _number@0_0 of
        1 ->
            io:put_chars(begin
                             _element@4_0 = [<<"One!"/utf8>>], [_element@4_0]
                         end),
            io:nl(),
            {};
        2 ->
            io:put_chars(begin
                             _element@5_0 = [<<"This is a prime"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        3 ->
            io:put_chars(begin
                             _element@5_0 = [<<"This is a prime"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        5 ->
            io:put_chars(begin
                             _element@5_0 = [<<"This is a prime"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        7 ->
            io:put_chars(begin
                             _element@5_0 = [<<"This is a prime"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        11 ->
            io:put_chars(begin
                             _element@5_0 = [<<"This is a prime"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        _range@6_0 when _range@6_0 >= 13, _range@6_0 =< 19 ->
            io:put_chars(begin
                             _element@7_0 = [<<"A teen"/utf8>>], [_element@7_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@8_0 = [<<"Ain't special"/utf8>>],
                             [_element@8_0]
                         end),
            io:nl(),
            {}
    end,
    _boolean@9_0 = true,
    _binary@10_0 = case _boolean@9_0 of
                       false -> 0;
                       true -> 1
                   end,
    begin
        io:put_chars(begin
                         _element@14_0 = begin
                                             _element@11_0 =
                                                 io_lib:format("~ktp",
                                                               [_boolean@9_0]),
                                             _element@12_0 = <<" -> "/utf8>>,
                                             _element@13_0 =
                                                 io_lib:format("~ktp",
                                                               [_binary@10_0]),
                                             [_element@11_0,
                                              _element@12_0,
                                              _element@13_0]
                                         end,
                         [_element@14_0]
                     end),
        io:nl(),
        {}
    end,
    _expected_value@15_0 = 42,
    case 123 of
        _expected_value@15_0 ->
            io:put_chars(begin
                             _element@16_0 = [<<"found expected value"/utf8>>],
                             [_element@16_0]
                         end),
            io:nl(),
            {};
        _value@17_0 ->
            io:put_chars(begin
                             _element@20_0 = begin
                                                 _element@18_0 =
                                                     <<"found unexpected value "/utf8>>,
                                                 _element@19_0 =
                                                     io_lib:format("~ktp",
                                                                   [_value@17_0]),
                                                 [_element@18_0, _element@19_0]
                                             end,
                             [_element@20_0]
                         end),
            io:nl(),
            {}
    end.
