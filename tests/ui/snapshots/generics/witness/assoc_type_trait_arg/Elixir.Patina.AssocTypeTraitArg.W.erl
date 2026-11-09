-file("tests/ui/src/generics/witness/assoc_type_trai"
      "t_arg.pt",
      12).

-module('Elixir.Patina.AssocTypeTraitArg.W').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'assoc_type_trait_arg::Wraps<<_ as assoc_type_trait_arg::Container>::Item>::wrapped'/2,
         'assoc_type_trait_arg::Wraps<<_ as assoc_type_trait_arg::Container>::Item>::$patina_witness'/1]).

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

'assoc_type_trait_arg::Wraps<<_ as assoc_type_trait_arg::Container>::Item>::wrapped'(_witness_Wraps@0_0,
                                                                                     _self@1_0) ->
    0.

'assoc_type_trait_arg::Wraps<<_ as assoc_type_trait_arg::Container>::Item>::$patina_witness'(_witnesses_where_Wraps@0_0) ->
    {{fun 'Elixir.Patina.AssocTypeTraitArg.W':'assoc_type_trait_arg::Wraps<<_ as assoc_type_trait_arg::Container>::Item>::wrapped'/2},
     {},
     _witnesses_where_Wraps@0_0}.
