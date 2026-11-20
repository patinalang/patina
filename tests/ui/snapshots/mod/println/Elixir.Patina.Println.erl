-file("tests/ui/src/mod/println.pt", 1).

-module('Elixir.Patina.Println').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, top/1]).

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

top(_n@0_0) ->
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [_n@0_0]),
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_n@0_0]),
                                        [<<"top "/utf8>>,
                                         _element@1_0,
                                         <<" "/utf8>>,
                                         _element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    {}.
