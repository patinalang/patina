-file("tests/ui/src/flow_control/match/binding__01.pt", 5).
-module('Elixir.Patina.Binding01').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,main/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
age() ->
    15.
main() ->
    begin
        io:put_chars([[<<"Tell me what type of person you are">>]]),
        io:nl()
    end,
    case age() of
        0 ->
            io:put_chars([[<<"I haven't celebrated my first birthday ye"
                             "t">>]]),
            io:nl();
        _n@0_0 = _range@1_0 when _range@1_0 >= 1, _range@1_0 =< 12 ->
            io:put_chars([[<<"I'm a child of age ">>,
                           io_lib:format("~ktp", [_n@0_0])]]),
            io:nl();
        _n@2_0 = _range@3_0 when _range@3_0 >= 13, _range@3_0 =< 19 ->
            io:put_chars([[<<"I'm a teen of age ">>,
                           io_lib:format("~ktp", [_n@2_0])]]),
            io:nl();
        _n@4_0 = 1 ->
            io:put_chars([[<<"I'm a teen of age ">>,
                           io_lib:format("~ktp", [_n@4_0])]]),
            io:nl();
        _n@4_0 = 7 ->
            io:put_chars([[<<"I'm a teen of age ">>,
                           io_lib:format("~ktp", [_n@4_0])]]),
            io:nl();
        _n@4_0 = 15 ->
            io:put_chars([[<<"I'm a teen of age ">>,
                           io_lib:format("~ktp", [_n@4_0])]]),
            io:nl();
        _n@4_0 = 13 ->
            io:put_chars([[<<"I'm a teen of age ">>,
                           io_lib:format("~ktp", [_n@4_0])]]),
            io:nl();
        _n@5_0 ->
            io:put_chars([[<<"I'm an old person of age ">>,
                           io_lib:format("~ktp", [_n@5_0])]]),
            io:nl()
    end.
