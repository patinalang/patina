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
         panic_in_loop/1,
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
                     begin
                         _element@1_0 = begin
                                            _element@0_0 = #{module =>
                                                                 'Elixir.Patina.Core.Panic'},
                                            {error_info, _element@0_0}
                                        end,
                         [_element@1_0]
                     end)
    end.

panic_with_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"unexpected value"/utf8>>),
                     none,
                     begin
                         _element@1_0 = begin
                                            _element@0_0 = #{module =>
                                                                 'Elixir.Patina.Core.Panic'},
                                            {error_info, _element@0_0}
                                        end,
                         [_element@1_0]
                     end)
    end.

todo_without_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented"/utf8>>),
                     none,
                     begin
                         _element@1_0 = begin
                                            _element@0_0 = #{module =>
                                                                 'Elixir.Patina.Core.Panic'},
                                            {error_info, _element@0_0}
                                        end,
                         [_element@1_0]
                     end)
    end.

todo_with_message() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented: handle the value"/utf8>>),
                     none,
                     begin
                         _element@1_0 = begin
                                            _element@0_0 = #{module =>
                                                                 'Elixir.Patina.Core.Panic'},
                                            {error_info, _element@0_0}
                                        end,
                         [_element@1_0]
                     end)
    end.

panic_in_let_else(_map@0_0) ->
    begin
        begin _value@1_0 = {} end,
        case _map@0_0 of
            #{1 := _value@1_1} -> {};
            _ ->
                _branch_value@4_0 = begin
                                        begin
                                            erlang:error('Elixir.Patina.Core.Panic':new(<<"missing key"/utf8>>),
                                                         none,
                                                         begin
                                                             _element@3_0 =
                                                                 begin
                                                                     _element@2_0 =
                                                                         #{module
                                                                               =>
                                                                               'Elixir.Patina.Core.Panic'},
                                                                     {error_info,
                                                                      _element@2_0}
                                                                 end,
                                                             [_element@3_0]
                                                         end)
                                        end
                                    end,
                _value@1_1 = _value@1_0,
                _branch_value@4_0
        end
    end,
    _value@1_1.

todo_in_let_else(_map@0_0) ->
    begin
        begin _value@1_0 = {} end,
        case _map@0_0 of
            #{1 := _value@1_1} -> {};
            _ ->
                _branch_value@4_0 = begin
                                        begin
                                            erlang:error('Elixir.Patina.Core.Panic':new(<<"not yet implemented"/utf8>>),
                                                         none,
                                                         begin
                                                             _element@3_0 =
                                                                 begin
                                                                     _element@2_0 =
                                                                         #{module
                                                                               =>
                                                                               'Elixir.Patina.Core.Panic'},
                                                                     {error_info,
                                                                      _element@2_0}
                                                                 end,
                                                             [_element@3_0]
                                                         end)
                                        end
                                    end,
                _value@1_1 = _value@1_0,
                _branch_value@4_0
        end
    end,
    _value@1_1.

panic_in_loop(_limit@0_0) ->
    _i@1_0 = 0,
    fun _$patina_loop(_i@1_0) ->
            _ = begin
                    case _i@1_0 =:= _limit@0_0 of
                        true ->
                            begin
                                erlang:error('Elixir.Patina.Core.Panic':new(<<"reached the limit"/utf8>>),
                                             none,
                                             begin
                                                 _element@3_0 = begin
                                                                    _element@2_0 =
                                                                        #{module
                                                                              =>
                                                                              'Elixir.Patina.Core.Panic'},
                                                                    {error_info,
                                                                     _element@2_0}
                                                                end,
                                                 [_element@3_0]
                                             end)
                            end;
                        _ -> {}
                    end,
                    _i@1_1 = _i@1_0 + 1,
                    {}
                end,
            _$patina_loop(_i@1_1)
    end(_i@1_0).

panic_in_nested_loop(_limit@0_0) ->
    _i@1_0 = 0,
    fun _$patina_loop(_i@1_0) ->
            _ = begin
                    _j@2_0 = 0,
                    case fun _$patina_loop(_j@2_0) ->
                                 _ = begin
                                         case _j@2_0 < _limit@0_0 of
                                             true ->
                                                 case _i@1_0 =:= _limit@0_0 of
                                                     true ->
                                                         begin
                                                             erlang:error('Elixir.Patina.Core.Panic':new(<<"reached the limit"/utf8>>),
                                                                          none,
                                                                          begin
                                                                              _element@4_0 =
                                                                                  begin
                                                                                      _element@3_0 =
                                                                                          #{module
                                                                                                =>
                                                                                                'Elixir.Patina.Core.Panic'},
                                                                                      {error_info,
                                                                                       _element@3_0}
                                                                                  end,
                                                                              [_element@4_0]
                                                                          end)
                                                         end;
                                                     _ -> {}
                                                 end,
                                                 _j@2_1 = _j@2_0 + 1,
                                                 {};
                                             _ ->
                                                 _branch_value@5_0 = begin
                                                                         begin
                                                                             '$patina_marker':break({break,
                                                                                                     1,
                                                                                                     {},
                                                                                                     _j@2_0}),
                                                                             []
                                                                         end
                                                                     end,
                                                 _j@2_1 = _j@2_0,
                                                 _branch_value@5_0
                                         end
                                     end,
                                 _$patina_loop(_j@2_1)
                         end(_j@2_0)
                        of
                        {break, 1, _loop_value@6_0, _j@2_1} -> _loop_value@6_0
                    end,
                    _i@1_1 = _i@1_0 + 1,
                    {}
                end,
            _$patina_loop(_i@1_1)
    end(_i@1_0).
