-file("tests/ui/src/generics/new_types__01.pt", 4).
-module('Elixir.Patina.NewTypes01').
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
is_adult(_var_0) ->
    map_get('0', _var_0) >= 18.
main() ->
    _var_0 =
        ('Elixir.Patina.NewTypes01.Years':'__struct__'())#{'0' := 25},
    _var_1 = 'Elixir.Patina.NewTypes01.Years':to_days(_var_0),
    begin
        io:put_chars([[<<"Is an adult? ">>,
                       io_lib:format("~tp", [is_adult(_var_0)])]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"Is an adult? ">>,
                       io_lib:format("~tp",
                                     [is_adult('Elixir.Patina.NewTypes01.Days':to_years(_var_1))])]]),
        io:nl()
    end.
