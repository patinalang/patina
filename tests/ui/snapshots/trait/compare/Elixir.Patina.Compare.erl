-file("tests/ui/src/trait/compare.pt", 1).

-module('Elixir.Patina.Compare').

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

compare(_witness@0_0, _a@1_0, _b@2_0) ->
    begin
        _witness@3_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                          _a@1_0,
                                                          _b@2_0)
    end.

main() ->
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [compare({'Elixir.Patina.Core.Int':'Core::PartialEq::$patina_witness'()},
                                              1,
                                              1)])]]),
        io:nl(),
        {}
    end.
