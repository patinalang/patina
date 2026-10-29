-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      4).

-module('Elixir.Patina.TraitArgumentPatterns.Marked').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TraitArgumentPatterns::Marker::value'/2,
         'TraitArgumentPatterns::Marker::$patina_witness'/0]).

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

'TraitArgumentPatterns::Marker::value'(_witness_Marker@0_0,
                                       _self@1_0) ->
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_self@1_0]),
                                        [<<"Marker::value("/utf8>>,
                                         _element@2_0,
                                         <<")"/utf8>>]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    _self@1_0.

'TraitArgumentPatterns::Marker::$patina_witness'() ->
    {{fun 'Elixir.Patina.TraitArgumentPatterns.Marked':'TraitArgumentPatterns::Marker::value'/2},
     {},
     {}}.
