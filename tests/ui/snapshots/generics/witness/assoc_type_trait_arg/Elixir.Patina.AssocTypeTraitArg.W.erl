-file("tests/ui/src/generics/witness/assoc_type_trai"
      "t_arg.pt",
      12).

-module('Elixir.Patina.AssocTypeTraitArg.W').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'AssocTypeTraitArg::Wraps<<_ as AssocTypeTraitArg::Container>::Item>::wrapped'/2,
         'AssocTypeTraitArg::Wraps<<_ as AssocTypeTraitArg::Container>::Item>::$patina_witness'/1]).

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

'AssocTypeTraitArg::Wraps<<_ as AssocTypeTraitArg::Container>::Item>::wrapped'(_witness_Wraps@0_0,
                                                                               _self@1_0) ->
    0.

'AssocTypeTraitArg::Wraps<<_ as AssocTypeTraitArg::Container>::Item>::$patina_witness'(_witnesses_where_Wraps@0_0) ->
    {{fun 'Elixir.Patina.AssocTypeTraitArg.W':'AssocTypeTraitArg::Wraps<<_ as AssocTypeTraitArg::Container>::Item>::wrapped'/2},
     {},
     _witnesses_where_Wraps@0_0}.
