-file("tests/ui/src/generics/witness/duplicate_preco"
      "nditions.pt",
      38).

-module('Elixir.Patina.DuplicatePreconditions.$impl.Core.Tuple').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'DuplicatePreconditions::Foo::foo<(_, _)>'/2,
         'DuplicatePreconditions::Foo::$patina_witness<(_, _)>'/1]).

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

'DuplicatePreconditions::Foo::foo<(_, _)>'(_witness_Foo@0_0,
                                           _self@1_0) ->
    _where_Foo@2_0 = erlang:element(3, _witness_Foo@0_0),
    _witness_Bar@3_0 = erlang:element(2, _where_Foo@2_0),
    _witness_Bar@4_0 = erlang:element(1, _where_Foo@2_0),
    _vtable_Bar@5_0 = erlang:element(1, _witness_Bar@3_0),
    _fn_Bar_bar@6_0 = erlang:element(1, _vtable_Bar@5_0),
    _vtable_Bar@7_0 = erlang:element(1, _witness_Bar@4_0),
    _fn_Bar_bar@8_0 = erlang:element(1, _vtable_Bar@7_0),
    begin
        {_a@9_0, _b@10_0} = _self@1_0,
        _fn_Bar_bar@8_0(_witness_Bar@4_0, _a@9_0) +
            _fn_Bar_bar@6_0(_witness_Bar@3_0, _b@10_0)
    end.

'DuplicatePreconditions::Foo::$patina_witness<(_, _)>'(_witnesses_where_Foo@0_0) ->
    {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Tuple':'DuplicatePreconditions::Foo::foo<(_, _)>'/2},
     {},
     _witnesses_where_Foo@0_0}.
