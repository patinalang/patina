-file("tests/ui/src/flow_control/for__03.pt", 4).

-module('Elixir.Patina.For03').

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
        'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
    _witnesses_where_IntoIterator@1_0 =
        {_witness_Iterator@0_0},
    _witness_IntoIterator@2_0 =
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'(_witnesses_where_IntoIterator@1_0),
    begin
        _names@3_0 = [<<"Bob"/utf8>>,
                      <<"Frank"/utf8>>,
                      <<"Ferris"/utf8>>],
        case
            'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'(_witness_IntoIterator@2_0,
                                                                       'Elixir.Patina.Core.List':iter(_names@3_0))
            of
            _iter@4_0 ->
                _iter@4_1 = fun _$patina_loop(_iter@4_0) ->
                                    case [] of
                                        {'$patina_label_0', _iter@4_2} ->
                                            _iter@4_2;
                                        _ ->
                                            _ = begin
                                                    case
                                                        'Elixir.Patina.Core.ListIter':'Core::Iterator::next'(_witness_Iterator@0_0,
                                                                                                             _iter@4_0)
                                                        of
                                                        nil ->
                                                            _branch_value@12_0 =
                                                                begin
                                                                    '$patina_marker':jump('$patina_label_0',
                                                                                          _iter@4_0),
                                                                    []
                                                                end,
                                                            _iter@4_1 =
                                                                _iter@4_0,
                                                            _branch_value@12_0;
                                                        _value@7_0 = {_name@5_0,
                                                                      _next@6_0}
                                                            when
                                                              erlang:is_tuple(_value@7_0)
                                                                  andalso
                                                                  erlang:tuple_size(_value@7_0)
                                                                      =:= 2 ->
                                                            _iter@4_1 =
                                                                _next@6_0,
                                                            begin
                                                                case _name@5_0
                                                                    of
                                                                    <<"Ferris"/utf8>> ->
                                                                        io:put_chars(begin
                                                                                         _element@8_0 =
                                                                                             [<<"There is a rustacean among us!"/utf8>>],
                                                                                         [_element@8_0]
                                                                                     end),
                                                                        io:nl(),
                                                                        {};
                                                                    _ ->
                                                                        io:put_chars(begin
                                                                                         _element@11_0 =
                                                                                             begin
                                                                                                 _element@9_0 =
                                                                                                     <<"Hello "/utf8>>,
                                                                                                 _element@10_0 =
                                                                                                     _name@5_0,
                                                                                                 [_element@9_0,
                                                                                                  _element@10_0]
                                                                                             end,
                                                                                         [_element@11_0]
                                                                                     end),
                                                                        io:nl(),
                                                                        {}
                                                                end
                                                            end
                                                    end
                                                end,
                                            _$patina_loop(_iter@4_1)
                                    end
                            end(_iter@4_0),
                {}
        end,
        begin
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@13_0 =
                                                     <<"names: "/utf8>>,
                                                 _element@14_0 =
                                                     io_lib:format("~ktp",
                                                                   [_names@3_0]),
                                                 [_element@13_0, _element@14_0]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            {}
        end
    end.
