-file("tests/ui/src/flow_control/loop__01.pt", 5).

-module('Elixir.Patina.Loop01').

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
    _count@0_0 = 0,
    io:put_chars([[<<"Let's count until infinity!"/utf8>>]]),
    io:nl(),
    _count@0_1 = fun _$patina_loop(_count@0_0) ->
                         case [] of
                             {'$patina_label_1', _count@0_2} -> _count@0_2;
                             _ ->
                                 case [] of
                                     {'$patina_label_0', _count@0_1} -> {};
                                     _ ->
                                         _count@0_1 = _count@0_0 + 1,
                                         case _count@0_1 =:= 3 of
                                             true ->
                                                 io:put_chars([[<<"three"/utf8>>]]),
                                                 io:nl(),
                                                 '$patina_marker':jump('$patina_label_0',
                                                                       _count@0_1),
                                                 [];
                                             _ -> {}
                                         end,
                                         io:put_chars(begin
                                                          _element@2_0 = begin
                                                                             _element@1_0 =
                                                                                 io_lib:format("~ktp",
                                                                                               [_count@0_1]),
                                                                             [_element@1_0]
                                                                         end,
                                                          [_element@2_0]
                                                      end),
                                         io:nl(),
                                         case _count@0_1 =:= 5 of
                                             true ->
                                                 io:put_chars([[<<"OK, that's enough"/utf8>>]]),
                                                 io:nl(),
                                                 '$patina_marker':jump('$patina_label_1',
                                                                       _count@0_1),
                                                 [];
                                             _ -> {}
                                         end
                                 end,
                                 _$patina_loop(_count@0_1)
                         end
                 end(_count@0_0),
    {}.
