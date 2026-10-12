-file("tests/ui/src/flow_control/for__03.pt", 4).

-module('Elixir.Patina.For03').

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

'main$patina_loop_1'(_iter@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.ListIter':'Core::Iterator::next'('Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'(),
                                                                     _iter@1_0)
                of
                nil ->
                    _branch_value@5_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@1_0}),
                    _iter@1_1 = _iter@1_0,
                    _branch_value@5_0;
                _value@4_0 = {_name@2_0, _next@3_0}
                    when erlang:is_tuple(_value@4_0) andalso
                             erlang:tuple_size(_value@4_0) =:= 2 ->
                    _iter@1_1 = _next@3_0,
                    begin
                        case _name@2_0 of
                            <<"Ferris"/utf8>> ->
                                io:put_chars([[<<"There is a rustacean among us!"/utf8>>]]),
                                io:nl(),
                                {};
                            _ ->
                                io:put_chars([[<<"Hello "/utf8>>, _name@2_0]]),
                                io:nl(),
                                {}
                        end
                    end
            end
        end,
    '$patina_marker':continue(_iter@1_1).

main() ->
    _names@0_0 = [<<"Bob"/utf8>>,
                  <<"Frank"/utf8>>,
                  <<"Ferris"/utf8>>],
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'('Elixir.Patina.Core.$impl':'Core::IntoIterator::$patina_witness'({'Elixir.Patina.Core.ListIter':'Core::Iterator::$patina_witness'()}),
                                                                   'Elixir.Patina.Core.List':iter(_names@0_0))
        of
        _iter@1_0 ->
            case 'main$patina_loop_1'(_iter@1_0) of
                {break, 0, _loop_value@6_0, _iter@1_1} ->
                    _loop_value@6_0
            end
    end,
    begin
        io:put_chars([[<<"names: "/utf8>>,
                       io_lib:format("~ktp", [_names@0_0])]]),
        io:nl(),
        {}
    end.
