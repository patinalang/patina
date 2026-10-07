-file("tests/ui/src/flow_control/for__02.pt", 4).

-module('Elixir.Patina.For02').

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

'main$patina_loop_1'(_iter@0_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next'({'impl Core::Iterator for Core::RangeInclusive<int>',
                                                                            'Elixir.Patina.Core.RangeInclusive'},
                                                                           _iter@0_0)
                of
                nil ->
                    _branch_value@4_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@0_0}),
                    _iter@0_1 = _iter@0_0,
                    _branch_value@4_0;
                _value@3_0 = {_n@1_0, _next@2_0}
                    when erlang:is_tuple(_value@3_0) andalso
                             erlang:tuple_size(_value@3_0) =:= 2 ->
                    _iter@0_1 = _next@2_0,
                    begin
                        case _n@1_0 rem 15 =:= 0 of
                            true ->
                                begin
                                    io:put_chars([[<<"fizzbuzz"/utf8>>]]),
                                    io:nl(),
                                    {}
                                end;
                            false ->
                                case _n@1_0 rem 3 =:= 0 of
                                    true ->
                                        begin
                                            io:put_chars([[<<"fizz"/utf8>>]]),
                                            io:nl(),
                                            {}
                                        end;
                                    false ->
                                        case _n@1_0 rem 5 =:= 0 of
                                            true ->
                                                begin
                                                    io:put_chars([[<<"buzz"/utf8>>]]),
                                                    io:nl(),
                                                    {}
                                                end;
                                            false ->
                                                begin
                                                    io:put_chars([[io_lib:format("~ktp",
                                                                                 [_n@1_0])]]),
                                                    io:nl(),
                                                    {}
                                                end
                                        end
                                end
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@0_1).

main() ->
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::RangeInclusive<int>',
                                                                     'Elixir.Patina.Core.RangeInclusive'}},
                                                                   'Elixir.Patina.Core.RangeInclusive':new(1,
                                                                                                           100))
        of
        _iter@0_0 ->
            case 'main$patina_loop_1'(_iter@0_0) of
                {break, 0, _loop_value@5_0, _iter@0_1} ->
                    _loop_value@5_0
            end
    end.
