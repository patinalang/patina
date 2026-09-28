-file("tests/ui/src/trait/defaults.pt", 486).

-module('Elixir.Patina.Defaults.Loud').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'Defaults::Ring::ring'/4]).

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

'Defaults::Ring::ring'(_Self@1_0, _T@0_0, _self@2_0,
                       _t@3_0) ->
    (erlang:element(1, _T@0_0)):'Defaults::Tag::tag'(_T@0_0,
                                                     _t@3_0).
