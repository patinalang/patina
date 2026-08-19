-file("tests/ui/src/macros/designators__01.pt", 4).
-module('Elixir.Patina.Designators01').
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
foo() ->
    begin
        io:put_chars([[<<"You called ">>,
                       io_lib:format("~tp", [<<"foo">>]),
                       <<"()">>]]),
        io:nl()
    end.
bar() ->
    begin
        io:put_chars([[<<"You called ">>,
                       io_lib:format("~tp", [<<"bar">>]),
                       <<"()">>]]),
        io:nl()
    end.
main() ->
    foo(),
    bar(),
    begin
        io:put_chars([[io_lib:format("~tp", [<<"1 + 1">>]),
                       <<" = ">>,
                       io_lib:format("~tp", [1 + 1])]]),
        io:nl()
    end,
    begin
        io:put_chars([[io_lib:format("~tp",
                                     [<<"{ let x = 1; x * x + 2 * x - 1"
                                        " }">>]),
                       <<" = ">>,
                       io_lib:format("~tp",
                                     [begin
                                          _x@0_1 = 1,
                                          _x@0_1 * _x@0_1 + 2 * _x@0_1
                                          -
                                          1
                                      end])]]),
        io:nl()
    end.
