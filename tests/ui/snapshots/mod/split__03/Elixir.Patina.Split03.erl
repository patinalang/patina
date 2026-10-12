-file("tests/ui/src/mod/split__03.pt", 4).

-module('Elixir.Patina.Split03').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, function/0]).

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
        io:put_chars(begin
                         _element@0_0 =
                             [<<"called `MyMod::Nested::function()`"/utf8>>],
                         [_element@0_0]
                     end),
        io:nl(),
        {}
    end.

private_function() ->
    begin
        io:put_chars(begin
                         _element@0_0 =
                             [<<"called `MyMod::Nested::private_function()`"/utf8>>],
                         [_element@0_0]
                     end),
        io:nl(),
        {}
    end.
