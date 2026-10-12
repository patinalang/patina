-file("tests/ui/src/flow_control/if_let__03.pt", 5).

-module('Elixir.Patina.IfLet03').

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
    _a@0_0 = 'Elixir.Patina.IfLet03.Foo.Bar',
    _b@1_0 = 'Elixir.Patina.IfLet03.Foo.Baz',
    _c@2_0 = 100,
    case _a@0_0 of
        'Elixir.Patina.IfLet03.Foo.Bar' ->
            begin
                io:put_chars(begin
                                 _element@3_0 = [<<"a is foobar"/utf8>>],
                                 [_element@3_0]
                             end),
                io:nl(),
                {}
            end;
        _ -> {}
    end,
    case _b@1_0 of
        'Elixir.Patina.IfLet03.Foo.Bar' ->
            begin
                io:put_chars(begin
                                 _element@4_0 = [<<"b is foobar"/utf8>>],
                                 [_element@4_0]
                             end),
                io:nl(),
                {}
            end;
        _ -> {}
    end,
    case _c@2_0 of
        _value@6_0 = _value@5_0
            when erlang:is_integer(_value@6_0) ->
            begin
                io:put_chars(begin
                                 _element@9_0 = begin
                                                    _element@7_0 =
                                                        <<"c is "/utf8>>,
                                                    _element@8_0 =
                                                        io_lib:format("~ktp",
                                                                      [_value@5_0]),
                                                    [_element@7_0, _element@8_0]
                                                end,
                                 [_element@9_0]
                             end),
                io:nl(),
                {}
            end;
        _ -> {}
    end,
    case _c@2_0 of
        _value@11_0 = _value@10_0 = 100
            when erlang:is_integer(_value@11_0) ->
            begin
                io:put_chars(begin
                                 _element@12_0 = [<<"c is one hundred"/utf8>>],
                                 [_element@12_0]
                             end),
                io:nl(),
                {}
            end;
        _ -> {}
    end.
