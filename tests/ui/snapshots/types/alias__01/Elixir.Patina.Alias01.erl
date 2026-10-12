-file("tests/ui/src/types/alias__01.pt", 5).

-module('Elixir.Patina.Alias01').

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
    _nanoseconds@0_0 = 5,
    _inches@1_0 = 2,
    begin
        io:put_chars(begin
                         _element@8_0 = begin
                                            _element@2_0 = io_lib:format("~ktp",
                                                                         [_nanoseconds@0_0]),
                                            _element@3_0 =
                                                <<" nanoseconds + "/utf8>>,
                                            _element@4_0 = io_lib:format("~ktp",
                                                                         [_inches@1_0]),
                                            _element@5_0 =
                                                <<" inches = "/utf8>>,
                                            _element@6_0 = io_lib:format("~ktp",
                                                                         [_nanoseconds@0_0
                                                                              +
                                                                              _inches@1_0]),
                                            _element@7_0 = <<" unit?"/utf8>>,
                                            [_element@2_0,
                                             _element@3_0,
                                             _element@4_0,
                                             _element@5_0,
                                             _element@6_0,
                                             _element@7_0]
                                        end,
                         [_element@8_0]
                     end),
        io:nl(),
        {}
    end.
