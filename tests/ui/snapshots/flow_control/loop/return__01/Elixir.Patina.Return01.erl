-file("tests/ui/src/flow_control/loop/return__01.pt",
      4).

-module('Elixir.Patina.Return01').

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
    _counter@0_0 = 0,
    _result@4_0 = begin
                      {_jump_value@3_0, _counter@0_1} = fun
                                                            _$patina_loop(_counter@0_0) ->
                                                                case [] of
                                                                    {'$patina_label_0',
                                                                     _jump_value@2_0,
                                                                     _counter@0_2} ->
                                                                        {_jump_value@2_0,
                                                                         _counter@0_2};
                                                                    _ ->
                                                                        _counter@0_1 =
                                                                            _counter@0_0
                                                                                +
                                                                                1,
                                                                        case
                                                                            _counter@0_1
                                                                                =:=
                                                                                10
                                                                            of
                                                                            true ->
                                                                                _jump_value@1_0 =
                                                                                    _counter@0_1
                                                                                        *
                                                                                        2,
                                                                                '$patina_marker':jump('$patina_label_0',
                                                                                                      _jump_value@1_0,
                                                                                                      _counter@0_1),
                                                                                [];
                                                                            _ ->
                                                                                {}
                                                                        end,
                                                                        _$patina_loop(_counter@0_1)
                                                                end
                                                        end(_counter@0_0),
                      _jump_value@3_0
                  end,
    case {_result@4_0, 20} of
        {_left_val@5_0, _right_val@6_0} ->
            case not (_left_val@5_0 =:= _right_val@6_0) of
                true ->
                    erlang:error('Elixir.Patina.Core.Panic':new(unicode:characters_to_binary(begin
                                                                                                 _element@7_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_left_val@5_0]),
                                                                                                 _element@8_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_right_val@6_0]),
                                                                                                 [<<"assertion `left == right` failed"/utf8>>,
                                                                                                  <<""/utf8>>,
                                                                                                  <<"\n  left: "/utf8>>,
                                                                                                  _element@7_0,
                                                                                                  <<"\n right: "/utf8>>,
                                                                                                  _element@8_0]
                                                                                             end)),
                                 none,
                                 [{error_info,
                                   #{module => 'Elixir.Patina.Core.Panic'}}]);
                _ -> {}
            end
    end.
