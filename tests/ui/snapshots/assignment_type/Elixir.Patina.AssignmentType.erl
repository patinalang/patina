-file("tests/ui/src/assignment_type.pt", 1).

-module('Elixir.Patina.AssignmentType').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         assign/0,
         assign_op/0,
         chained_assign/0,
         assign_as_argument/0,
         assign_as_match_scrutinee/0,
         destructuring_assign/0,
         destructuring_assign_value/0,
         swap/0,
         nested/1,
         struct_fields/0,
         assign_coerces_to_var_type/0,
         fibonacci/1,
         discard/0,
         parenthesized_struct_in_scrutinee/0,
         assign_in_boolean_context/0,
         empty_block/0,
         block_ending_in_let/0,
         block_ending_in_uninitialized_let/0,
         block_ending_in_let_else/1,
         block_ending_in_unit_expr/0,
         block_ending_in_assignment/0,
         block_ending_in_destructuring_assignment/0]).

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

destructuring_assign() ->
    begin _first@0_0 = {} end,
    begin _second@1_0 = {} end,
    {_first@0_1, _second@1_1} = {40, 2},
    _first@0_1 + _second@1_1.

destructuring_assign_value() ->
    begin _first@0_0 = {} end,
    begin _second@1_0 = {} end,
    {_first@0_1, _second@1_1} = {40, 2}.

swap() ->
    _first@0_0 = 1,
    _second@1_0 = 2,
    {_first@0_1, _second@1_1} = {_second@1_0, _first@0_0},
    {_first@0_1, _second@1_1}.

nested(_value@0_0) ->
    begin _first@1_0 = {} end,
    begin _second@2_0 = {} end,
    begin _third@3_0 = {} end,
    {{_first@1_1, _second@2_1}, {_third@3_1, _}} =
        _value@0_0,
    _first@1_1 + _second@2_1 + _third@3_1.

struct_fields() ->
    begin _x@0_0 = {} end,
    begin _y@1_0 = {} end,
    #'Elixir.Patina.AssignmentType.Point':'Point'{x =
                                                      _x@0_1,
                                                  y = _y@1_1} =
        'Elixir.Patina.AssignmentType.Point':'$patina_constructor__Point'(40,
                                                                          2),
    _x@0_1 + _y@1_1.

assign_coerces_to_var_type() ->
    begin _first@0_0 = {} end,
    begin _second@1_0 = {} end,
    {_first@0_1, _second@1_1} = {40, 2},
    {_first@0_1, _second@1_1}.

fibonacci(_n@0_0) ->
    _current@1_0 = 0,
    _next@2_0 = 1,
    _i@3_0 = 0,
    case fun _$patina_loop(_i@3_0, _current@1_0,
                           _next@2_0) ->
                 _ = begin
                         case _i@3_0 < _n@0_0 of
                             true ->
                                 {_current@1_1, _next@2_1} = begin
                                                                 _element@4_0 =
                                                                     _current@1_0
                                                                         +
                                                                         _next@2_0,
                                                                 {_next@2_0,
                                                                  _element@4_0}
                                                             end,
                                 _i@3_1 = _i@3_0 + 1,
                                 {};
                             _ ->
                                 _branch_value@5_0 = begin
                                                         begin
                                                             '$patina_marker':break({break,
                                                                                     0,
                                                                                     {},
                                                                                     _i@3_0,
                                                                                     _current@1_0,
                                                                                     _next@2_0}),
                                                             []
                                                         end
                                                     end,
                                 _current@1_1 = _current@1_0,
                                 _next@2_1 = _next@2_0,
                                 _i@3_1 = _i@3_0,
                                 _branch_value@5_0
                         end
                     end,
                 _$patina_loop(_i@3_1, _current@1_1, _next@2_1)
         end(_i@3_0, _current@1_0, _next@2_0)
        of
        {break,
         0,
         _loop_value@6_0,
         _i@3_1,
         _current@1_1,
         _next@2_1} ->
            _loop_value@6_0
    end,
    _current@1_1.

discard() -> _ = 42.

parenthesized_struct_in_scrutinee() ->
    begin _x@0_0 = {} end,
    begin _y@1_0 = {} end,
    case #'Elixir.Patina.AssignmentType.Point':'Point'{x =
                                                           _x@0_1,
                                                       y = _y@1_1} =
             'Elixir.Patina.AssignmentType.Point':'$patina_constructor__Point'(1,
                                                                               2)
        of
        #'Elixir.Patina.AssignmentType.Point':'Point'{x = 1,
                                                      y = _} ->
            _x@0_1 + _y@1_1;
        _ -> 0
    end.

assign_in_boolean_context() ->
    _value@0_0 = false,
    case _value@0_1 = true of
        true -> _value@0_1;
        _ -> false
    end.

empty_block() -> {}.

block_ending_in_let() ->
    __value@0_0 = 42,
    {}.

block_ending_in_uninitialized_let() ->
    begin __value@0_0 = {} end.

block_ending_in_let_else(_value@0_0) ->
    case _value@0_0 of
        42 -> {};
        _ -> begin '$patina_marker':return({}), [] end
    end.

block_ending_in_unit_expr() -> {}.

block_ending_in_assignment() ->
    _value@0_0 = 0,
    _value@0_1 = 42,
    {}.

block_ending_in_destructuring_assignment() ->
    begin __first@0_0 = {} end,
    {__first@0_1, _} = {1, 2},
    {}.
