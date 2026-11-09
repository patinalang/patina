-file("tests/ui/src/generics/witness/bound_generic.pt",
      1).

-module('Elixir.Patina.BoundGeneric').

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

call(_witnesses_where_call@0_0, _h@1_0) ->
    _witness_Bound@2_0 = erlang:element(1,
                                        _witnesses_where_call@0_0),
    _witnesses_where_Tr@3_0 = {_witness_Bound@2_0},
    _witness_Tr@4_0 =
        'Elixir.Patina.BoundGeneric.Holder':'bound_generic::Tr::$patina_witness'(_witnesses_where_Tr@3_0),
    'Elixir.Patina.BoundGeneric.Holder':'bound_generic::Tr::method'(_witness_Tr@4_0,
                                                                    _h@1_0).

main() ->
    _witness_Bound@0_0 =
        'Elixir.Patina.BoundGeneric.Number':'bound_generic::Bound::$patina_witness'(),
    _witnesses_where_call@1_0 = {_witness_Bound@0_0},
    _h@3_0 = begin _value@2_0 = 5, _value@2_0 end,
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [call(_witnesses_where_call@1_0,
                                                                           _h@3_0)]),
                                        [_element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    {}.
