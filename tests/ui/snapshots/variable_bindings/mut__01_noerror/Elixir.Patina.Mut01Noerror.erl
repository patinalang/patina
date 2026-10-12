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
    begin
        io:put_chars(begin
                         _element@4_0 = begin
                                            _element@2_0 =
                                                <<"Before mutating the binding: "/utf8>>,
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [_mutable_binding@1_0]),
                                            [_element@2_0, _element@3_0]
                                        end,
                         [_element@4_0]
                     end),
        io:nl(),
        {}
    end,
    _mutable_binding@1_1 = _mutable_binding@1_0 + 1,
    begin
        io:put_chars(begin
                         _element@7_0 = begin
                                            _element@5_0 =
                                                <<"After mutating the binding: "/utf8>>,
                                            _element@6_0 = io_lib:format("~ktp",
                                                                         [_mutable_binding@1_1]),
                                            [_element@5_0, _element@6_0]
                                        end,
                         [_element@7_0]
                     end),
        io:nl(),
        {}
    end,
    _my_fun@14_0 = fun () ->
                           _inner_mutable_binding@8_0 = _mutable_binding@1_1,
                           _inner_mutable_binding@8_1 =
                               _inner_mutable_binding@8_0 + 1,
                           begin
                               io:put_chars(begin
                                                _element@13_0 = begin
                                                                    _element@9_0 =
                                                                        io_lib:format("~ktp",
                                                                                      [_inner_mutable_binding@8_1]),
                                                                    _element@10_0 =
                                                                        <<" == "/utf8>>,
                                                                    _element@11_0 =
                                                                        io_lib:format("~ktp",
                                                                                      [_mutable_binding@1_1]),
                                                                    _element@12_0 =
                                                                        <<" + 1"/utf8>>,
                                                                    [_element@9_0,
                                                                     _element@10_0,
                                                                     _element@11_0,
                                                                     _element@12_0]
                                                                end,
                                                [_element@13_0]
                                            end),
                               io:nl(),
                               {}
                           end
                   end,
    {}.
