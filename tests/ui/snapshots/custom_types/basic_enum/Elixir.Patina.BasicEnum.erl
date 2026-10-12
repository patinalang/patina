-file("tests/ui/src/custom_types/basic_enum.pt", 1).

-module('Elixir.Patina.BasicEnum').

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
    _x@0_0 = 'Elixir.Patina.BasicEnum.BasicEnum.Unit',
    _y@1_0 =
        'Elixir.Patina.BasicEnum.BasicEnum':'$patina_constructor__Tuple'(1,
                                                                         <<"hello"/utf8>>),
    _z@2_0 =
        'Elixir.Patina.BasicEnum.BasicEnum':'$patina_constructor__Struct'(1,
                                                                          <<"hello"/utf8>>),
    begin
        io:put_chars(begin
                         _element@8_0 = begin
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [_x@0_0]),
                                            _element@4_0 = <<"\n"/utf8>>,
                                            _element@5_0 = io_lib:format("~ktp",
                                                                         [_y@1_0]),
                                            _element@6_0 = <<"\n"/utf8>>,
                                            _element@7_0 = io_lib:format("~ktp",
                                                                         [_z@2_0]),
                                            [_element@3_0,
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
