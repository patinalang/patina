-file("tests/ui/src/flow_control/let_chains.pt", 1).

-module('Elixir.Patina.LetChains').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         let_and_bool/1,
         bool_and_let/2,
         let_let_cond/2,
         let_and_nested/3,
         or_pattern/2,
         assign_in_first/1,
         assign_in_later/1,
         assign_two_vars/1,
         local_in_operand/1,
         mutate_in_branches/1,
         deferred_init/1,
         return_in_else/1,
         return_in_then/1,
         big_else/1,
         ladder/2,
         while_let_chain/1,
         in_loop/1,
         in_closure/1]).

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

opt(_x@0_0) ->
    case _x@0_0 >= 0 of
        true -> _x@0_0;
        _ -> nil
    end.

let_and_bool(_a@0_0) ->
    case [] of
        {'$patina_label_0'} -> 0;
        _ ->
            case opt(_a@0_0) of
                _value@2_0 = _v@1_0
                    when erlang:is_integer(_value@2_0) ->
                    case _v@1_0 > 5 of
                        true -> _v@1_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

bool_and_let(_flag@0_0, _a@1_0) ->
    case [] of
        {'$patina_label_0'} -> -1;
        _ ->
            case _flag@0_0 of
                true ->
                    case opt(_a@1_0) of
                        _value@3_0 = _v@2_0
                            when erlang:is_integer(_value@3_0) ->
                            _v@2_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

let_let_cond(_a@0_0, _b@1_0) ->
    case [] of
        {'$patina_label_0'} -> 0;
        _ ->
            case opt(_a@0_0) of
                _value@3_0 = _v@2_0
                    when erlang:is_integer(_value@3_0) ->
                    case opt(_b@1_0) of
                        _value@5_0 = _w@4_0
                            when erlang:is_integer(_value@5_0) ->
                            case _v@2_0 > _w@4_0 of
                                true -> _v@2_0 + _w@4_0;
                                _ ->
                                    '$patina_marker':jump('$patina_label_0'),
                                    []
                            end;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

let_and_nested(_a@0_0, _x@1_0, _y@2_0) ->
    case [] of
        {'$patina_label_0'} -> 0;
        _ ->
            case opt(_a@0_0) of
                _value@4_0 = _v@3_0
                    when erlang:is_integer(_value@4_0) ->
                    case _x@1_0 andalso _y@2_0 of
                        true -> _v@3_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

or_pattern(_kind@0_0, _c@1_0) ->
    _s@2_0 = case _kind@0_0 =:= 0 of
                 true -> 10;
                 _ ->
                     case _kind@0_0 =:= 1 of
                         true ->
                             'Elixir.Patina.LetChains.Shape':'$patina_constructor__B'(20);
                         _ -> 'Elixir.Patina.LetChains.Shape.C'
                     end
             end,
    case [] of
        {'$patina_label_0'} -> 0;
        _ ->
            case _s@2_0 of
                _value@4_0 = _x@3_0
                    when erlang:is_integer(_value@4_0) ->
                    case _c@1_0 of
                        true -> _x@3_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                #'Elixir.Patina.LetChains.Shape':'B'{'0' = _x@3_0} ->
                    case _c@1_0 of
                        true -> _x@3_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

assign_in_first(_a@0_0) ->
    _y@1_0 = 0,
    case [] of
        {'$patina_label_0', _y@1_1} -> _y@1_1;
        _ ->
            case begin _y@1_1 = 5, opt(_a@0_0) end of
                _value@3_0 = _v@2_0
                    when erlang:is_integer(_value@3_0) ->
                    case _v@2_0 > 1 of
                        true -> _v@2_0 + _y@1_1;
                        _ ->
                            '$patina_marker':jump('$patina_label_0', _y@1_1),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0', _y@1_1),
                    []
            end
    end.

assign_in_later(_a@0_0) ->
    _y@1_0 = 0,
    _r@7_0 = case [] of
                 {'$patina_label_0', _y@1_1} ->
                     _branch_value@6_0 = _y@1_1,
                     _y@1_2 = _y@1_1,
                     _branch_value@6_0;
                 _ ->
                     case opt(_a@0_0) of
                         _value@3_0 = _v@2_0
                             when erlang:is_integer(_value@3_0) ->
                             case begin _y@1_1 = _y@1_0 + 1, _v@2_0 > 0 end of
                                 true ->
                                     case begin
                                              _y@1_2 = _y@1_1 + 10, _v@2_0 > 5
                                          end
                                         of
                                         true -> _v@2_0 * 100;
                                         _ ->
                                             '$patina_marker':jump('$patina_label_0',
                                                                   _y@1_2),
                                             []
                                     end;
                                 _ ->
                                     _branch_value@4_0 = begin
                                                             '$patina_marker':jump('$patina_label_0',
                                                                                   _y@1_1),
                                                             []
                                                         end,
                                     _y@1_2 = _y@1_1,
                                     _branch_value@4_0
                             end;
                         _ ->
                             _branch_value@5_0 = begin
                                                     '$patina_marker':jump('$patina_label_0',
                                                                           _y@1_0),
                                                     []
                                                 end,
                             _y@1_2 = _y@1_0,
                             _branch_value@5_0
                     end
             end,
    _r@7_0 * 1000 + _y@1_2.

assign_two_vars(_a@0_0) ->
    _x@1_0 = 0,
    _z@2_0 = 0,
    case [] of
        {'$patina_label_0', _x@1_1, _z@2_1} ->
            _x@1_1 * 10 + _z@2_1;
        _ ->
            case opt(_a@0_0) of
                _value@4_0 = _v@3_0
                    when erlang:is_integer(_value@4_0) ->
                    case begin _x@1_1 = _v@3_0, _v@3_0 > 1 end of
                        true ->
                            case begin _z@2_1 = _v@3_0 * 2, _v@3_0 > 5 end of
                                true -> _x@1_1 + _z@2_1;
                                _ ->
                                    '$patina_marker':jump('$patina_label_0',
                                                          _x@1_1,
                                                          _z@2_1),
                                    []
                            end;
                        _ ->
                            _branch_value@5_0 = begin
                                                    '$patina_marker':jump('$patina_label_0',
                                                                          _x@1_1,
                                                                          _z@2_0),
                                                    []
                                                end,
                            _z@2_1 = _z@2_0,
                            _branch_value@5_0
                    end;
                _ ->
                    _branch_value@6_0 = begin
                                            '$patina_marker':jump('$patina_label_0',
                                                                  _x@1_0,
                                                                  _z@2_0),
                                            []
                                        end,
                    _x@1_1 = _x@1_0,
                    _z@2_1 = _z@2_0,
                    _branch_value@6_0
            end
    end.

local_in_operand(_a@0_0) ->
    case [] of
        {'$patina_label_0'} -> -1;
        _ ->
            case opt(_a@0_0) of
                _value@2_0 = _v@1_0
                    when erlang:is_integer(_value@2_0) ->
                    case begin
                             _t@3_0 = _v@1_0, _t@3_1 = _t@3_0 + 1, _t@3_1 > 5
                         end
                        of
                        true -> _v@1_0;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

mutate_in_branches(_a@0_0) ->
    _y@1_0 = 1,
    case [] of
        {'$patina_label_0', _y@1_1} ->
            _y@1_2 = _y@1_1 - 1,
            {};
        _ ->
            _branch_value@6_0 = case opt(_a@0_0) of
                                    _value@3_0 = _v@2_0
                                        when erlang:is_integer(_value@3_0) ->
                                        case _v@2_0 > 1 of
                                            true ->
                                                _y@1_1 = _y@1_0 + _v@2_0,
                                                {};
                                            _ ->
                                                _branch_value@4_0 = begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _y@1_0),
                                                                        []
                                                                    end,
                                                _y@1_1 = _y@1_0,
                                                _branch_value@4_0
                                        end;
                                    _ ->
                                        _branch_value@5_0 = begin
                                                                '$patina_marker':jump('$patina_label_0',
                                                                                      _y@1_0),
                                                                []
                                                            end,
                                        _y@1_1 = _y@1_0,
                                        _branch_value@5_0
                                end,
            _y@1_2 = _y@1_1,
            _branch_value@6_0
    end,
    _y@1_2.

deferred_init(_a@0_0) ->
    _x@1_0 = {},
    case [] of
        {'$patina_label_0', _x@1_1} ->
            _x@1_2 = 0,
            {};
        _ ->
            _branch_value@6_0 = case opt(_a@0_0) of
                                    _value@3_0 = _v@2_0
                                        when erlang:is_integer(_value@3_0) ->
                                        case _v@2_0 > 1 of
                                            true ->
                                                _x@1_1 = _v@2_0,
                                                {};
                                            _ ->
                                                _branch_value@4_0 = begin
                                                                        '$patina_marker':jump('$patina_label_0',
                                                                                              _x@1_0),
                                                                        []
                                                                    end,
                                                _x@1_1 = _x@1_0,
                                                _branch_value@4_0
                                        end;
                                    _ ->
                                        _branch_value@5_0 = begin
                                                                '$patina_marker':jump('$patina_label_0',
                                                                                      _x@1_0),
                                                                []
                                                            end,
                                        _x@1_1 = _x@1_0,
                                        _branch_value@5_0
                                end,
            _x@1_2 = _x@1_1,
            _branch_value@6_0
    end,
    _x@1_2.

return_in_else(_a@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@4_0} -> _jump_value@4_0;
        _ ->
            _r@3_0 = case [] of
                         {'$patina_label_1'} ->
                             '$patina_marker':jump('$patina_label_0', -1),
                             [];
                         _ ->
                             case opt(_a@0_0) of
                                 _value@2_0 = _v@1_0
                                     when erlang:is_integer(_value@2_0) ->
                                     case _v@1_0 > 1 of
                                         true -> _v@1_0;
                                         _ ->
                                             '$patina_marker':jump('$patina_label_1'),
                                             []
                                     end;
                                 _ ->
                                     '$patina_marker':jump('$patina_label_1'),
                                     []
                             end
                     end,
            _r@3_0 + 1
    end.

return_in_then(_a@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@3_0} -> _jump_value@3_0;
        _ ->
            case [] of
                {'$patina_label_1'} -> {};
                _ ->
                    case opt(_a@0_0) of
                        _value@2_0 = _v@1_0
                            when erlang:is_integer(_value@2_0) ->
                            case _v@1_0 > 1 of
                                true ->
                                    '$patina_marker':jump('$patina_label_0',
                                                          _v@1_0),
                                    [];
                                _ ->
                                    '$patina_marker':jump('$patina_label_1'),
                                    []
                            end;
                        _ ->
                            '$patina_marker':jump('$patina_label_1'),
                            []
                    end
            end,
            0
    end.

big_else(_a@0_0) ->
    case [] of
        {'$patina_label_0'} ->
            io:put_chars([[<<"out of range"/utf8>>]]),
            io:nl(),
            io:put_chars([[<<"not between 2 and 9"/utf8>>]]),
            io:nl(),
            io:put_chars([[<<"or missing"/utf8>>]]),
            io:nl(),
            {};
        _ ->
            case opt(_a@0_0) of
                _value@2_0 = _v@1_0
                    when erlang:is_integer(_value@2_0) ->
                    case _v@1_0 > 1 of
                        true ->
                            case _v@1_0 < 10 of
                                true ->
                                    io:put_chars(begin
                                                     _element@4_0 = begin
                                                                        _element@3_0 =
                                                                            io_lib:format("~ktp",
                                                                                          [_v@1_0]),
                                                                        [<<"in range "/utf8>>,
                                                                         _element@3_0]
                                                                    end,
                                                     [_element@4_0]
                                                 end),
                                    io:nl(),
                                    {};
                                _ ->
                                    '$patina_marker':jump('$patina_label_0'),
                                    []
                            end;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_0'),
                    []
            end
    end.

ladder(_a@0_0, _b@1_0) ->
    case [] of
        {'$patina_label_1'} ->
            case [] of
                {'$patina_label_0'} ->
                    case _a@0_0 =:= _b@1_0 of
                        true -> 3;
                        _ -> 4
                    end;
                _ ->
                    case opt(_b@1_0) of
                        _value@5_0 = _w@4_0
                            when erlang:is_integer(_value@5_0) ->
                            case _w@4_0 > 10 of
                                true -> 2;
                                _ ->
                                    '$patina_marker':jump('$patina_label_0'),
                                    []
                            end;
                        _ ->
                            '$patina_marker':jump('$patina_label_0'),
                            []
                    end
            end;
        _ ->
            case opt(_a@0_0) of
                _value@3_0 = _v@2_0
                    when erlang:is_integer(_value@3_0) ->
                    case _v@2_0 > 10 of
                        true -> 1;
                        _ ->
                            '$patina_marker':jump('$patina_label_1'),
                            []
                    end;
                _ ->
                    '$patina_marker':jump('$patina_label_1'),
                    []
            end
    end.

while_let_chain(_limit@0_0) ->
    _i@1_0 = 0,
    _n@2_0 = 0,
    _sum@3_0 = 0,
    {_i@1_1, _n@2_1, _sum@3_1} = fun _$patina_loop(_i@1_0,
                                                   _n@2_0, _sum@3_0) ->
                                         case [] of
                                             {'$patina_label_1',
                                              _i@1_2,
                                              _n@2_2,
                                              _sum@3_2} ->
                                                 {_i@1_2, _n@2_2, _sum@3_2};
                                             _ ->
                                                 case [] of
                                                     {'$patina_label_0',
                                                      _i@1_1,
                                                      _n@2_1,
                                                      _sum@3_1} ->
                                                         {};
                                                     _ ->
                                                         case [] of
                                                             {'$patina_label_2',
                                                              _i@1_1,
                                                              _n@2_1,
                                                              _sum@3_1} ->
                                                                 '$patina_marker':jump('$patina_label_1',
                                                                                       _i@1_1,
                                                                                       _n@2_1,
                                                                                       _sum@3_1),
                                                                 [];
                                                             _ ->
                                                                 case
                                                                     opt(_i@1_0)
                                                                     of
                                                                     _value@5_0 =
                                                                         _v@4_0
                                                                         when
                                                                           erlang:is_integer(_value@5_0) ->
                                                                         case
                                                                             begin
                                                                                 _n@2_1 =
                                                                                     _n@2_0
                                                                                         +
                                                                                         1,
                                                                                 _v@4_0
                                                                                     <
                                                                                     _limit@0_0
                                                                             end
                                                                             of
                                                                             true ->
                                                                                 _i@1_1 =
                                                                                     _i@1_0
                                                                                         +
                                                                                         1,
                                                                                 case
                                                                                     _v@4_0
                                                                                         =:=
                                                                                         2
                                                                                     of
                                                                                     true ->
                                                                                         '$patina_marker':jump('$patina_label_0',
                                                                                                               _i@1_1,
                                                                                                               _n@2_1,
                                                                                                               _sum@3_0),
                                                                                         [];
                                                                                     _ ->
                                                                                         {}
                                                                                 end,
                                                                                 case
                                                                                     _v@4_0
                                                                                         =:=
                                                                                         7
                                                                                     of
                                                                                     true ->
                                                                                         '$patina_marker':jump('$patina_label_1',
                                                                                                               _i@1_1,
                                                                                                               _n@2_1,
                                                                                                               _sum@3_0),
                                                                                         [];
                                                                                     _ ->
                                                                                         {}
                                                                                 end,
                                                                                 _sum@3_1 =
                                                                                     _sum@3_0
                                                                                         +
                                                                                         _v@4_0,
                                                                                 {};
                                                                             _ ->
                                                                                 _branch_value@6_0 =
                                                                                     begin
                                                                                         '$patina_marker':jump('$patina_label_2',
                                                                                                               _i@1_0,
                                                                                                               _n@2_1,
                                                                                                               _sum@3_0),
                                                                                         []
                                                                                     end,
                                                                                 _i@1_1 =
                                                                                     _i@1_0,
                                                                                 _sum@3_1 =
                                                                                     _sum@3_0,
                                                                                 _branch_value@6_0
                                                                         end;
                                                                     _ ->
                                                                         _branch_value@7_0 =
                                                                             begin
                                                                                 '$patina_marker':jump('$patina_label_2',
                                                                                                       _i@1_0,
                                                                                                       _n@2_0,
                                                                                                       _sum@3_0),
                                                                                 []
                                                                             end,
                                                                         _n@2_1 =
                                                                             _n@2_0,
                                                                         _i@1_1 =
                                                                             _i@1_0,
                                                                         _sum@3_1 =
                                                                             _sum@3_0,
                                                                         _branch_value@7_0
                                                                 end
                                                         end
                                                 end,
                                                 _$patina_loop(_i@1_1,
                                                               _n@2_1,
                                                               _sum@3_1)
                                         end
                                 end(_i@1_0, _n@2_0, _sum@3_0),
    _sum@3_1 * 100 + _n@2_1.

in_loop(_n@0_0) ->
    _total@1_0 = 0,
    _i@2_0 = 0,
    {_i@2_1, _total@1_1} = fun _$patina_loop(_i@2_0,
                                             _total@1_0) ->
                                   case [] of
                                       {'$patina_label_0',
                                        _i@2_2,
                                        _total@1_3} ->
                                           {_i@2_2, _total@1_3};
                                       _ ->
                                           case _i@2_0 >= _n@0_0 of
                                               true ->
                                                   '$patina_marker':jump('$patina_label_0',
                                                                         _i@2_0,
                                                                         _total@1_0),
                                                   [];
                                               _ -> {}
                                           end,
                                           case [] of
                                               {'$patina_label_1',
                                                _total@1_1} ->
                                                   _total@1_2 = _total@1_1 +
                                                                    100,
                                                   {};
                                               _ ->
                                                   _branch_value@7_0 = case
                                                                           opt(_i@2_0
                                                                                   -
                                                                                   2)
                                                                           of
                                                                           _value@4_0 =
                                                                               _v@3_0
                                                                               when
                                                                                 erlang:is_integer(_value@4_0) ->
                                                                               case
                                                                                   _v@3_0
                                                                                       =/=
                                                                                       1
                                                                                   of
                                                                                   true ->
                                                                                       _total@1_1 =
                                                                                           _total@1_0
                                                                                               +
                                                                                               _v@3_0,
                                                                                       {};
                                                                                   _ ->
                                                                                       _branch_value@5_0 =
                                                                                           begin
                                                                                               '$patina_marker':jump('$patina_label_1',
                                                                                                                     _total@1_0),
                                                                                               []
                                                                                           end,
                                                                                       _total@1_1 =
                                                                                           _total@1_0,
                                                                                       _branch_value@5_0
                                                                               end;
                                                                           _ ->
                                                                               _branch_value@6_0 =
                                                                                   begin
                                                                                       '$patina_marker':jump('$patina_label_1',
                                                                                                             _total@1_0),
                                                                                       []
                                                                                   end,
                                                                               _total@1_1 =
                                                                                   _total@1_0,
                                                                               _branch_value@6_0
                                                                       end,
                                                   _total@1_2 = _total@1_1,
                                                   _branch_value@7_0
                                           end,
                                           _i@2_1 = _i@2_0 + 1,
                                           _$patina_loop(_i@2_1, _total@1_2)
                                   end
                           end(_i@2_0, _total@1_0),
    _total@1_1.

in_closure(_a@0_0) ->
    _f@4_0 = fun (_x@1_0) ->
                     case [] of
                         {'$patina_label_0'} -> 0;
                         _ ->
                             case opt(_x@1_0) of
                                 _value@3_0 = _v@2_0
                                     when erlang:is_integer(_value@3_0) ->
                                     case _v@2_0 > 1 of
                                         true -> _v@2_0;
                                         _ ->
                                             '$patina_marker':jump('$patina_label_0'),
                                             []
                                     end;
                                 _ ->
                                     '$patina_marker':jump('$patina_label_0'),
                                     []
                             end
                     end
             end,
    _f@4_0(_a@0_0) + _f@4_0(_a@0_0 + 1).
