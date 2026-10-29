-file("tests/ui/src/flow_control/while__01.pt", 4).

-module('Elixir.Patina.While01').

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
    _n@0_0 = 1,
    _n@0_1 = fun _$patina_loop(_n@0_0) ->
                     case [] of
                         {'$patina_label_0', _n@0_2} -> _n@0_2;
                         _ ->
                             case _n@0_0 < 101 of
                                 true ->
                                     case _n@0_0 rem 15 =:= 0 of
                                         true ->
                                             io:put_chars([[<<"fizzbuzz"/utf8>>]]),
                                             io:nl(),
                                             {};
                                         _ ->
                                             case _n@0_0 rem 3 =:= 0 of
                                                 true ->
                                                     io:put_chars([[<<"fizz"/utf8>>]]),
                                                     io:nl(),
                                                     {};
                                                 _ ->
                                                     case _n@0_0 rem 5 =:= 0 of
                                                         true ->
                                                             io:put_chars([[<<"buzz"/utf8>>]]),
                                                             io:nl(),
                                                             {};
                                                         _ ->
                                                             io:put_chars(begin
                                                                              _element@2_0 =
                                                                                  begin
                                                                                      _element@1_0 =
                                                                                          io_lib:format("~ktp",
                                                                                                        [_n@0_0]),
                                                                                      [_element@1_0]
                                                                                  end,
                                                                              [_element@2_0]
                                                                          end),
                                                             io:nl(),
                                                             {}
                                                     end
                                             end
                                     end,
                                     _n@0_1 = _n@0_0 + 1,
                                     {};
                                 _ ->
                                     _branch_value@3_0 = begin
                                                             '$patina_marker':jump('$patina_label_0',
                                                                                   _n@0_0),
                                                             []
                                                         end,
                                     _n@0_1 = _n@0_0,
                                     _branch_value@3_0
                             end,
                             _$patina_loop(_n@0_1)
                     end
             end(_n@0_0),
    {}.
