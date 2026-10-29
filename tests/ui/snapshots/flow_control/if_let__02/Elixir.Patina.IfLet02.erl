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
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  [_i@3_0]),
                                                [<<"Matched "/utf8>>,
                                                 _element@5_0,
                                                 <<"!"/utf8>>]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            {};
        _ -> {}
    end,
    case _letter@1_0 of
        _value@8_0 = _i@7_0
            when erlang:is_integer(_value@8_0) ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_i@7_0]),
                                                 [<<"Matched "/utf8>>,
                                                  _element@9_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Didn't match a number. Let's go with "
                             "a letter!"/utf8>>]]),
            io:nl(),
            {}
    end,
    _i_like_letters@11_0 = false,
    case _emoticon@2_0 of
        _value@13_0 = _i@12_0
            when erlang:is_integer(_value@13_0) ->
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@14_0 =
                                                     io_lib:format("~ktp",
                                                                   [_i@12_0]),
                                                 [<<"Matched "/utf8>>,
                                                  _element@14_0,
                                                  <<"!"/utf8>>]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            {};
        _ ->
            case _i_like_letters@11_0 of
                true ->
                    io:put_chars([[<<"Didn't match a number. Let's go with "
                                     "a letter!"/utf8>>]]),
                    io:nl(),
                    {};
                _ ->
                    io:put_chars([[<<"I don't like letters. Let's go with "
                                     "an emoticon :)!"/utf8>>]]),
                    io:nl(),
                    {}
            end
    end.
