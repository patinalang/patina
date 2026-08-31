-file("tests/ui/src/assignment_type.pt", 1).

-module('Elixir.Patina.AssignmentType').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         assign/0,
         assign_op/0,
         chained_assign/0,
         assign_as_argument/0,
         assign_as_match_scrutinee/0,
         assign_in_boolean_context/0,
         empty_block/0,
         block_ending_in_let/0,
         block_ending_in_uninitialized_let/0,
         block_ending_in_let_else/1,
         block_ending_in_unit_expr/0,
         block_ending_in_assignment/0]).

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

assign() ->
    _value@0_0 = 0,
    _value@0_1 = 42.

assign_op() ->
    _value@0_0 = 40,
    _value@0_1 = _value@0_0 + 2.

chained_assign() ->
    _first@0_0 = 0,
    _second@1_0 = 0,
    _first@0_1 = _second@1_1 = 21.

identity(_value@0_0) -> _value@0_0.

assign_as_argument() ->
    _value@0_0 = 0,
    identity(_value@0_1 = 42).

assign_as_match_scrutinee() ->
    _value@0_0 = 0,
    case _value@0_1 = 42 of
        42 -> _value@0_1;
        _ -> 0
    end.

assign_in_boolean_context() ->
    _value@0_0 = false,
    case _value@0_1 = true of
        true -> _value@0_1;
        false -> false
    end.

empty_block() -> {}.

block_ending_in_let() ->
    __value@0_0 = 42,
    {}.

block_ending_in_uninitialized_let() -> {}.

block_ending_in_let_else(_value@0_0) ->
    case _value@0_0 of
        42 -> {};
        _ ->
            '$patina_marker':return({}),
            {}
    end.

block_ending_in_unit_expr() -> {}.

block_ending_in_assignment() ->
    _value@0_0 = 0,
    _value@0_1 = 42,
    {}.
