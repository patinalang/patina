-file("tests/ui/src/generics/generic_consts.pt", 5).

-module('Elixir.Patina.GenericConsts.$impl.Core.Int').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'generic_consts::Konst::$patina_const::K'/1,
         'generic_consts::Konst::$patina_witness'/0]).

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

'generic_consts::Konst::$patina_const::K'(_witness_Konst@0_0) ->
    7.

'generic_consts::Konst::$patina_witness'() ->
    {{fun 'Elixir.Patina.GenericConsts.$impl.Core.Int':'generic_consts::Konst::$patina_const::K'/1},
     {},
     {}}.
