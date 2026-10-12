-file("tests/ui/src/primitives/maps__01.pt", 4).

-module('Elixir.Patina.Maps01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'main$patina_loop_1'/1, main/0]).

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

'main$patina_loop_1'(_iter@5_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.MapIter':'Core::Iterator::next'('Elixir.Patina.Core.MapIter':'Core::Iterator::$patina_witness'(),
                                                                    _iter@5_0)
                of
                nil ->
                    _branch_value@10_0 = '$patina_marker':break({break,
                                                                 0,
                                                                 {},
                                                                 _iter@5_0}),
                    _iter@5_1 = _iter@5_0,
                    _branch_value@10_0;
                _value@9_0 = {{_contact@6_0, _number@7_0}, _next@8_0}
                    when erlang:is_tuple(_value@9_0) andalso
                             erlang:tuple_size(_value@9_0) =:= 2 ->
                    _iter@5_1 = _next@8_0,
                    begin
                        begin
                            io:put_chars([[<<"Calling "/utf8>>,
                                           _contact@6_0,
                                           <<": "/utf8>>,
                                           call(_number@7_0)]]),
                            io:nl(),
                            {}
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@5_1).

main() ->
    _contacts@0_0 = #{<<"Daniel"/utf8>> =>
                          <<"798-1364"/utf8>>,
                      <<"Ashley"/utf8>> => <<"645-7689"/utf8>>,
                      <<"Katie"/utf8>> => <<"435-8291"/utf8>>,
                      <<"Robert"/utf8>> => <<"956-1745"/utf8>>},
    case 'Elixir.Patina.Core.Map':get(_contacts@0_0,
                                      <<"Daniel"/utf8>>)
        of
        _value@2_0 = _number@1_0
            when erlang:is_binary(_value@2_0) ->
            io:put_chars([[<<"Calling Daniel: "/utf8>>,
                           call(_number@1_0)]]),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Daniel's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@0_1 =
        'Elixir.Patina.Core.Map':insert(_contacts@0_0,
                                        <<"Daniel"/utf8>>,
                                        <<"164-6743"/utf8>>),
    case 'Elixir.Patina.Core.Map':get(_contacts@0_1,
                                      <<"Ashley"/utf8>>)
        of
        _value@4_0 = _number@3_0
            when erlang:is_binary(_value@4_0) ->
            io:put_chars([[<<"Calling Ashley: "/utf8>>,
                           call(_number@3_0)]]),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"Don't have Ashley's number."/utf8>>]]),
            io:nl(),
            {}
    end,
    _contacts@0_2 =
        'Elixir.Patina.Core.Map':remove(_contacts@0_1,
                                        <<"Ashley"/utf8>>),
    _contacts@0_3 = _contacts@0_2#{<<"Laura"/utf8>> =>
                                       <<"132-8372"/utf8>>},
    case
        'Elixir.Patina.Core.Map':'Core::IntoIterator::into_iter'('Elixir.Patina.Core.Map':'Core::IntoIterator::$patina_witness'(),
                                                                 _contacts@0_3)
        of
        _iter@5_0 ->
            case 'main$patina_loop_1'(_iter@5_0) of
                {break, 0, _loop_value@11_0, _iter@5_1} ->
                    _loop_value@11_0
            end
    end.
