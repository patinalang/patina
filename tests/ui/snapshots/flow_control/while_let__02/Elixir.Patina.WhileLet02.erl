-file("tests/ui/src/flow_control/while_let__02.pt", 4).

-module('Elixir.Patina.WhileLet02').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'main$patina_loop_1'/1, main/0]).

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

'main$patina_loop_1'(_optional@0_0) ->
    _ = begin
            case _optional@0_0 of
                _value@2_0 = _i@1_0
                    when erlang:is_integer(_value@2_0) ->
                    case _i@1_0 > 9 of
                        true ->
                            begin
                                io:put_chars(begin
                                                 _element@3_0 =
                                                     [<<"Greater than 9, quit!"/utf8>>],
                                                 [_element@3_0]
                                             end),
                                io:nl(),
                                {}
                            end,
                            _optional@0_1 = nil,
                            {};
                        _ ->
                            begin
                                io:put_chars(begin
                                                 _element@7_0 = begin
                                                                    _element@4_0 =
                                                                        <<"`i` is `"/utf8>>,
                                                                    _element@5_0 =
                                                                        io_lib:format("~ktp",
                                                                                      [_i@1_0]),
                                                                    _element@6_0 =
                                                                        <<"`. Try again."/utf8>>,
                                                                    [_element@4_0,
                                                                     _element@5_0,
                                                                     _element@6_0]
                                                                end,
                                                 [_element@7_0]
                                             end),
                                io:nl(),
                                {}
                            end,
                            _optional@0_1 = begin
                                                _0@8_0 = _i@1_0 + 1, _0@8_0
                                            end,
                            {}
                    end;
                _ ->
                    _branch_value@9_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _optional@0_0})
                                        end,
                    _optional@0_1 = _optional@0_0,
                    _branch_value@9_0
            end
        end,
    '$patina_marker':continue(_optional@0_1).

main() ->
    _optional@0_0 = 0,
    case 'main$patina_loop_1'(_optional@0_0) of
        {break, 0, _loop_value@10_0, _optional@0_1} ->
            _loop_value@10_0
    end.
