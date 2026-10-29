-file("tests/ui/src/custom_types/repr__03.pt", 4).

-module('Elixir.Patina.Repr03').

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
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.Repr03.Unit']),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [custom_name]),
                                        [_element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [variant_a]),
                                        [_element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [variantB]),
                                        [_element@6_0]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    {}.
