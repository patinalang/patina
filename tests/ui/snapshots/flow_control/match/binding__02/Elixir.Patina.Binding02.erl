-file("tests/ui/src/flow_control/match/binding__02.pt",
      4).

-module('Elixir.Patina.Binding02').

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

some_number() -> 42.

main() ->
    case some_number() of
        _value@1_0 = _n@0_0 = 42
            when erlang:is_integer(_value@1_0) ->
            io:put_chars(begin
                             _element@3_0 = begin
                                                _element@2_0 =
                                                    io_lib:format("~ktp",
                                                                  [_n@0_0]),
                                                [<<"The Answer: "/utf8>>,
                                                 _element@2_0,
                                                 <<"!"/utf8>>]
                                            end,
                             [_element@3_0]
                         end),
            io:nl(),
            {};
        _value@5_0 = _n@4_0
            when erlang:is_integer(_value@5_0) ->
            io:put_chars(begin
                             _element@7_0 = begin
                                                _element@6_0 =
                                                    io_lib:format("~ktp",
                                                                  [_n@4_0]),
                                                [<<"Not interesting... "/utf8>>,
                                                 _element@6_0]
                                            end,
                             [_element@7_0]
                         end),
            io:nl(),
            {};
        _ -> {}
    end.
