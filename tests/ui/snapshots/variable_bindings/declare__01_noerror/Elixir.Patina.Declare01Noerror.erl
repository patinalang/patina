-file("tests/ui/src/variable_bindings/declare__01_no"
      "error.pt",
      1).

-module('Elixir.Patina.Declare01Noerror').

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
    _a_binding@0_0 = {},
    _x@1_0 = 2,
    _a_binding@0_1 = _x@1_0 * _x@1_0,
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_a_binding@0_1]),
                                        [<<"a binding: "/utf8>>, _element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    _another_binding@4_0 = {},
    _another_binding@4_1 = 1,
    io:put_chars(begin
                     _element@6_0 = begin
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [_another_binding@4_1]),
                                        [<<"another binding: "/utf8>>,
                                         _element@5_0]
                                    end,
                     [_element@6_0]
                 end),
    io:nl(),
    {}.
