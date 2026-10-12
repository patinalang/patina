-file("tests/ui/src/fn/diverging__04.pt", 4).

-module('Elixir.Patina.Diverging04').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         main/0,
         'main::sum_odd_numbers$patina_loop_1'/2]).

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
    begin
        io:put_chars(begin
                         _element@2_0 = begin
                                            _element@0_0 =
                                                <<"Sum of odd numbers up to 9 (excluding): "/utf8>>,
                                            _element@1_0 = io_lib:format("~ktp",
                                                                         ['main::sum_odd_numbers'(9)]),
                                            [_element@0_0, _element@1_0]
                                        end,
                         [_element@2_0]
                     end),
        io:nl(),
        {}
    end.

'main::sum_odd_numbers$patina_loop_1'(_iter@2_0,
                                      _acc@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.Range':'Core::Iterator::next<int>'('Elixir.Patina.Core.Range':'Core::Iterator::$patina_witness<int>'(),
                                                                       _iter@2_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _acc@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _acc@1_1 = _acc@1_0,
                    _branch_value@7_0;
                _value@5_0 = {_i@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin
                        _addition@6_0 = case _i@3_0 rem 2 =:= 1 of
                                            true -> _i@3_0;
                                            false ->
                                                '$patina_marker':continue(_iter@2_1,
                                                                          _acc@1_0)
                                        end,
                        _acc@1_1 = _acc@1_0 + _addition@6_0,
                        {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _acc@1_1).

'main::sum_odd_numbers'(_up_to@0_0) ->
    _acc@1_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'('Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'({'Elixir.Patina.Core.Range':'Core::Iterator::$patina_witness<int>'()}),
                                                                   'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                                                                           _up_to@0_0))
        of
        _iter@2_0 ->
            case 'main::sum_odd_numbers$patina_loop_1'(_iter@2_0,
                                                       _acc@1_0)
                of
                {break, 0, _loop_value@8_0, _iter@2_1, _acc@1_1} ->
                    _loop_value@8_0
            end
    end,
    _acc@1_1.
