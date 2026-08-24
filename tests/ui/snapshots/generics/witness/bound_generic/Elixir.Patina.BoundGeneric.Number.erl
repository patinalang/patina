-file("tests/ui/src/generics/witness/bound_generic.pt", 5).
-module('Elixir.Patina.BoundGeneric.Number').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         'BoundGeneric::Bound::b'/2]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    [#{field => n, default => nil, required => false}];
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
    #{'__struct__' => 'Elixir.Patina.BoundGeneric.Number', n => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
'BoundGeneric::Bound::b'(_Self@0_0, _self@1_0) ->
    begin
        io:put_chars([[<<"Bound::b(">>,
                       io_lib:format("~ktp", [_self@1_0]),
                       <<")">>]]),
        io:nl()
    end,
    map_get(n, _self@1_0).
