-file("tests/ui/src/fn/diverging__03.pt", 4).

-module('Elixir.Patina.Diverging03').

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

some_fn() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"This call never returns."/utf8>>),
                     none,
                     [{error_info, #{module => 'Elixir.Patina.Core.Panic'}}])
    end.

main() ->
    __a@0_0 = some_fn(),
    begin
        io:put_chars([[<<"You will never see this line!"/utf8>>]]),
        io:nl(),
        {}
    end.
