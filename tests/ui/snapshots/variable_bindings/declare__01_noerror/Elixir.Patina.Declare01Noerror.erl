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
    begin _a_binding@0_0 = {} end,
    begin
        _x@1_0 = 2, _a_binding@0_1 = _x@1_0 * _x@1_0, {}
    end,
    begin
        io:put_chars([[<<"a binding: ">>,
                       io_lib:format("~ktp", [_a_binding@0_1])]]),
        io:nl(),
        {}
    end,
    begin _another_binding@2_0 = {} end,
    _another_binding@2_1 = 1,
    begin
        io:put_chars([[<<"another binding: ">>,
                       io_lib:format("~ktp", [_another_binding@2_1])]]),
        io:nl(),
        {}
    end.
