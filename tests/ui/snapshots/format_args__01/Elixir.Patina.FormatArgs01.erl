-file("tests/ui/src/format_args__01.pt", 4).

-module('Elixir.Patina.FormatArgs01').

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
    _name@0_0 = <<"world"/utf8>>,
    _n@1_0 = 42,
    begin
        io:put_chars(begin
                         _element@5_0 = begin
                                            _element@2_0 = <<"hello, "/utf8>>,
                                            _element@3_0 = _name@0_0,
                                            _element@4_0 = <<"!"/utf8>>,
                                            [_element@2_0,
                                             _element@3_0,
                                             _element@4_0]
                                        end,
                         [_element@5_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@9_0 = begin
                                            _element@6_0 = _name@0_0,
                                            _element@7_0 = <<" = "/utf8>>,
                                            _element@8_0 = io_lib:format("~ktp",
                                                                         [_n@1_0]),
                                            [_element@6_0,
                                             _element@7_0,
                                             _element@8_0]
                                        end,
                         [_element@9_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@13_0 = begin
                                             _element@10_0 = _name@0_0,
                                             _element@11_0 = <<" and "/utf8>>,
                                             _element@12_0 =
                                                 io_lib:format("~ktp",
                                                               [_n@1_0]),
                                             [_element@10_0,
                                              _element@11_0,
                                              _element@12_0]
                                         end,
                         [_element@13_0]
                     end),
        io:nl(),
        {}
    end,
    begin io:put_chars([<<"no "/utf8>>]), {} end,
    begin io:put_chars([<<"newline"/utf8>>]), {} end,
    begin io:nl(), {} end.
