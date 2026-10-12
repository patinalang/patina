-file("tests/ui/src/flow_control/match/guard__01.pt",
      4).

-module('Elixir.Patina.Guard01').

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
    _temperature@0_0 = 35,
    case _temperature@0_0 of
        _value@2_0 = _t@1_0
            when erlang:is_integer(_value@2_0), _t@1_0 > 30 ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_t@1_0]),
                                                _element@4_0 =
                                                    <<"C is above 30 Celsius"/utf8>>,
                                                [_element@3_0, _element@4_0]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        _value@7_0 = _t@6_0
            when erlang:is_integer(_value@7_0) ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@8_0 =
                                                     io_lib:format("~ktp",
                                                                   [_t@6_0]),
                                                 _element@9_0 =
                                                     <<"C is equal to or below 30 Celsius"/utf8>>,
                                                 [_element@8_0, _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' =
                                                              _t@11_0}
            when _t@11_0 > 86 ->
            io:put_chars(begin
                             _element@14_0 = begin
                                                 _element@12_0 =
                                                     io_lib:format("~ktp",
                                                                   [_t@11_0]),
                                                 _element@13_0 =
                                                     <<"F is above 86 Fahrenheit"/utf8>>,
                                                 [_element@12_0, _element@13_0]
                                             end,
                             [_element@14_0]
                         end),
            io:nl(),
            {};
        #'Elixir.Patina.Guard01.Temperature':'Fahrenheit'{'0' =
                                                              _t@15_0} ->
            io:put_chars(begin
                             _element@18_0 = begin
                                                 _element@16_0 =
                                                     io_lib:format("~ktp",
                                                                   [_t@15_0]),
                                                 _element@17_0 =
                                                     <<"F is equal to or below 86 Fahrenheit"/utf8>>,
                                                 [_element@16_0, _element@17_0]
                                             end,
                             [_element@18_0]
                         end),
            io:nl(),
            {}
    end.
