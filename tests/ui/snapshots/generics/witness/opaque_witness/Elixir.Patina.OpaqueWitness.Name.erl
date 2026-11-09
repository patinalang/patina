-file("tests/ui/src/generics/witness/opaque_witness.pt",
      7).

-module('Elixir.Patina.OpaqueWitness.Name').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'opaque_witness::Describe::describe'/2,
         'opaque_witness::Describe::$patina_witness'/0]).

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

'opaque_witness::Describe::describe'(_witness_Describe@0_0,
                                     _self@1_0) ->
    _self@1_0.

'opaque_witness::Describe::$patina_witness'() ->
    {{fun 'Elixir.Patina.OpaqueWitness.Name':'opaque_witness::Describe::describe'/2},
     {},
     {}}.
