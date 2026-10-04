-file("tests/ui/src/mod/split__02.pt", 7).

-module('Elixir.Patina.Split02').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, function/0, indirect_access/0]).

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

function() ->
    begin
        io:put_chars([[<<"called `MyMod::function()`"/utf8>>]]),
        io:nl(),
        {}
    end.

private_function() ->
    begin
        io:put_chars([[<<"called `MyMod::private_function()`"/utf8>>]]),
        io:nl(),
        {}
    end.

indirect_access() ->
    begin
        io:put_chars([<<"called `MyMod::indirect_access()`, that\n> "/utf8>>]),
        {}
    end,
    private_function().
