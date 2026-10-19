-file("tests/ui/src/flow_control/if_let__02.pt", 4).

-module('Elixir.Patina.IfLet02').

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
    _number@0_0 = 7,
    _letter@1_0 = nil,
    _emoticon@2_0 = nil,
    case _number@0_0 of
        _value@4_0 = _i@3_0
            when erlang:is_integer(_value@4_0) ->
            begin
                io:put_chars(begin
                                 _element@8_0 = begin
                                                    _element@5_0 =
                                                        <<"Matched "/utf8>>,
                                                    _element@6_0 =
                                                        io_lib:format("~ktp",
                                                                      [_i@3_0]),
                                                    _element@7_0 = <<"!"/utf8>>,
                                                    [_element@5_0,
                                                     _element@6_0,
                                                     _element@7_0]
                                                end,
                                 [_element@8_0]
                             end),
                io:nl(),
                {}
            end;
        _ -> {}
    end,
    case _letter@1_0 of
        _value@10_0 = _i@9_0
            when erlang:is_integer(_value@10_0) ->
            begin
                io:put_chars(begin
                                 _element@14_0 = begin
                                                     _element@11_0 =
                                                         <<"Matched "/utf8>>,
                                                     _element@12_0 =
                                                         io_lib:format("~ktp",
                                                                       [_i@9_0]),
                                                     _element@13_0 =
                                                         <<"!"/utf8>>,
                                                     [_element@11_0,
                                                      _element@12_0,
                                                      _element@13_0]
                                                 end,
                                 [_element@14_0]
                             end),
                io:nl(),
                {}
            end;
        _ ->
            begin
                io:put_chars(begin
                                 _element@15_0 =
                                     [<<"Didn't match a number. Let's go with "
                                        "a letter!"/utf8>>],
                                 [_element@15_0]
                             end),
                io:nl(),
                {}
            end
    end,
    _i_like_letters@16_0 = false,
    case _emoticon@2_0 of
        _value@18_0 = _i@17_0
            when erlang:is_integer(_value@18_0) ->
            begin
                io:put_chars(begin
                                 _element@22_0 = begin
                                                     _element@19_0 =
                                                         <<"Matched "/utf8>>,
                                                     _element@20_0 =
                                                         io_lib:format("~ktp",
                                                                       [_i@17_0]),
                                                     _element@21_0 =
                                                         <<"!"/utf8>>,
                                                     [_element@19_0,
                                                      _element@20_0,
                                                      _element@21_0]
                                                 end,
                                 [_element@22_0]
                             end),
                io:nl(),
                {}
            end;
        _ ->
            case _i_like_letters@16_0 of
                true ->
                    begin
                        io:put_chars(begin
                                         _element@23_0 =
                                             [<<"Didn't match a number. Let's go with "
                                                "a letter!"/utf8>>],
                                         [_element@23_0]
                                     end),
                        io:nl(),
                        {}
                    end;
                _ ->
                    begin
                        io:put_chars(begin
                                         _element@24_0 =
                                             [<<"I don't like letters. Let's go with "
                                                "an emoticon :)!"/utf8>>],
                                         [_element@24_0]
                                     end),
                        io:nl(),
                        {}
                    end
            end
    end.
