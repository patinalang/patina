-file("tests/ui/src/fn/diverging__01.pt", 4).

-module('Elixir.Patina.Diverging01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1]).

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

foo() ->
    begin
        erlang:error('Elixir.Patina.Core.Panic':new(<<"This call never returns."/utf8>>),
                     none,
                     begin
                         _element@1_0 = begin
                                            _element@0_0 = #{module =>
                                                                 'Elixir.Patina.Core.Panic'},
                                            {error_info, _element@0_0}
                                        end,
                         [_element@1_0]
                     end)
    end.
