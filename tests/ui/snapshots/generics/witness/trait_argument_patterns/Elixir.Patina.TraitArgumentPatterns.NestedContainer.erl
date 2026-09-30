-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      3).

-module('Elixir.Patina.TraitArgumentPatterns.NestedContainer').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TraitArgumentPatterns::Contains::contains'/4]).

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

'TraitArgumentPatterns::Contains::contains'(_witness@0_0 =
                                                {'impl TraitArgumentPatterns::Contains<int, TraitArgumentPatterns::Wrapped<_>> for TraitArgumentPatterns::NestedContainer',
                                                 _},
                                            _self@1_0, _a@2_0, _wrapped@3_0) ->
    begin
        io:put_chars([[<<"NestedContainer::contains("/utf8>>,
                       io_lib:format("~ktp", [_self@1_0]),
                       <<", "/utf8>>,
                       io_lib:format("~ktp", [_a@2_0]),
                       <<", "/utf8>>,
                       io_lib:format("~ktp", [_wrapped@3_0]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    true.
