-file("tests/ui/src/primitives/maps_patterns.pt", 1).

-module('Elixir.Patina.MapsPatterns').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         lookup_name/1,
         lookup_name_in_term_keyed_map/1,
         nested_lookup/1,
         is_map/1]).

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

lookup_name(_map@0_0) ->
    case _map@0_0 of
        #{<<110, 97, 109, 101>> := _name@1_0} -> _name@1_0;
        _ -> -1
    end.

lookup_name_in_term_keyed_map(_map@0_0) ->
    case _map@0_0 of
        #{<<110, 97, 109, 101>> := _name@1_0} -> _name@1_0;
        _ -> -1
    end.

nested_lookup(_map@0_0) ->
    case _map@0_0 of
        #{<<111, 117, 116, 101, 114>> :=
              #{<<105, 110, 110, 101, 114>> := _value@1_0}} ->
            _value@1_0;
        _ -> -1
    end.

is_map(_map@0_0) -> case _map@0_0 of #{} -> true end.
