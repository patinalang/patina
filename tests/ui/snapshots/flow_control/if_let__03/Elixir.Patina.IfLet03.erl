-file("tests/ui/src/flow_control/if_let__03.pt", 5).

-module('Elixir.Patina.IfLet03').

-compile([no_auto_import]).

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
    _c@2_0 = #'Elixir.Patina.IfLet03.Foo':'Qux'{'0' = 100},
    case _a@0_0 of
        'Elixir.Patina.IfLet03.Foo.Bar' ->
            begin io:put_chars([[<<"a is foobar">>]]), io:nl() end;
        _ -> {}
    end,
    case _b@1_0 of
        'Elixir.Patina.IfLet03.Foo.Bar' ->
            begin io:put_chars([[<<"b is foobar">>]]), io:nl() end;
        _ -> {}
    end,
    case _c@2_0 of
        #'Elixir.Patina.IfLet03.Foo':'Qux'{'0' = _value@3_0} ->
            begin
                io:put_chars([[<<"c is ">>,
                               io_lib:format("~ktp", [_value@3_0])]]),
                io:nl()
            end;
        _ -> {}
    end,
    case _c@2_0 of
        #'Elixir.Patina.IfLet03.Foo':'Qux'{'0' =
                                               _value@4_0 = 100} ->
            begin
                io:put_chars([[<<"c is one hundred">>]]), io:nl()
            end;
        _ -> {}
    end.
