-file("tests/ui/src/generics/witness/gat_witness.pt",
      1).

-module('Elixir.Patina.GatWitness').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1]).

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

describe_member(_witnesses_where_describe_member@0_0,
                _family@1_0) ->
    _witness_Describe@2_0 = erlang:element(2,
                                           _witnesses_where_describe_member@0_0),
    _witness_Family@3_0 = erlang:element(1,
                                         _witnesses_where_describe_member@0_0),
    _vtable_Describe@4_0 = erlang:element(1,
                                          _witness_Describe@2_0),
    _fn_Describe_describe@5_0 = erlang:element(1,
                                               _vtable_Describe@4_0),
    _vtable_Family@6_0 = erlang:element(1,
                                        _witness_Family@3_0),
    _fn_Family_member@7_0 = erlang:element(2,
                                           _vtable_Family@6_0),
    begin
        _fn_Describe_describe@5_0(_witness_Describe@2_0,
                                  _fn_Family_member@7_0(_witness_Family@3_0,
                                                        _family@1_0,
                                                        1))
    end.
