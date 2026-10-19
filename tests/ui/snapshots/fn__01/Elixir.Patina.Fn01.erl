-file("tests/ui/src/fn__01.pt", 5).

-module('Elixir.Patina.Fn01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         main/0,
         'fizzbuzz_to$patina_loop_1'/2]).

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

main() -> fizzbuzz_to(100).

is_divisible_by(_lhs@0_0, _rhs@1_0) ->
    case _rhs@1_0 =:= 0 of
        true -> '$patina_marker':return(false);
        _ -> {}
    end,
    _lhs@0_0 rem _rhs@1_0 =:= 0.

fizzbuzz(_n@0_0) ->
    case is_divisible_by(_n@0_0, 15) of
        true ->
            begin
                io:put_chars(begin
                                 _element@1_0 = [<<"fizzbuzz"/utf8>>],
                                 [_element@1_0]
                             end),
                io:nl(),
                {}
            end;
        _ ->
            case is_divisible_by(_n@0_0, 3) of
                true ->
                    begin
                        io:put_chars(begin
                                         _element@2_0 = [<<"fizz"/utf8>>],
                                         [_element@2_0]
                                     end),
                        io:nl(),
                        {}
                    end;
                _ ->
                    case is_divisible_by(_n@0_0, 5) of
                        true ->
                            begin
                                io:put_chars(begin
                                                 _element@3_0 =
                                                     [<<"buzz"/utf8>>],
                                                 [_element@3_0]
                                             end),
                                io:nl(),
                                {}
                            end;
                        _ ->
                            begin
                                io:put_chars(begin
                                                 _element@5_0 = begin
                                                                    _element@4_0 =
                                                                        io_lib:format("~ktp",
                                                                                      [_n@0_0]),
                                                                    [_element@4_0]
                                                                end,
                                                 [_element@5_0]
                                             end),
                                io:nl(),
                                {}
                            end
                    end
            end
    end.

'fizzbuzz_to$patina_loop_1'(_witness_Iterator@1_0,
                            _iter@4_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next<int>'(_witness_Iterator@1_0,
                                                                                _iter@4_0)
                of
                nil ->
                    _branch_value@8_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@4_0}),
                    _iter@4_1 = _iter@4_0,
                    _branch_value@8_0;
                _value@7_0 = {_n@5_0, _next@6_0}
                    when erlang:is_tuple(_value@7_0) andalso
                             erlang:tuple_size(_value@7_0) =:= 2 ->
                    _iter@4_1 = _next@6_0,
                    begin fizzbuzz(_n@5_0) end
            end
        end,
    '$patina_marker':continue(_witness_Iterator@1_0,
                              _iter@4_1).

fizzbuzz_to(_n@0_0) ->
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::$patina_witness<int>'(),
    _witnesses_where_IntoIterator@2_0 =
        {_witness_Iterator@1_0},
    _witness_IntoIterator@3_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@2_0),
    begin
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@3_0,
                                                                       'Elixir.Patina.Core.RangeInclusive':new(1,
                                                                                                               _n@0_0))
            of
            _iter@4_0 ->
                case 'fizzbuzz_to$patina_loop_1'(_witness_Iterator@1_0,
                                                 _iter@4_0)
                    of
                    {break, 0, _loop_value@9_0, _iter@4_1} ->
                        _loop_value@9_0
                end
        end
    end.
