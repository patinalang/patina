-file("tests/ui/src/generics/witness/trait_argument_patterns.pt", 3).
-module('Elixir.Patina.TraitArgumentPatterns.NestedContainer').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         'TraitArgumentPatterns::Contains::contains'/6]).
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
    #{'__struct__' =>
          'Elixir.Patina.TraitArgumentPatterns.NestedContainer'}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
'TraitArgumentPatterns::Contains::contains'(_Self@0_0,
                                            _A@1_0 =
                                                {'Elixir.Patina.Core.int',
                                                 {}},
                                            _B@2_0 =
                                                {'Elixir.Patina.TraitArgumentPatterns.Wrapped',
                                                 {_T@3_0}},
                                            _self@4_0, _a@5_0,
                                            _wrapped@6_0) ->
    begin
        io:put_chars([[<<"NestedContainer::contains(">>,
                       io_lib:format("~ktp", [_self@4_0]),
                       <<", ">>,
                       io_lib:format("~ktp", [_a@5_0]),
                       <<", ">>,
                       io_lib:format("~ktp", [_wrapped@6_0]),
                       <<")">>]]),
        io:nl()
    end,
    true.
