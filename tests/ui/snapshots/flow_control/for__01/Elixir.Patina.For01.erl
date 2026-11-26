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

main() ->
    _iter@1_0 = begin
                    _self@0_0 =
                        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(1,
                                                                                101),
                    _self@0_0
                end,
    _iter@1_1 = fun _$patina_loop(_iter@1_0) ->
                        case [] of
                            {'$patina_label_0', _iter@1_2} -> _iter@1_2;
                            _ ->
                                case begin
                                         _self@2_0 = _iter@1_0,
                                         case
                                             _self@2_0#'Elixir.Patina.Core.Range':'Range'.start
                                                 <
                                                 _self@2_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                             of
                                             true ->
                                                 _0@6_0 = begin
                                                              _element@3_0 =
                                                                  _self@2_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                              _element@5_0 =
                                                                  begin
                                                                      _start@4_0 =
                                                                          _self@2_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                              +
                                                                              1,
                                                                      _self@2_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                       =
                                                                                                                       _start@4_0}
                                                                  end,
                                                              {_element@3_0,
                                                               _element@5_0}
                                                          end,
                                                 _0@6_0;
                                             _ -> nil
                                         end
                                     end
                                    of
                                    nil ->
                                        _branch_value@12_0 = begin
                                                                 '$patina_marker':jump('$patina_label_0',
                                                                                       _iter@1_0),
                                                                 []
                                                             end,
                                        _iter@1_1 = _iter@1_0,
                                        _branch_value@12_0;
                                    _value@9_0 = {_n@7_0, _next@8_0}
                                        when erlang:is_tuple(_value@9_0) andalso
                                                 erlang:tuple_size(_value@9_0)
                                                     =:= 2 ->
                                        _iter@1_1 = _next@8_0,
                                        case _n@7_0 rem 15 =:= 0 of
                                            true ->
                                                io:put_chars([[<<"fizzbuzz"/utf8>>]]),
                                                io:nl(),
                                                {};
                                            _ ->
                                                case _n@7_0 rem 3 =:= 0 of
                                                    true ->
                                                        io:put_chars([[<<"fizz"/utf8>>]]),
                                                        io:nl(),
                                                        {};
                                                    _ ->
                                                        case _n@7_0 rem 5 =:= 0
                                                            of
                                                            true ->
                                                                io:put_chars([[<<"buzz"/utf8>>]]),
                                                                io:nl(),
                                                                {};
                                                            _ ->
                                                                io:put_chars(begin
                                                                                 _element@11_0 =
                                                                                     begin
                                                                                         _element@10_0 =
                                                                                             io_lib:format("~ktp",
                                                                                                           [_n@7_0]),
                                                                                         [_element@10_0]
                                                                                     end,
                                                                                 [_element@11_0]
                                                                             end),
                                                                io:nl(),
                                                                {}
                                                        end
                                                end
                                        end
                                end,
                                _$patina_loop(_iter@1_1)
                        end
                end(_iter@1_0),
    {}.
