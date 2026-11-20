-file("tests/ui/src/fn/diverging__04.pt", 4).

-module('Elixir.Patina.Diverging04').

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
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     ['main::sum_odd_numbers'(9)]),
                                        [<<"Sum of odd numbers up to 9 (excluding): "/utf8>>,
                                         _element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {}.

'main::sum_odd_numbers'(_up_to@0_0) ->
    _acc@1_0 = 0,
    case begin
             _self@2_0 =
                 'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                         _up_to@0_0),
             _self@2_0
         end
        of
        _iter@3_0 ->
            {_iter@3_1, _acc@1_1} = fun _$patina_loop(_iter@3_0,
                                                      _acc@1_0) ->
                                            case [] of
                                                {'$patina_label_0',
                                                 _iter@3_2,
                                                 _acc@1_2} ->
                                                    {_iter@3_2, _acc@1_2};
                                                _ ->
                                                    case [] of
                                                        {'$patina_label_1',
                                                         _iter@3_1,
                                                         _acc@1_1} ->
                                                            {};
                                                        _ ->
                                                            case begin
                                                                     _self@4_0 =
                                                                         _iter@3_0,
                                                                     case
                                                                         _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                             <
                                                                             _self@4_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                                                         of
                                                                         true ->
                                                                             _0@8_0 =
                                                                                 begin
                                                                                     _element@5_0 =
                                                                                         _self@4_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                                                     _element@7_0 =
                                                                                         begin
                                                                                             _start@6_0 =
                                                                                                 _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                                                     +
                                                                                                     1,
                                                                                             _self@4_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                                              =
                                                                                                                                              _start@6_0}
                                                                                         end,
                                                                                     {_element@5_0,
                                                                                      _element@7_0}
                                                                                 end,
                                                                             _0@8_0;
                                                                         _ ->
                                                                             nil
                                                                     end
                                                                 end
                                                                of
                                                                nil ->
                                                                    _branch_value@13_0 =
                                                                        begin
                                                                            '$patina_marker':jump('$patina_label_0',
                                                                                                  _iter@3_0,
                                                                                                  _acc@1_0),
                                                                            []
                                                                        end,
                                                                    _iter@3_1 =
                                                                        _iter@3_0,
                                                                    _acc@1_1 =
                                                                        _acc@1_0,
                                                                    _branch_value@13_0;
                                                                _value@11_0 =
                                                                    {_i@9_0,
                                                                     _next@10_0}
                                                                    when
                                                                      erlang:is_tuple(_value@11_0)
                                                                          andalso
                                                                          erlang:tuple_size(_value@11_0)
                                                                              =:=
                                                                              2 ->
                                                                    _iter@3_1 =
                                                                        _next@10_0,
                                                                    _addition@12_0 =
                                                                        case
                                                                            _i@9_0
                                                                                rem
                                                                                2
                                                                                =:=
                                                                                1
                                                                            of
                                                                            true ->
                                                                                _i@9_0;
                                                                            false ->
                                                                                '$patina_marker':jump('$patina_label_1',
                                                                                                      _iter@3_1,
                                                                                                      _acc@1_0),
                                                                                []
                                                                        end,
                                                                    _acc@1_1 =
                                                                        _acc@1_0
                                                                            +
                                                                            _addition@12_0,
                                                                    {}
                                                            end
                                                    end,
                                                    _$patina_loop(_iter@3_1,
                                                                  _acc@1_1)
                                            end
                                    end(_iter@3_0, _acc@1_0),
            {}
    end,
    _acc@1_1.
