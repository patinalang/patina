-file("tests/ui/src/flow_control/if_else__01.pt", 4).

-module('Elixir.Patina.IfElse01').

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
    _n@0_0 = 5,
    case _n@0_0 < 0 of
        true ->
            io:put_chars(begin
                             _element@1_0 = io_lib:format("~ktp", [_n@0_0]),
                             [_element@1_0, <<" is negative"/utf8>>]
                         end),
            {};
        _ ->
            case _n@0_0 > 0 of
                true ->
                    io:put_chars(begin
                                     _element@2_0 = io_lib:format("~ktp",
                                                                  [_n@0_0]),
                                     [_element@2_0, <<" is positive"/utf8>>]
                                 end),
                    {};
                _ ->
                    io:put_chars(begin
                                     _element@3_0 = io_lib:format("~ktp",
                                                                  [_n@0_0]),
                                     [_element@3_0, <<" is zero"/utf8>>]
                                 end),
                    {}
            end
    end,
    _big_n@4_0 = case _n@0_0 < 10 andalso _n@0_0 > -10 of
                     true ->
                         io:put_chars([[<<", and is a small number, increase ten-fold"/utf8>>]]),
                         io:nl(),
                         10 * _n@0_0;
                     _ ->
                         io:put_chars([[<<", and is a big number, halve the number"/utf8>>]]),
                         io:nl(),
                         _n@0_0 div 2
                 end,
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [_n@0_0]),
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_big_n@4_0]),
                                        [_element@5_0,
                                         <<" -> "/utf8>>,
                                         _element@6_0]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    {}.
