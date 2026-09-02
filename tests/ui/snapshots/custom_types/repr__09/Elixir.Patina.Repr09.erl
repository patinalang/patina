-file("tests/ui/src/custom_types/repr__09.pt", 4).

-module('Elixir.Patina.Repr09').

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
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [#'Elixir.Patina.Repr09.TransparentWithTags':'MaybeUntagged'{'0'
                                                                                                      =
                                                                                                      <<"hello">>}])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [#'Elixir.Patina.Repr09.TransparentWithTags':'MaybeUntagged'{'0'
                                                                                                      =
                                                                                                      42}])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [#'Elixir.Patina.Repr09.TransparentWithTags':'MaybeUntagged'{'0'
                                                                                                      =
                                                                                                      hi}])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [#'Elixir.Patina.Repr09.TransparentWithTags':'MaybeUntagged'{'0'
                                                                                                      =
                                                                                                      {othertag,
                                                                                                       1}}])]]),
        io:nl(),
        {}
    end.
