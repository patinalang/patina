-file("tests/ui/src/generics/witness/duplicate_preco"
      "nditions.pt",
      1).

-module('Elixir.Patina.DuplicatePreconditions').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, duplicate_where_clauses/0]).

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

use_foo(_witnesses_where_use_foo@0_0, _f@1_0) ->
    _witness_Foo@2_0 = erlang:element(1,
                                      _witnesses_where_use_foo@0_0),
    _vtable_Foo@3_0 = erlang:element(1, _witness_Foo@2_0),
    _fn_Foo_foo@4_0 = erlang:element(1, _vtable_Foo@3_0),
    begin _fn_Foo_foo@4_0(_witness_Foo@2_0, _f@1_0) end.

bar_from_left_or_right(_witnesses_where_bar_from_left_or_right@0_0,
                       _a@1_0, _b@2_0) ->
    _witness_Right@3_0 = erlang:element(2,
                                        _witnesses_where_bar_from_left_or_right@0_0),
    _supertraits_Right@4_0 = erlang:element(2,
                                            _witness_Right@3_0),
    _witness_Bar@5_0 = erlang:element(1,
                                      _supertraits_Right@4_0),
    _witnesses_where_Foo@6_0 = {_witness_Bar@5_0,
                                _witness_Bar@5_0},
    _witness_Foo@7_0 =
        'Elixir.Patina.DuplicatePreconditions.$impl.Core.Tuple':'DuplicatePreconditions::Foo::$patina_witness<(_, _)>'(_witnesses_where_Foo@6_0),
    _witnesses_where_use_foo@8_0 = {_witness_Foo@7_0},
    begin
        use_foo(_witnesses_where_use_foo@8_0, {_a@1_0, _b@2_0})
    end.

duplicate_where_clauses() ->
    _witness_Left@0_0 =
        'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'DuplicatePreconditions::Left::$patina_witness'(),
    _witness_Right@1_0 =
        'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'DuplicatePreconditions::Right::$patina_witness'(),
    _witnesses_where_bar_from_left_or_right@2_0 =
        {_witness_Left@0_0, _witness_Right@1_0},
    begin
        bar_from_left_or_right(_witnesses_where_bar_from_left_or_right@2_0,
                               30,
                               3)
    end.
