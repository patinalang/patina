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
            begin
                io:put_chars(begin
                                 _element@1_0 = io_lib:format("~ktp", [_n@0_0]),
                                 _element@2_0 = <<" is negative"/utf8>>,
                                 [_element@1_0, _element@2_0]
                             end),
                {}
            end;
        _ ->
            case _n@0_0 > 0 of
                true ->
                    begin
                        io:put_chars(begin
                                         _element@3_0 = io_lib:format("~ktp",
                                                                      [_n@0_0]),
                                         _element@4_0 = <<" is positive"/utf8>>,
                                         [_element@3_0, _element@4_0]
                                     end),
                        {}
                    end;
                _ ->
                    begin
                        io:put_chars(begin
                                         _element@5_0 = io_lib:format("~ktp",
                                                                      [_n@0_0]),
                                         _element@6_0 = <<" is zero"/utf8>>,
                                         [_element@5_0, _element@6_0]
                                     end),
                        {}
                    end
            end
    end,
    _big_n@9_0 = case _n@0_0 < 10 andalso _n@0_0 > -10 of
                     true ->
                         begin
                             io:put_chars(begin
                                              _element@7_0 =
                                                  [<<", and is a small number, increase ten-fold"/utf8>>],
                                              [_element@7_0]
                                          end),
                             io:nl(),
                             {}
                         end,
                         10 * _n@0_0;
                     _ ->
                         begin
                             io:put_chars(begin
                                              _element@8_0 =
                                                  [<<", and is a big number, halve the number"/utf8>>],
                                              [_element@8_0]
                                          end),
                             io:nl(),
                             {}
                         end,
                         _n@0_0 div 2
                 end,
    begin
        io:put_chars(begin
                         _element@13_0 = begin
                                             _element@10_0 =
                                                 io_lib:format("~ktp",
                                                               [_n@0_0]),
                                             _element@11_0 = <<" -> "/utf8>>,
                                             _element@12_0 =
                                                 io_lib:format("~ktp",
                                                               [_big_n@9_0]),
                                             [_element@10_0,
                                              _element@11_0,
                                              _element@12_0]
                                         end,
                         [_element@13_0]
                     end),
        io:nl(),
        {}
    end.
