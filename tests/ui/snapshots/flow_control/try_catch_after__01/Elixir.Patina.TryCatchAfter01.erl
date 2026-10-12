-file("tests/ui/src/flow_control/try_catch_after__01.pt",
      4).

-module('Elixir.Patina.TryCatchAfter01').

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

open_resource() -> ok.

main() ->
    try open_resource(), {} catch
        error:badarg ->
            io:put_chars(begin
                             _element@1_0 = [<<"badarg!"/utf8>>], [_element@1_0]
                         end),
            io:nl(),
            {};
        error:(#{'__struct__' := 'Elixir.ArithmeticError'}) ->
            io:put_chars(begin
                             _element@2_0 = [<<"Elixir ArithmeticError"/utf8>>],
                             [_element@2_0]
                         end),
            io:nl(),
            {};
        error:(_reason@3_0 = #{'__struct__' :=
                                   'Elixir.ArgumentError'}) ->
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@4_0 =
                                                    <<"Elixir ArgumentError "/utf8>>,
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  [_reason@3_0]),
                                                [_element@4_0, _element@5_0]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            {};
        error:_reason@8_0:_stacktrace@7_0 ->
            io:put_chars(begin
                             _element@13_0 = begin
                                                 _element@9_0 =
                                                     <<"other error: "/utf8>>,
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_reason@8_0]),
                                                 _element@11_0 =
                                                     <<" from "/utf8>>,
                                                 _element@12_0 =
                                                     io_lib:format("~ktp",
                                                                   [_stacktrace@7_0]),
                                                 [_element@9_0,
                                                  _element@10_0,
                                                  _element@11_0,
                                                  _element@12_0]
                                             end,
                             [_element@13_0]
                         end),
            io:nl(),
            {};
        _value@14_0 ->
            io:put_chars(begin
                             _element@17_0 = begin
                                                 _element@15_0 =
                                                     <<"thrown: "/utf8>>,
                                                 _element@16_0 =
                                                     io_lib:format("~ktp",
                                                                   [_value@14_0]),
                                                 [_element@15_0, _element@16_0]
                                             end,
                             [_element@17_0]
                         end),
            io:nl(),
            {};
        exit:_reason@18_0 ->
            io:put_chars(begin
                             _element@21_0 = begin
                                                 _element@19_0 =
                                                     <<"exit: "/utf8>>,
                                                 _element@20_0 =
                                                     io_lib:format("~ktp",
                                                                   [_reason@18_0]),
                                                 [_element@19_0, _element@20_0]
                                             end,
                             [_element@21_0]
                         end),
            io:nl(),
            {}
    after
        begin
            io:put_chars(begin
                             _element@0_0 = [<<"cleaning up"/utf8>>],
                             [_element@0_0]
                         end),
            io:nl(),
            {}
        end
    end.
