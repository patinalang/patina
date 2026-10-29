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
    _witness_IntoIterator@0_0 =
        'Elixir.Patina.Core.Map':'Core::IntoIterator::$patina_witness'(),
    _witness_Iterator@1_0 =
        'Elixir.Patina.Core.MapIter':'Core::Iterator::$patina_witness'(),
    _contacts@2_0 = #{<<"Daniel"/utf8>> =>
                          <<"798-1364"/utf8>>,
                      <<"Ashley"/utf8>> => <<"645-7689"/utf8>>,
                      <<"Katie"/utf8>> => <<"435-8291"/utf8>>,
                      <<"Robert"/utf8>> => <<"956-1745"/utf8>>},
    case 'Elixir.Patina.Core.Map':get(_contacts@2_0,
                                      <<"Daniel"/utf8>>)
        of
        _value@4_0 = _number@3_0
            when erlang:is_binary(_value@4_0) ->
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@5_0 =
                                                    call(_number@3_0),
                                                [<<"Calling Daniel: "/utf8>>,
                                                 _element@5_0]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Daniel's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@2_1 =
        'Elixir.Patina.Core.Map':insert(_contacts@2_0,
                                        <<"Daniel"/utf8>>,
                                        <<"164-6743"/utf8>>),
    case 'Elixir.Patina.Core.Map':get(_contacts@2_1,
                                      <<"Ashley"/utf8>>)
        of
        _value@8_0 = _number@7_0
            when erlang:is_binary(_value@8_0) ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@9_0 =
                                                     call(_number@7_0),
                                                 [<<"Calling Ashley: "/utf8>>,
                                                  _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Ashley's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@2_2 =
        'Elixir.Patina.Core.Map':remove(_contacts@2_1,
                                        <<"Ashley"/utf8>>),
    _contacts@2_3 = _contacts@2_2#{<<"Laura"/utf8>> =>
                                       <<"132-8372"/utf8>>},
    case
        'Elixir.Patina.Core.Map':'Core::IntoIterator::into_iter'(_witness_IntoIterator@0_0,
                                                                 _contacts@2_3)
        of
        _iter@11_0 ->
            _iter@11_1 = fun _$patina_loop(_iter@11_0) ->
                                 case [] of
                                     {'$patina_label_0', _iter@11_2} ->
                                         _iter@11_2;
                                     _ ->
                                         case
                                             'Elixir.Patina.Core.MapIter':'Core::Iterator::next'(_witness_Iterator@1_0,
                                                                                                 _iter@11_0)
                                             of
                                             nil ->
                                                 _branch_value@19_0 = begin
                                                                          '$patina_marker':jump('$patina_label_0',
                                                                                                _iter@11_0),
                                                                          []
                                                                      end,
                                                 _iter@11_1 = _iter@11_0,
                                                 _branch_value@19_0;
                                             _value@15_0 = {{_contact@12_0,
                                                             _number@13_0},
                                                            _next@14_0}
                                                 when
                                                   erlang:is_tuple(_value@15_0)
                                                       andalso
                                                       erlang:tuple_size(_value@15_0)
                                                           =:= 2 ->
                                                 _iter@11_1 = _next@14_0,
                                                 io:put_chars(begin
                                                                  _element@18_0 =
                                                                      begin
                                                                          _element@16_0 =
                                                                              _contact@12_0,
                                                                          _element@17_0 =
                                                                              call(_number@13_0),
                                                                          [<<"Calling "/utf8>>,
                                                                           _element@16_0,
                                                                           <<": "/utf8>>,
                                                                           _element@17_0]
                                                                      end,
                                                                  [_element@18_0]
                                                              end),
                                                 io:nl(),
                                                 {}
                                         end,
                                         _$patina_loop(_iter@11_1)
                                 end
                         end(_iter@11_0),
            {}
    end.
