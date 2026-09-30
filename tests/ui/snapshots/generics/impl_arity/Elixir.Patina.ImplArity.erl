-file("tests/ui/src/generics/impl_arity.pt", 1).

-module('Elixir.Patina.ImplArity').

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

main() ->
    _a@0_0 = 5,
    _b@1_0 = <<"hi"/utf8>>,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     ['Elixir.Patina.ImplArity.GenVal':'value<int>'(_a@0_0)]),
                       <<" "/utf8>>,
                       'Elixir.Patina.ImplArity.GenVal':'value<str>'(_b@1_0,
                                                                     0)]]),
        io:nl(),
        {}
    end.
