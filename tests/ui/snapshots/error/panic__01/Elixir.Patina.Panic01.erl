-file("tests/ui/src/error/panic__01.pt", 4).

-module('Elixir.Patina.Panic01').

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

drink(_beverage@0_0) ->
    case _beverage@0_0 =:= <<"lemonade"/utf8>> of
        true ->
            begin
                erlang:error('Elixir.Patina.Core.Panic':new(<<"AAAaaaaa!!!!"/utf8>>),
                             none,
                             [{error_info,
                               #{module => 'Elixir.Patina.Core.Panic'}}])
            end,
            {};
        false -> {}
    end,
    begin
        io:put_chars([[<<"Some refreshing "/utf8>>,
                       _beverage@0_0,
                       <<" is all I need."/utf8>>]]),
        io:nl(),
        {}
    end.

main() ->
    drink(<<"water"/utf8>>),
    drink(<<"lemonade"/utf8>>),
    drink(<<"still water"/utf8>>).
