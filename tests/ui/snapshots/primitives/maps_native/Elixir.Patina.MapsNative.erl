-file("tests/ui/src/primitives/maps_native.pt", 1).

-module('Elixir.Patina.MapsNative').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         create/0,
         empty/0,
         param_and_return/1,
         infer_empty_by_usage/0,
         infer_from_base/1,
         update/1,
         update_identity/1,
         duplicate/0]).

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

create() -> #{<<"a"/utf8>> => 1, <<"b"/utf8>> => 2}.

empty() -> #{}.

param_and_return(_map@0_0) -> _map@0_0.

erase_map_type(_map@0_0) -> _map@0_0.

infer_empty_by_usage() ->
    _map@0_0 = #{},
    erase_map_type(_map@0_0).

infer_from_base(_base@0_0) -> _base@0_0#{}.

update(_base@0_0) ->
    _base@0_0#{<<"a"/utf8>> => 10, <<"c"/utf8>> => 3}.

update_identity(_base@0_0) -> _base@0_0#{}.

duplicate() -> #{<<"a"/utf8>> => 1, <<"a"/utf8>> => 2}.
