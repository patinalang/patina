-file("tests/ui/src/generics/witness/bound_generic.pt",
      3).

-module('Elixir.Patina.BoundGeneric.Holder').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BoundGeneric::Tr::method'/2,
         'BoundGeneric::Tr::$patina_witness'/1]).

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

'BoundGeneric::Tr::method'(_witness_Tr@0_0,
                           _self@1_0) ->
    _where_Tr@2_0 = erlang:element(3, _witness_Tr@0_0),
    _witness_Bound@3_0 = erlang:element(1, _where_Tr@2_0),
    _vtable_Bound@4_0 = erlang:element(1,
                                       _witness_Bound@3_0),
    _fn_Bound_b@5_0 = erlang:element(1, _vtable_Bound@4_0),
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_self@1_0]),
                                        [<<"Tr::method("/utf8>>,
                                         _element@6_0,
                                         <<")"/utf8>>]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    _fn_Bound_b@5_0(_witness_Bound@3_0, _self@1_0).

'BoundGeneric::Tr::$patina_witness'(_witnesses_where_Tr@0_0) ->
    {{fun 'Elixir.Patina.BoundGeneric.Holder':'BoundGeneric::Tr::method'/2},
     {},
     _witnesses_where_Tr@0_0}.
