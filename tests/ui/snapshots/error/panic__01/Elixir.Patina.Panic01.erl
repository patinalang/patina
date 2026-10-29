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
            erlang:error('Elixir.Patina.Core.Panic':new(<<"AAAaaaaa!!!!"/utf8>>),
                         none,
                         [{error_info,
                           #{module => 'Elixir.Patina.Core.Panic'}}]);
        _ -> {}
    end,
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@1_0 = _beverage@0_0,
                                        [<<"Some refreshing "/utf8>>,
                                         _element@1_0,
                                         <<" is all I need."/utf8>>]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.

main() ->
    drink(<<"water"/utf8>>),
    drink(<<"lemonade"/utf8>>),
    drink(<<"still water"/utf8>>).
