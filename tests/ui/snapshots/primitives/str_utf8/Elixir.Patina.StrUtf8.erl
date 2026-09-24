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
        <<104, 195, 169, 108, 108, 111, 44, 32, 119, 195, 182,
          114, 108, 100, 32, 226, 136, 158>> ->
            true;
        _ -> false
    end.

main() -> {}.
