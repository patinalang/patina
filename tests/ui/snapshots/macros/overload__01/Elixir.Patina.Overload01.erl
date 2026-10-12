-file("tests/ui/src/macros/overload__01.pt", 6).

-module('Elixir.Patina.Overload01').

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
        io:put_chars(begin
                         _element@5_0 = begin
                                            _element@0_0 = io_lib:format("~ktp",
                                                                         [<<"1 + 1 == 2"/utf8>>]),
                                            _element@1_0 = <<" and "/utf8>>,
                                            _element@2_0 = io_lib:format("~ktp",
                                                                         [<<"2 * 2 == 4"/utf8>>]),
                                            _element@3_0 = <<" is "/utf8>>,
                                            _element@4_0 = io_lib:format("~ktp",
                                                                         [1 + 1
                                                                              =:=
                                                                              2
                                                                              andalso
                                                                              2
                                                                                  *
                                                                                  2
                                                                                  =:=
                                                                                  4]),
                                            [_element@0_0,
                                             _element@1_0,
                                             _element@2_0,
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
                         _element@11_0 = begin
                                             _element@6_0 =
                                                 io_lib:format("~ktp",
                                                               [<<"true"/utf8>>]),
                                             _element@7_0 = <<" or "/utf8>>,
                                             _element@8_0 =
                                                 io_lib:format("~ktp",
                                                               [<<"false"/utf8>>]),
                                             _element@9_0 = <<" is "/utf8>>,
                                             _element@10_0 =
                                                 io_lib:format("~ktp",
                                                               [true orelse
                                                                    false]),
                                             [_element@6_0,
                                              _element@7_0,
                                              _element@8_0,
                                              _element@9_0,
                                              _element@10_0]
                                         end,
                         [_element@11_0]
                     end),
        io:nl(),
        {}
    end.
