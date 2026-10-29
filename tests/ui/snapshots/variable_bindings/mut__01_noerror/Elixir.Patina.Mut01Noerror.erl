-file("tests/ui/src/variable_bindings/mut__01_noerro"
      "r.pt",
      1).

-module('Elixir.Patina.Mut01Noerror').

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
    __immutable_binding@0_0 = 1,
    _mutable_binding@1_0 = 1,
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_mutable_binding@1_0]),
                                        [<<"Before mutating the binding: "/utf8>>,
                                         _element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    _mutable_binding@1_1 = _mutable_binding@1_0 + 1,
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [_mutable_binding@1_1]),
                                        [<<"After mutating the binding: "/utf8>>,
                                         _element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    _my_fun@10_0 = fun () ->
                           _inner_mutable_binding@6_0 = _mutable_binding@1_1,
                           _inner_mutable_binding@6_1 =
                               _inner_mutable_binding@6_0 + 1,
                           io:put_chars(begin
                                            _element@9_0 = begin
                                                               _element@7_0 =
                                                                   io_lib:format("~ktp",
                                                                                 [_inner_mutable_binding@6_1]),
                                                               _element@8_0 =
                                                                   io_lib:format("~ktp",
                                                                                 [_mutable_binding@1_1]),
                                                               [_element@7_0,
                                                                <<" == "/utf8>>,
                                                                _element@8_0,
                                                                <<" + 1"/utf8>>]
                                                           end,
                                            [_element@9_0]
                                        end),
                           io:nl(),
                           {}
                   end,
    {}.
