-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      2).

-module('Elixir.Patina.TraitArgumentPatterns.BoundContainer').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TraitArgumentPatterns::Contains::contains'/6]).

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

'TraitArgumentPatterns::Contains::contains'(_Self@1_0,
                                            _A@2_0 = {'Elixir.Patina.Core.int',
                                                      {}},
                                            _T@0_0, _self@3_0, _a@4_0,
                                            _value@5_0) ->
    begin
        io:put_chars([[<<"BoundContainer::contains(">>,
                       io_lib:format("~ktp", [_self@3_0]),
                       <<", ">>,
                       io_lib:format("~ktp", [_a@4_0]),
                       <<", ">>,
                       io_lib:format("~ktp", [_value@5_0]),
                       <<")">>]]),
        io:nl(),
        {}
    end,
    (erlang:element(1,
                    _T@0_0)):'TraitArgumentPatterns::Marker::value'(_T@0_0,
                                                                    _value@5_0)
        > 0.
