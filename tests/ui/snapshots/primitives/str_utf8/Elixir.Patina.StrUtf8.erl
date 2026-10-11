-file("tests/ui/src/primitives/str_utf8.pt", 3).

-module('Elixir.Patina.StrUtf8').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         greeting/0,
         is_greeting/1,
         main/0]).

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

greeting() -> <<"héllo, wörld ∞"/utf8>>.

is_greeting(_s@0_0) ->
    case _s@0_0 of
        <<"héllo, wörld ∞"/utf8>> -> true;
        _ -> false
    end.

main() -> {}.
