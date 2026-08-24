-file("tests/ui/src/generics/witness/forward.pt", 5).
-module('Elixir.Patina.Forward.S').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         'Forward::Tr::method'/2]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    [];
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Forward.S'}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
'Forward::Tr::method'(_Self@0_0, _self@1_0) ->
    begin
        io:put_chars([[<<"Tr::method(">>,
                       io_lib:format("~ktp", [_self@1_0]),
                       <<")">>]]),
        io:nl()
    end.
