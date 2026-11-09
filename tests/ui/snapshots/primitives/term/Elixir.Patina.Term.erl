-file("tests/ui/src/primitives/term.pt", 3).

-module('Elixir.Patina.Term').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         equal/2,
         not_equal/2,
         partial_eq_equal/2]).

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

equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

not_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =/= _right@1_0.

generic_equal(_witnesses_where_generic_equal@0_0,
              _left@1_0, _right@2_0) ->
    _witness_PartialEq@3_0 = erlang:element(1,
                                            _witnesses_where_generic_equal@0_0),
    _vtable_PartialEq@4_0 = erlang:element(1,
                                           _witness_PartialEq@3_0),
    _fn_PartialEq_eq@5_0 = erlang:element(1,
                                          _vtable_PartialEq@4_0),
    _fn_PartialEq_eq@5_0(_witness_PartialEq@3_0,
                         _left@1_0,
                         _right@2_0).

partial_eq_equal(_left@0_0, _right@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Term':'core::PartialEq::$patina_witness'(),
    _witnesses_where_generic_equal@3_0 =
        {_witness_PartialEq@2_0},
    generic_equal(_witnesses_where_generic_equal@3_0,
                  _left@0_0,
                  _right@1_0).
