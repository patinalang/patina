-file("tests/ui/src/generics/witness/supertraits.pt",
      76).

-module('Elixir.Patina.Supertraits.IsEven').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Supertraits::Pred::test'/3,
         'Supertraits::Pred::$patina_witness'/0]).

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

'Supertraits::Pred::test'(_witness_Pred@0_0, _self@1_0,
                          _n@2_0) ->
    _n@2_0 rem 2 =:= 0.

'Supertraits::Pred::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits.IsEven':'Supertraits::Pred::test'/3},
     {},
     {}}.
