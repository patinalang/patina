-file("tests/ui/src/primitives/maps_named_const.pt", 1).

-module('Elixir.Patina.MapsNamedConst').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, const_key/1, named_const/1]).

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

const_key(_map@0_0) ->
    case _map@0_0 of
        #{<<110, 97, 109, 101>> := _value@1_0} -> _value@1_0;
        _ -> 0
    end.

named_const(_map@0_0) ->
    case _map@0_0 of
        #{<<110, 97, 109, 101>> := _value@1_0} ->
            #{<<"hi">> => _value@1_0};
        _ -> #{<<"name">> => 1}
    end.
