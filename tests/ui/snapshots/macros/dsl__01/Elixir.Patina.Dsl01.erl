-file("tests/ui/src/macros/dsl__01.pt", 4).

-module('Elixir.Patina.Dsl01').

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
    begin
        _val@0_0 = 1 + 2,
        begin
            io:put_chars(begin
                             _element@4_0 = begin
                                                _element@1_0 = <<"1 + 2"/utf8>>,
                                                _element@2_0 = <<" = "/utf8>>,
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_val@0_0]),
                                                [_element@1_0,
                                                 _element@2_0,
                                                 _element@3_0]
                                            end,
                             [_element@4_0]
                         end),
            io:nl(),
            {}
        end
    end,
    begin
        _val@5_0 = (1 + 2) * (3 div 4),
        begin
            io:put_chars(begin
                             _element@9_0 = begin
                                                _element@6_0 =
                                                    <<"(1 + 2) * (3 / 4)"/utf8>>,
                                                _element@7_0 = <<" = "/utf8>>,
                                                _element@8_0 =
                                                    io_lib:format("~ktp",
                                                                  [_val@5_0]),
                                                [_element@6_0,
                                                 _element@7_0,
                                                 _element@8_0]
                                            end,
                             [_element@9_0]
                         end),
            io:nl(),
            {}
        end
    end.
