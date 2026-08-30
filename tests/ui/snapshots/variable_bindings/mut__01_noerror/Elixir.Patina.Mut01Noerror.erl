-file("tests/ui/src/variable_bindings/mut__01_noerro"
      "r.pt",
      1).

-module('Elixir.Patina.Mut01Noerror').

-compile([no_auto_import]).

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
        io:put_chars([[<<"Before mutating the binding: ">>,
                       io_lib:format("~ktp", [_mutable_binding@1_0])]]),
        io:nl()
    end,
    _mutable_binding@1_1 = _mutable_binding@1_0 + 1,
    begin
        io:put_chars([[<<"After mutating the binding: ">>,
                       io_lib:format("~ktp", [_mutable_binding@1_1])]]),
        io:nl()
    end,
    __immutable_binding@0_1 = __immutable_binding@0_0 + 1,
    _my_fun@3_0 = fun () ->
                          _inner_mutable_binding@2_0 = _mutable_binding@1_1,
                          _inner_mutable_binding@2_1 =
                              _inner_mutable_binding@2_0 + 1,
                          begin
                              io:put_chars([[io_lib:format("~ktp",
                                                           [_inner_mutable_binding@2_1]),
                                             <<" == ">>,
                                             io_lib:format("~ktp",
                                                           [_mutable_binding@1_1]),
                                             <<" + 1">>]]),
                              io:nl()
                          end
                  end.
