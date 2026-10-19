-file("tests/ui/src/generics/witness/trait_argument_"
      "patterns.pt",
      2).

-module('Elixir.Patina.TraitArgumentPatterns.BoundContainer').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'TraitArgumentPatterns::Contains<int, _>::contains'/4,
         'TraitArgumentPatterns::Contains<int, _>::$patina_witness'/1]).

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

'TraitArgumentPatterns::Contains<int, _>::contains'(_witness_Contains@0_0,
                                                    _self@1_0, _a@2_0,
                                                    _value@3_0) ->
    _where_Contains@4_0 = erlang:element(3,
                                         _witness_Contains@0_0),
    _witness_Marker@5_0 = erlang:element(1,
                                         _where_Contains@4_0),
    _vtable_Marker@6_0 = erlang:element(1,
                                        _witness_Marker@5_0),
    _fn_Marker_value@7_0 = erlang:element(1,
                                          _vtable_Marker@6_0),
    begin
        begin
            io:put_chars(begin
                             _element@15_0 = begin
                                                 _element@8_0 =
                                                     <<"BoundContainer::contains("/utf8>>,
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_self@1_0]),
                                                 _element@10_0 = <<", "/utf8>>,
                                                 _element@11_0 =
                                                     io_lib:format("~ktp",
                                                                   [_a@2_0]),
                                                 _element@12_0 = <<", "/utf8>>,
                                                 _element@13_0 =
                                                     io_lib:format("~ktp",
                                                                   [_value@3_0]),
                                                 _element@14_0 = <<")"/utf8>>,
                                                 [_element@8_0,
                                                  _element@9_0,
                                                  _element@10_0,
                                                  _element@11_0,
                                                  _element@12_0,
                                                  _element@13_0,
                                                  _element@14_0]
                                             end,
                             [_element@15_0]
                         end),
            io:nl(),
            {}
        end,
        _fn_Marker_value@7_0(_witness_Marker@5_0, _value@3_0) >
            0
    end.

'TraitArgumentPatterns::Contains<int, _>::$patina_witness'(_witnesses_where_Contains@0_0) ->
    {{fun 'Elixir.Patina.TraitArgumentPatterns.BoundContainer':'TraitArgumentPatterns::Contains<int, _>::contains'/4},
     {},
     _witnesses_where_Contains@0_0}.
