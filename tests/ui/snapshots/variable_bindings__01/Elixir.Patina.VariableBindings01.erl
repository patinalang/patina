-file("tests/ui/src/variable_bindings__01.pt", 4).

-module('Elixir.Patina.VariableBindings01').

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
    _an_integer@0_0 = 1,
    _a_boolean@1_0 = true,
    _unit@2_0 = {},
    _copied_integer@3_0 = _an_integer@0_0,
    begin
        io:put_chars(begin
                         _element@6_0 = begin
                                            _element@4_0 =
                                                <<"An integer: "/utf8>>,
                                            _element@5_0 = io_lib:format("~ktp",
                                                                         [_copied_integer@3_0]),
                                            [_element@4_0, _element@5_0]
                                        end,
                         [_element@6_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@9_0 = begin
                                            _element@7_0 =
                                                <<"A boolean: "/utf8>>,
                                            _element@8_0 = io_lib:format("~ktp",
                                                                         [_a_boolean@1_0]),
                                            [_element@7_0, _element@8_0]
                                        end,
                         [_element@9_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@12_0 = begin
                                             _element@10_0 =
                                                 <<"Meet the unit value: "/utf8>>,
                                             _element@11_0 =
                                                 io_lib:format("~ktp",
                                                               [_unit@2_0]),
                                             [_element@10_0, _element@11_0]
                                         end,
                         [_element@12_0]
                     end),
        io:nl(),
        {}
    end,
    __unused_variable@13_0 = 3,
    _noisy_unused_variable@14_0 = 2,
    {}.
