-file("tests/ui/src/primitives/cast_unchecked.pt", 4).

-module('Elixir.Patina.CastUnchecked.Point').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Point']).

-export(['__info__'/1, '$patina_constructor__Point'/2]).

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

-record #'Point'{x, y}.

'$patina_constructor__Point'(_x@0_0, _y@1_0) ->
    #'Point'{x = _x@0_0, y = _y@1_0}.
