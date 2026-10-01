-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_structures__01.pt",
      5).

-module('Elixir.Patina.DestructureStructures01.main.Foo').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Foo']).

-export(['__info__'/1, '$patina_constructor__Foo'/2]).

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

-record #'Foo'{x, y}.

'$patina_constructor__Foo'(_x@0_0, _y@1_0) ->
    #'Foo'{x = _x@0_0, y = _y@1_0}.
