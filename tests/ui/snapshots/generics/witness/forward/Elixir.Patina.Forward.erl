-file("tests/ui/src/generics/witness/forward.pt", 1).

-module('Elixir.Patina.Forward').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

b(_witnesses_where_b@0_0, _x@1_0) ->
    _witness_Tr@2_0 = erlang:element(1,
                                     _witnesses_where_b@0_0),
    _vtable_Tr@3_0 = erlang:element(1, _witness_Tr@2_0),
    _fn_Tr_method@4_0 = erlang:element(1, _vtable_Tr@3_0),
    _fn_Tr_method@4_0(_witness_Tr@2_0, _x@1_0).

a(_witnesses_where_a@0_0, _x@1_0) ->
    _witness_Tr@2_0 = erlang:element(1,
                                     _witnesses_where_a@0_0),
    _witnesses_where_b@3_0 = {_witness_Tr@2_0},
    b(_witnesses_where_b@3_0, _x@1_0).

main() ->
    _witness_Tr@0_0 =
        'Elixir.Patina.Forward.S':'Forward::Tr::$patina_witness'(),
    _witnesses_where_a@1_0 = {_witness_Tr@0_0},
    a(_witnesses_where_a@1_0, 'Elixir.Patina.Forward.S').
