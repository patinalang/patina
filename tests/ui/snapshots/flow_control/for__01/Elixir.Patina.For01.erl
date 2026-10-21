-file("tests/ui/src/flow_control/for__01.pt", 4).

-module('Elixir.Patina.For01').

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

'main$patina_loop_1'(_witness_Iterator@0_0,
                     _iter@3_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.Range':'Core::Iterator::next<int>'(_witness_Iterator@0_0,
                                                                       _iter@3_0)
                of
                nil ->
                    _branch_value@12_0 = begin
                                             '$patina_marker':break({break,
                                                                     0,
                                                                     {},
                                                                     _iter@3_0}),
                                             []
                                         end,
                    _iter@3_1 = _iter@3_0,
                    _branch_value@12_0;
                _value@6_0 = {_n@4_0, _next@5_0}
                    when erlang:is_tuple(_value@6_0) andalso
                             erlang:tuple_size(_value@6_0) =:= 2 ->
                    _iter@3_1 = _next@5_0,
                    begin
                        case _n@4_0 rem 15 =:= 0 of
                            true ->
                                begin
                                    io:put_chars(begin
                                                     _element@7_0 =
                                                         [<<"fizzbuzz"/utf8>>],
                                                     [_element@7_0]
                                                 end),
                                    io:nl(),
                                    {}
                                end;
                            _ ->
                                case _n@4_0 rem 3 =:= 0 of
                                    true ->
                                        begin
                                            io:put_chars(begin
                                                             _element@8_0 =
                                                                 [<<"fizz"/utf8>>],
                                                             [_element@8_0]
                                                         end),
                                            io:nl(),
                                            {}
                                        end;
                                    _ ->
                                        case _n@4_0 rem 5 =:= 0 of
                                            true ->
                                                begin
                                                    io:put_chars(begin
                                                                     _element@9_0 =
                                                                         [<<"buzz"/utf8>>],
                                                                     [_element@9_0]
                                                                 end),
                                                    io:nl(),
                                                    {}
                                                end;
                                            _ ->
                                                begin
                                                    io:put_chars(begin
                                                                     _element@11_0 =
                                                                         begin
                                                                             _element@10_0 =
                                                                                 io_lib:format("~ktp",
                                                                                               [_n@4_0]),
                                                                             [_element@10_0]
                                                                         end,
                                                                     [_element@11_0]
                                                                 end),
                                                    io:nl(),
                                                    {}
                                                end
                                        end
                                end
                        end
                    end
            end
        end,
    begin
        '$patina_marker':continue(_witness_Iterator@0_0,
                                  _iter@3_1),
        []
    end.

main() ->
    _witness_Iterator@0_0 =
        'Elixir.Patina.Core.Range':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@1_0 =
        {_witness_Iterator@0_0},
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@1_0),
    begin
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                       'Elixir.Patina.Core.Range':'$patina_constructor__Range'(1,
                                                                                                                               101))
            of
            _iter@3_0 ->
                case 'main$patina_loop_1'(_witness_Iterator@0_0,
                                          _iter@3_0)
                    of
                    {break, 0, _loop_value@13_0, _iter@3_1} ->
                        _loop_value@13_0
                end
        end
    end.
