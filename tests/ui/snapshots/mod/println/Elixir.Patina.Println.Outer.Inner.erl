-file("tests/ui/src/mod/println.pt", 11).

-module('Elixir.Patina.Println.Outer.Inner').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, inner/1]).

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

inner(_n@0_0) ->
    unicode:characters_to_binary(begin
                                     _element@1_0 = io_lib:format("~ktp",
                                                                  [_n@0_0]),
                                     [<<"inner "/utf8>>, _element@1_0]
                                 end).
