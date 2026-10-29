-file("tests/ui/src/flow_control/match/binding__01.pt",
      5).

-module('Elixir.Patina.Binding01').

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

age() -> 15.

main() ->
    io:put_chars([[<<"Tell me what type of person you are"/utf8>>]]),
    io:nl(),
    case [] of
        {'$patina_label_0', _n@12_0} ->
            io:put_chars(begin
                             _element@14_0 = begin
                                                 _element@13_0 =
                                                     io_lib:format("~ktp",
                                                                   [_n@12_0]),
                                                 [<<"I'm a teen of age "/utf8>>,
                                                  _element@13_0]
                                             end,
                             [_element@14_0]
                         end),
            io:nl(),
            {};
        _ ->
            case age() of
                0 ->
                    io:put_chars([[<<"I haven't celebrated my first birthday "
                                     "yet"/utf8>>]]),
                    io:nl(),
                    {};
                _n@0_0 = _range@1_0
                    when _range@1_0 >= 1, _range@1_0 =< 12 ->
                    io:put_chars(begin
                                     _element@3_0 = begin
                                                        _element@2_0 =
                                                            io_lib:format("~ktp",
                                                                          [_n@0_0]),
                                                        [<<"I'm a child of age "/utf8>>,
                                                         _element@2_0]
                                                    end,
                                     [_element@3_0]
                                 end),
                    io:nl(),
                    {};
                _n@4_0 = _range@5_0
                    when _range@5_0 >= 13, _range@5_0 =< 19 ->
                    io:put_chars(begin
                                     _element@7_0 = begin
                                                        _element@6_0 =
                                                            io_lib:format("~ktp",
                                                                          [_n@4_0]),
                                                        [<<"I'm a teen of age "/utf8>>,
                                                         _element@6_0]
                                                    end,
                                     [_element@7_0]
                                 end),
                    io:nl(),
                    {};
                _n@8_0 = 1 ->
                    '$patina_marker':jump('$patina_label_0', _n@8_0),
                    [];
                _n@8_0 = 7 ->
                    '$patina_marker':jump('$patina_label_0', _n@8_0),
                    [];
                _n@8_0 = 15 ->
                    '$patina_marker':jump('$patina_label_0', _n@8_0),
                    [];
                _n@8_0 = 13 ->
                    '$patina_marker':jump('$patina_label_0', _n@8_0),
                    [];
                _n@9_0 ->
                    io:put_chars(begin
                                     _element@11_0 = begin
                                                         _element@10_0 =
                                                             io_lib:format("~ktp",
                                                                           [_n@9_0]),
                                                         [<<"I'm an old person of age "/utf8>>,
                                                          _element@10_0]
                                                     end,
                                     [_element@11_0]
                                 end),
                    io:nl(),
                    {}
            end
    end.
