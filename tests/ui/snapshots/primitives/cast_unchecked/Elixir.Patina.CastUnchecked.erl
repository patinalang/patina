-file("tests/ui/src/primitives/cast_unchecked.pt", 1).

-module('Elixir.Patina.CastUnchecked').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         inferred_target/1,
         explicit_target/1,
         term_into_struct/1,
         round_trip/0]).

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

inferred_target(_map@0_0) ->
    begin maps:keys(_map@0_0) end.

explicit_target(_xs@0_0) ->
    _len@1_0 = begin erlang:length(_xs@0_0) end,
    _len@1_0 * 2.

term_into_struct(_point@0_0) ->
    #'Elixir.Patina.CastUnchecked.Point':'Point'{x = _x@1_0,
                                                 y = _y@2_0} =
        begin _point@0_0 end,
    _x@1_0 + _y@2_0.

round_trip() ->
    term_into_struct('Elixir.Patina.CastUnchecked.Point':'$patina_constructor__Point'(1,
                                                                                      2)).
