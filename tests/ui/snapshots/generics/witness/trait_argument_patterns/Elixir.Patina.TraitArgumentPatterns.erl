-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      1).

-module('Elixir.Patina.TraitArgumentPatterns').

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
    _witness_Contains@0_0 =
        'Elixir.Patina.TraitArgumentPatterns.UnboundContainer':'trait_argument_patterns::Contains<int, _>::$patina_witness'(),
    _witness_Marker@1_0 =
        'Elixir.Patina.TraitArgumentPatterns.Marked':'trait_argument_patterns::Marker::$patina_witness'(),
    _witnesses_where_Contains@2_0 = {_witness_Marker@1_0},
    _witness_Contains@3_0 =
        'Elixir.Patina.TraitArgumentPatterns.BoundContainer':'trait_argument_patterns::Contains<int, _>::$patina_witness'(_witnesses_where_Contains@2_0),
    _witness_Contains@4_0 =
        'Elixir.Patina.TraitArgumentPatterns.NestedContainer':'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::$patina_witness'(),
    io:put_chars(begin
                     _element@6_0 = begin
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.TraitArgumentPatterns.UnboundContainer':'trait_argument_patterns::Contains<int, _>::contains'(_witness_Contains@0_0,
                                                                                                                                                                                   'Elixir.Patina.TraitArgumentPatterns.UnboundContainer',
                                                                                                                                                                                   1,
                                                                                                                                                                                   false)]),
                                        [_element@5_0]
                                    end,
                     [_element@6_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@8_0 = begin
                                        _element@7_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.TraitArgumentPatterns.BoundContainer':'trait_argument_patterns::Contains<int, _>::contains'(_witness_Contains@3_0,
                                                                                                                                                                                 'Elixir.Patina.TraitArgumentPatterns.BoundContainer',
                                                                                                                                                                                 1,
                                                                                                                                                                                 2)]),
                                        [_element@7_0]
                                    end,
                     [_element@8_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@10_0 = begin
                                         _element@9_0 = io_lib:format("~ktp",
                                                                      ['Elixir.Patina.TraitArgumentPatterns.NestedContainer':'trait_argument_patterns::Contains<int, trait_argument_patterns::Wrapped<_>>::contains'(_witness_Contains@4_0,
                                                                                                                                                                                                                     'Elixir.Patina.TraitArgumentPatterns.NestedContainer',
                                                                                                                                                                                                                     1,
                                                                                                                                                                                                                     false)]),
                                         [_element@9_0]
                                     end,
                     [_element@10_0]
                 end),
    io:nl(),
    {}.
