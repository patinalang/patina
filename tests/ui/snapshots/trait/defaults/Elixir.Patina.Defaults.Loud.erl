-file("tests/ui/src/trait/defaults.pt", 486).

-module('Elixir.Patina.Defaults.Loud').

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

'Defaults::Ring::ring'(_witness_self@0_0 = {_, {}, {}},
                       _witness_bounds@2_0 = {_witness_bound@1_0}, _self@3_0,
                       _t@4_0) ->
    (erlang:element(1,
                    erlang:element(1,
                                   _witness_bound@1_0)))(_witness_bound@1_0,
                                                         _t@4_0).

'Defaults::Ring::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Loud':'Defaults::Ring::ring'/4},
     {},
     {}}.
