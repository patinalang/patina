-file("tests/ui/src/fn__01.pt", 5).

-module('Elixir.Patina.Fn01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         main/0,
         'fizzbuzz_to$patina_loop_1'/1]).

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
        false -> {}
    end,
    _lhs@0_0 rem _rhs@1_0 =:= 0.

fizzbuzz(_n@0_0) ->
    case is_divisible_by(_n@0_0, 15) of
        true ->
            begin
                io:put_chars([[<<"fizzbuzz"/utf8>>]]), io:nl(), {}
            end;
        false ->
            case is_divisible_by(_n@0_0, 3) of
                true ->
                    begin
                        io:put_chars([[<<"fizz"/utf8>>]]), io:nl(), {}
                    end;
                false ->
                    case is_divisible_by(_n@0_0, 5) of
                        true ->
                            begin
                                io:put_chars([[<<"buzz"/utf8>>]]), io:nl(), {}
                            end;
                        false ->
                            begin
                                io:put_chars([[io_lib:format("~ktp",
                                                             [_n@0_0])]]),
                                io:nl(),
                                {}
                            end
                    end
            end
    end.

'fizzbuzz_to$patina_loop_1'(_iter@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::next<int>'('Elixir.Patina.Core.RangeInclusive':'Core::Iterator::$patina_witness<int>'(),
                                                                                _iter@1_0)
                of
                nil ->
                    _branch_value@5_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@1_0}),
                    _iter@1_1 = _iter@1_0,
                    _branch_value@5_0;
                _value@4_0 = {_n@2_0, _next@3_0}
                    when erlang:is_tuple(_value@4_0) andalso
                             erlang:tuple_size(_value@4_0) =:= 2 ->
                    _iter@1_1 = _next@3_0,
                    begin fizzbuzz(_n@2_0) end
            end
        end,
    '$patina_marker':continue(_iter@1_1).

fizzbuzz_to(_n@0_0) ->
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'('Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'({'Elixir.Patina.Core.RangeInclusive':'Core::Iterator::$patina_witness<int>'()}),
                                                                   'Elixir.Patina.Core.RangeInclusive':new(1,
                                                                                                           _n@0_0))
        of
        _iter@1_0 ->
            case 'fizzbuzz_to$patina_loop_1'(_iter@1_0) of
                {break, 0, _loop_value@6_0, _iter@1_1} ->
                    _loop_value@6_0
            end
    end.
