-file("tests/ui/src/trait/defaults.pt", 488).

-module('Elixir.Patina.Defaults.Quiet').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Ring::ring'/4,
         'Defaults::Ring::$patina_witness'/0]).

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

'Defaults::Ring::ring'(_witness_Ring@0_0,
                       _witnesses_where_Ring_ring@1_0, _self@2_0, __t@3_0) ->
    <<"..."/utf8>>.

'Defaults::Ring::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::ring'/4},
     {},
     {}}.
