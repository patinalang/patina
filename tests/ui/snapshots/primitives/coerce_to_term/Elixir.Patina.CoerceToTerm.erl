-file("tests/ui/src/primitives/coerce_to_term.pt", 1).

-module('Elixir.Patina.CoerceToTerm').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         coerce/1,
         coerce_map_value/1,
         coerce_map_key/1,
         update_map/1,
         coerce_slice/1,
         coerce_tuple/1,
         coerce_nested_map/1]).

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

coerce(_base@0_0) -> _base@0_0.

coerce_map_value(_base@0_0) -> _base@0_0.

coerce_map_key(_base@0_0) -> _base@0_0.

update_map(_base@0_0) ->
    _field_base@1_0 = _base@0_0,
    _field_base@1_0#{<<"enabled"/utf8>> => true}.

coerce_slice(_values@0_0) -> _values@0_0.

coerce_tuple(_value@0_0) -> _value@0_0.

coerce_nested_map(_base@0_0) -> _base@0_0.
