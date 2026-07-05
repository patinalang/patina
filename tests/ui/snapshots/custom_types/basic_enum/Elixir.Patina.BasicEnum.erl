-file("tests/ui/src/custom_types/basic_enum.pt", 1).
-module('Elixir.Patina.BasicEnum').
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
main() ->
    _var_0 = 'Elixir.Patina.BasicEnum.BasicEnum.Unit',
    _var_1 =
        #'Elixir.Patina.BasicEnum.BasicEnum':'Tuple'{'0' = 1,
                                                     '1' = <<"hello">>},
    _var_2 =
        #'Elixir.Patina.BasicEnum.BasicEnum':'Struct'{x = 1,
                                                      y = <<"hello">>},
    begin
        io:put_chars([[io_lib:format("~tp", [_var_0]),
                       <<"\n">>,
                       io_lib:format("~tp", [_var_1]),
                       <<"\n">>,
                       io_lib:format("~tp", [_var_2])]]),
        io:nl()
    end.
