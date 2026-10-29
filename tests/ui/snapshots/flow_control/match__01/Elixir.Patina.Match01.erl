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
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_number@0_0]),
                                        [<<"Tell me about "/utf8>>,
                                         _element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    case [] of
        {'$patina_label_0'} ->
            io:put_chars([[<<"This is a prime"/utf8>>]]),
            io:nl(),
            {};
        _ ->
            case _number@0_0 of
                1 ->
                    io:put_chars([[<<"One!"/utf8>>]]),
                    io:nl(),
                    {};
                2 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                3 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                5 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                7 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                11 ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _range@3_0 when _range@3_0 >= 13, _range@3_0 =< 19 ->
                    io:put_chars([[<<"A teen"/utf8>>]]),
                    io:nl(),
                    {};
                _ ->
                    io:put_chars([[<<"Ain't special"/utf8>>]]),
                    io:nl(),
                    {}
            end
    end,
    _boolean@4_0 = true,
    _binary@5_0 = case _boolean@4_0 of
                      false -> 0;
                      true -> 1
                  end,
    io:put_chars(begin
                     _element@8_0 = begin
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_boolean@4_0]),
                                        _element@7_0 = io_lib:format("~ktp",
                                                                     [_binary@5_0]),
                                        [_element@6_0,
                                         <<" -> "/utf8>>,
                                         _element@7_0]
                                    end,
                     [_element@8_0]
                 end),
    io:nl(),
    _expected_value@9_0 = 42,
    case 123 of
        _expected_value@9_0 ->
            io:put_chars([[<<"found expected value"/utf8>>]]),
            io:nl(),
            {};
        _value@10_0 ->
            io:put_chars(begin
                             _element@12_0 = begin
                                                 _element@11_0 =
                                                     io_lib:format("~ktp",
                                                                   [_value@10_0]),
                                                 [<<"found unexpected value "/utf8>>,
                                                  _element@11_0]
                                             end,
                             [_element@12_0]
                         end),
            io:nl(),
            {}
    end.
