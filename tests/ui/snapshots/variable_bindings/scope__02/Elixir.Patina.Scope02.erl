-file("tests/ui/src/variable_bindings/scope__02.pt", 4).

-module('Elixir.Patina.Scope02').

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
    _shadowed_binding@0_0 = 1,
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_shadowed_binding@0_0]),
                                        [<<"before being shadowed: "/utf8>>,
                                         _element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    _shadowed_binding@3_0 = <<"abc"/utf8>>,
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@4_0 = _shadowed_binding@3_0,
                                        [<<"shadowed in inner block: "/utf8>>,
                                         _element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_shadowed_binding@0_0]),
                                        [<<"outside inner block: "/utf8>>,
                                         _element@6_0]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    _shadowed_binding@8_0 = 2,
    io:put_chars(begin
                     _element@10_0 = begin
                                         _element@9_0 = io_lib:format("~ktp",
                                                                      [_shadowed_binding@8_0]),
                                         [<<"shadowed in outer block: "/utf8>>,
                                          _element@9_0]
                                     end,
                     [_element@10_0]
                 end),
    io:nl(),
    {}.
