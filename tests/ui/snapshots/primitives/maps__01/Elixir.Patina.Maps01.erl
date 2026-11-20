-file("tests/ui/src/primitives/maps__01.pt", 4).

-module('Elixir.Patina.Maps01').

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

call(_number@0_0) ->
    case _number@0_0 of
        <<"798-1364"/utf8>> ->
            <<"We're sorry, the call cannot be completed "
              "as dialed.\n            Please hang "
              "up and try again."/utf8>>;
        <<"645-7689"/utf8>> ->
            <<"Hello, this is Mr. Awesome's Pizza. "
              "My name is Fred.\n            What can "
              "I get for you today?"/utf8>>;
        _ -> <<"Hi! Who is this again?"/utf8>>
    end.

main() ->
    _contacts@0_0 = #{<<"Daniel"/utf8>> =>
                          <<"798-1364"/utf8>>,
                      <<"Ashley"/utf8>> => <<"645-7689"/utf8>>,
                      <<"Katie"/utf8>> => <<"435-8291"/utf8>>,
                      <<"Robert"/utf8>> => <<"956-1745"/utf8>>},
    case begin
             _self@1_0 = _contacts@0_0,
             _key@2_0 = <<"Daniel"/utf8>>,
             case _self@1_0 of
                 #{_key@2_0 := _value@3_0} -> _value@3_0;
                 _ -> nil
             end
         end
        of
        _value@5_0 = _number@4_0
            when erlang:is_binary(_value@5_0) ->
            io:put_chars(begin
                             _element@7_0 = begin
                                                _element@6_0 =
                                                    call(_number@4_0),
                                                [<<"Calling Daniel: "/utf8>>,
                                                 _element@6_0]
                                            end,
                             [_element@7_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Daniel's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@0_1 = begin
                        _self@8_0 = _contacts@0_0,
                        _key@9_0 = <<"Daniel"/utf8>>,
                        _value@10_0 = <<"164-6743"/utf8>>,
                        maps:put(_key@9_0, _value@10_0, _self@8_0)
                    end,
    case begin
             _self@11_0 = _contacts@0_1,
             _key@12_0 = <<"Ashley"/utf8>>,
             case _self@11_0 of
                 #{_key@12_0 := _value@13_0} -> _value@13_0;
                 _ -> nil
             end
         end
        of
        _value@15_0 = _number@14_0
            when erlang:is_binary(_value@15_0) ->
            io:put_chars(begin
                             _element@17_0 = begin
                                                 _element@16_0 =
                                                     call(_number@14_0),
                                                 [<<"Calling Ashley: "/utf8>>,
                                                  _element@16_0]
                                             end,
                             [_element@17_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Ashley's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@0_2 = begin
                        _self@18_0 = _contacts@0_1,
                        _key@19_0 = <<"Ashley"/utf8>>,
                        maps:remove(_key@19_0, _self@18_0)
                    end,
    _contacts@0_3 = _contacts@0_2#{<<"Laura"/utf8>> =>
                                       <<"132-8372"/utf8>>},
    case begin
             _self@20_0 = _contacts@0_3,
             _self@21_0 = _self@20_0,
             _0@22_0 = maps:iterator(_self@21_0),
             _0@22_0
         end
        of
        _iter@23_0 ->
            _iter@23_1 = fun _$patina_loop(_iter@23_0) ->
                                 case [] of
                                     {'$patina_label_0', _iter@23_2} ->
                                         _iter@23_2;
                                     _ ->
                                         case begin
                                                  _self@24_0 = _iter@23_0,
                                                  case maps:next(_self@24_0) of
                                                      {_key@25_0,
                                                       _value@26_0,
                                                       _rest@27_0} ->
                                                          _entry@30_0 = begin
                                                                            _element@28_0 =
                                                                                _key@25_0,
                                                                            _element@29_0 =
                                                                                _value@26_0,
                                                                            {_element@28_0,
                                                                             _element@29_0}
                                                                        end,
                                                          _0@32_0 = begin
                                                                        _element@31_0 =
                                                                            _rest@27_0,
                                                                        {_entry@30_0,
                                                                         _element@31_0}
                                                                    end,
                                                          _0@32_0;
                                                      _ -> nil
                                                  end
                                              end
                                             of
                                             nil ->
                                                 _branch_value@40_0 = begin
                                                                          '$patina_marker':jump('$patina_label_0',
                                                                                                _iter@23_0),
                                                                          []
                                                                      end,
                                                 _iter@23_1 = _iter@23_0,
                                                 _branch_value@40_0;
                                             _value@36_0 = {{_contact@33_0,
                                                             _number@34_0},
                                                            _next@35_0}
                                                 when
                                                   erlang:is_tuple(_value@36_0)
                                                       andalso
                                                       erlang:tuple_size(_value@36_0)
                                                           =:= 2 ->
                                                 _iter@23_1 = _next@35_0,
                                                 io:put_chars(begin
                                                                  _element@39_0 =
                                                                      begin
                                                                          _element@37_0 =
                                                                              _contact@33_0,
                                                                          _element@38_0 =
                                                                              call(_number@34_0),
                                                                          [<<"Calling "/utf8>>,
                                                                           _element@37_0,
                                                                           <<": "/utf8>>,
                                                                           _element@38_0]
                                                                      end,
                                                                  [_element@39_0]
                                                              end),
                                                 io:nl(),
                                                 {}
                                         end,
                                         _$patina_loop(_iter@23_1)
                                 end
                         end(_iter@23_0),
            {}
    end.
