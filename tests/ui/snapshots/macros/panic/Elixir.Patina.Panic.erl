-file("tests/ui/src/macros/panic.pt", 1).

-module('Elixir.Patina.Panic').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         explicit_panic/0,
         panic_with_message/0,
         todo_without_message/0,
         todo_with_message/0,
         panic_in_let_else/1,
         todo_in_let_else/1,
         'panic_in_loop$patina_loop_1'/2,
         panic_in_loop/1,
         'panic_in_nested_loop$patina_loop_2'/3,
         'panic_in_nested_loop$patina_loop_1'/2,
         panic_in_nested_loop/1]).

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

explicit_panic() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"explicit panic"/utf8>>),
                     none,
                     [{error_info, #{module => 'Elixir.Patina.Core.Panic'}}])
    end.

panic_with_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"unexpected value"/utf8>>),
                     none,
                     [{error_info, #{module => 'Elixir.Patina.Core.Panic'}}])
    end.

todo_without_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented"/utf8>>),
                     none,
                     [{error_info, #{module => 'Elixir.Patina.Core.Panic'}}])
    end.

todo_with_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented: handle the value"/utf8>>),
                     none,
                     [{error_info, #{module => 'Elixir.Patina.Core.Panic'}}])
    end.

panic_in_let_else(_map@0_0) ->
    begin
        _value@1_0 = {},
        case _map@0_0 of
            #{1 := _value@1_1} -> {};
            _ ->
                _branch_value@2_0 = begin
                                        begin
                                            erlang:error('Elixir.Patina.Core.Panic':new(<<"missing key"/utf8>>),
                                                         none,
                                                         [{error_info,
                                                           #{module =>
                                                                 'Elixir.Patina.Core.Panic'}}])
                                        end
                                    end,
                _value@1_1 = _value@1_0,
                _branch_value@2_0
        end
    end,
    _value@1_1.

todo_in_let_else(_map@0_0) ->
    begin
        _value@1_0 = {},
        case _map@0_0 of
            #{1 := _value@1_1} -> {};
            _ ->
                _branch_value@2_0 = begin
                                        begin
                                            erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented"/utf8>>),
                                                         none,
                                                         [{error_info,
                                                           #{module =>
                                                                 'Elixir.Patina.Core.Panic'}}])
                                        end
                                    end,
                _value@1_1 = _value@1_0,
                _branch_value@2_0
        end
    end,
    _value@1_1.

'panic_in_loop$patina_loop_1'(_i@1_0, _limit@0_0) ->
    _ = begin
            case _i@1_0 =:= _limit@0_0 of
                true ->
                    begin
                        erlang:error('Elixir.Patina.Core.Panic':new(<<"reached the limit"/utf8>>),
                                     none,
                                     [{error_info,
                                       #{module =>
                                             'Elixir.Patina.Core.Panic'}}])
                    end;
                false -> {}
            end,
            _i@1_1 = _i@1_0 + 1,
            {}
        end,
    '$patina_marker':continue(_i@1_1, _limit@0_0).

panic_in_loop(_limit@0_0) ->
    _i@1_0 = 0,
    'panic_in_loop$patina_loop_1'(_i@1_0, _limit@0_0).

'panic_in_nested_loop$patina_loop_2'(_j@2_0, _limit@0_0,
                                     _i@1_0) ->
    _ = begin
            case _j@2_0 < _limit@0_0 of
                true ->
                    case _i@1_0 =:= _limit@0_0 of
                        true ->
                            begin
                                erlang:error('Elixir.Patina.Core.Panic':new(<<"reached the limit"/utf8>>),
                                             none,
                                             [{error_info,
                                               #{module =>
                                                     'Elixir.Patina.Core.Panic'}}])
                            end;
                        false -> {}
                    end,
                    _j@2_1 = _j@2_0 + 1,
                    {};
                false ->
                    _branch_value@3_0 = begin
                                            '$patina_marker':break({break,
                                                                    1,
                                                                    {},
                                                                    _j@2_0})
                                        end,
                    _j@2_1 = _j@2_0,
                    _branch_value@3_0
            end
        end,
    '$patina_marker':continue(_j@2_1, _limit@0_0, _i@1_0).

'panic_in_nested_loop$patina_loop_1'(_limit@0_0,
                                     _i@1_0) ->
    _ = begin
            _j@2_0 = 0,
            case 'panic_in_nested_loop$patina_loop_2'(_j@2_0,
                                                      _limit@0_0,
                                                      _i@1_0)
                of
                {break, 1, _loop_value@4_0, _j@2_1} -> _loop_value@4_0
            end,
            _i@1_1 = _i@1_0 + 1,
            {}
        end,
    '$patina_marker':continue(_limit@0_0, _i@1_1).

panic_in_nested_loop(_limit@0_0) ->
    _i@1_0 = 0,
    'panic_in_nested_loop$patina_loop_1'(_limit@0_0,
                                         _i@1_0).
