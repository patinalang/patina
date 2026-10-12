-file("tests/ui/src/generics/witness/opaque_witness.pt",
      1).

-module('Elixir.Patina.OpaqueWitness').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, show_opaque_return/0]).

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

make() -> <<"a"/utf8>>.

show(_witness_bounds@1_0 = {_witness_bound@0_0},
     _t@2_0) ->
    (erlang:element(1,
                    erlang:element(1,
                                   _witness_bound@0_0)))(_witness_bound@0_0,
                                                         _t@2_0).

show_opaque_return() ->
    show({'Elixir.Patina.OpaqueWitness.Name':'OpaqueWitness::Describe::$patina_witness'()},
         make()).
