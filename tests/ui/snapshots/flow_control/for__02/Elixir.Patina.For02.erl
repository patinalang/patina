-file("tests/ui/src/flow_control/for__02.pt", 4).

-module('Elixir.Patina.For02').

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
    _witness_Iterator@0_0 =
        'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@1_0 =
        {_witness_Iterator@0_0},
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@1_0),
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                   'Elixir.Patina.Core.RangeInclusive':new(1,
                                                                                                           100))
        of
        _iter@3_0 ->
            _iter@3_1 = fun _$patina_loop(_iter@3_0) ->
                                case [] of
                                    {'$patina_label_0', _iter@3_2} -> _iter@3_2;
                                    _ ->
                                        case
                                            'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next<int>'(_witness_Iterator@0_0,
                                                                                                            _iter@3_0)
                                            of
                                            nil ->
                                                _branch_value@9_0 = begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _iter@3_0),
                                                                        []
                                                                    end,
                                                _iter@3_1 = _iter@3_0,
                                                _branch_value@9_0;
                                            _value@6_0 = {_n@4_0, _next@5_0}
                                                when erlang:is_tuple(_value@6_0)
                                                         andalso
                                                         erlang:tuple_size(_value@6_0)
                                                             =:= 2 ->
                                                _iter@3_1 = _next@5_0,
                                                case _n@4_0 rem 15 =:= 0 of
                                                    true ->
                                                        io:put_chars([[<<"fizzbuzz"/utf8>>]]),
                                                        io:nl(),
                                                        {};
                                                    _ ->
                                                        case _n@4_0 rem 3 =:= 0
                                                            of
                                                            true ->
                                                                io:put_chars([[<<"fizz"/utf8>>]]),
                                                                io:nl(),
                                                                {};
                                                            _ ->
                                                                case _n@4_0 rem
                                                                         5
                                                                         =:= 0
                                                                    of
                                                                    true ->
                                                                        io:put_chars([[<<"buzz"/utf8>>]]),
                                                                        io:nl(),
                                                                        {};
                                                                    _ ->
                                                                        io:put_chars(begin
                                                                                         _element@8_0 =
                                                                                             begin
                                                                                                 _element@7_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_n@4_0]),
                                                                                                 [_element@7_0]
                                                                                             end,
                                                                                         [_element@8_0]
                                                                                     end),
                                                                        io:nl(),
                                                                        {}
                                                                end
                                                        end
                                                end
                                        end,
                                        _$patina_loop(_iter@3_1)
                                end
                        end(_iter@3_0),
            {}
    end.
