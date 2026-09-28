-file("tests/ui/src/custom_types/repr__09.pt", 4).

-module('Elixir.Patina.Repr09').

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
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [case <<"hello"/utf8>> of
                                          _value@0_0
                                              when
                                                erlang:is_binary(_value@0_0) ->
                                              {othertag, _value@0_0};
                                          _value@0_0 -> _value@0_0
                                      end])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp", [42])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [case hi of
                                          _value@1_0
                                              when
                                                erlang:is_binary(_value@1_0) ->
                                              {othertag, _value@1_0};
                                          _value@1_0
                                              when (erlang:is_tuple(_value@1_0)
                                                        andalso
                                                        erlang:tuple_size(_value@1_0)
                                                            =:= 2)
                                                       andalso
                                                       erlang:element(1,
                                                                      _value@1_0)
                                                           =:= othertag ->
                                              {othertag, _value@1_0};
                                          _value@1_0
                                              when (erlang:is_tuple(_value@1_0)
                                                        andalso
                                                        erlang:tuple_size(_value@1_0)
                                                            =:= 2)
                                                       andalso
                                                       erlang:element(1,
                                                                      _value@1_0)
                                                           =:= mytag ->
                                              {othertag, _value@1_0};
                                          _value@1_0 -> _value@1_0
                                      end])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [case {othertag, 1} of
                                          _value@2_0
                                              when
                                                erlang:is_binary(_value@2_0) ->
                                              {othertag, _value@2_0};
                                          _value@2_0
                                              when (erlang:is_tuple(_value@2_0)
                                                        andalso
                                                        erlang:tuple_size(_value@2_0)
                                                            =:= 2)
                                                       andalso
                                                       erlang:element(1,
                                                                      _value@2_0)
                                                           =:= othertag ->
                                              {othertag, _value@2_0};
                                          _value@2_0
                                              when (erlang:is_tuple(_value@2_0)
                                                        andalso
                                                        erlang:tuple_size(_value@2_0)
                                                            =:= 2)
                                                       andalso
                                                       erlang:element(1,
                                                                      _value@2_0)
                                                           =:= mytag ->
                                              {othertag, _value@2_0};
                                          _value@2_0 -> _value@2_0
                                      end])]]),
        io:nl(),
        {}
    end.
