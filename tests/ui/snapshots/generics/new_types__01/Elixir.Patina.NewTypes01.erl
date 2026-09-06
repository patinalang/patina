-file("tests/ui/src/generics/new_types__01.pt", 4).

-module('Elixir.Patina.NewTypes01').

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

is_adult(_age@0_0) ->
    erlang:map_get('0', _age@0_0) >= 18.

main() ->
    _age@0_0 =
        ('Elixir.Patina.NewTypes01.Years':'__struct__'())#{'0'
                                                               := 25},
    _age_days@1_0 =
        'Elixir.Patina.NewTypes01.Years':to_days(_age@0_0),
    begin
        io:put_chars([[<<"Is an adult? ">>,
                       io_lib:format("~ktp", [is_adult(_age@0_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"Is an adult? ">>,
                       io_lib:format("~ktp",
                                     [is_adult('Elixir.Patina.NewTypes01.Days':to_years(_age_days@1_0))])]]),
        io:nl(),
        {}
    end.
