-file("tests/ui/src/mod/super__01.pt", 15).

-module('Elixir.Patina.Super01.My').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, indirect_call/0]).

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
    io:put_chars([[<<"called `my::function()`"/utf8>>]]),
    io:nl(),
    {}.

indirect_call() ->
    io:put_chars([<<"called `my::indirect_call()`, that\n> "/utf8>>]),
    function(),
    function(),
    'Elixir.Patina.Super01.My.Cool':function(),
    'Elixir.Patina.Super01':function(),
    'Elixir.Patina.Super01.Cool':function().
