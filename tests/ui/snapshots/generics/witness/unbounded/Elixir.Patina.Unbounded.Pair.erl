-file("tests/ui/src/generics/witness/unbounded.pt", 3).

-module('Elixir.Patina.Unbounded.Pair').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export(['__info__'/1,
         '$patina_constructor__Pair'/2,
         len/1]).

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

-record #'Pair'{a, b}.

'$patina_constructor__Pair'(_a@0_0, _b@1_0) ->
    #'Pair'{a = _a@0_0, b = _b@1_0}.

len(_self@0_0) ->
    begin
        io:put_chars([[<<"Pair::len("/utf8>>,
                       io_lib:format("~ktp", [_self@0_0]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    2.
