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
    _names@0_0 = [<<"Bob"/utf8>>,
                  <<"Frank"/utf8>>,
                  <<"Ferris"/utf8>>],
    case begin
             _self@2_0 = begin _self@1_0 = _names@0_0, _self@1_0 end,
             _self@2_0
         end
        of
        _iter@3_0 ->
            _iter@3_1 = fun _$patina_loop(_iter@3_0) ->
                                case [] of
                                    {'$patina_label_0', _iter@3_2} -> _iter@3_2;
                                    _ ->
                                        case begin
                                                 _self@4_0 = _iter@3_0,
                                                 case _self@4_0 of
                                                     [_head@6_0 | _tail@5_0 =
                                                                      _] ->
                                                         _0@8_0 = begin
                                                                      _element@7_0 =
                                                                          _tail@5_0,
                                                                      {_head@6_0,
                                                                       _element@7_0}
                                                                  end,
                                                         _0@8_0;
                                                     [] -> nil
                                                 end
                                             end
                                            of
                                            nil ->
                                                _branch_value@14_0 = begin
                                                                         '$patina_marker':jump('$patina_label_0',
                                                                                               _iter@3_0),
                                                                         []
                                                                     end,
                                                _iter@3_1 = _iter@3_0,
                                                _branch_value@14_0;
                                            _value@11_0 = {_name@9_0,
                                                           _next@10_0}
                                                when
                                                  erlang:is_tuple(_value@11_0)
                                                      andalso
                                                      erlang:tuple_size(_value@11_0)
                                                          =:= 2 ->
                                                _iter@3_1 = _next@10_0,
                                                case _name@9_0 of
                                                    <<"Ferris"/utf8>> ->
                                                        io:put_chars([[<<"There is a rustacean among us!"/utf8>>]]),
                                                        io:nl(),
                                                        {};
                                                    _ ->
                                                        io:put_chars(begin
                                                                         _element@13_0 =
                                                                             begin
                                                                                 _element@12_0 =
                                                                                     _name@9_0,
                                                                                 [<<"Hello "/utf8>>,
                                                                                  _element@12_0]
                                                                             end,
                                                                         [_element@13_0]
                                                                     end),
                                                        io:nl(),
                                                        {}
                                                end
                                        end,
                                        _$patina_loop(_iter@3_1)
                                end
                        end(_iter@3_0),
            {}
    end,
    io:put_chars(begin
                     _element@16_0 = begin
                                         _element@15_0 = io_lib:format("~ktp",
                                                                       [_names@0_0]),
                                         [<<"names: "/utf8>>, _element@15_0]
                                     end,
                     [_element@16_0]
                 end),
    io:nl(),
    {}.
