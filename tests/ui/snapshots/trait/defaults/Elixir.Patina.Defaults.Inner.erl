-file("tests/ui/src/trait/defaults.pt", 179).

-module('Elixir.Patina.Defaults.Inner').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Inner::Doubler::doubled'/2]).

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

helper(_x@0_0) -> _x@0_0 * 2.

'Defaults::Inner::Doubler::doubled'(_witness_self@0_0,
                                    _self@1_0) ->
    helper((erlang:element(1,
                           erlang:element(1,
                                          _witness_self@0_0)))(_witness_self@0_0,
                                                               _self@1_0)).
