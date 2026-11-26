-file("tests/ui/src/primitives/pid_port_ref.pt", 1).

-module('Elixir.Patina.PidPortRef').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         kind/1,
         pids_equal/2,
         pids_less_than/2,
         pids_less_than_equal/2,
         pids_equal_generic/2,
         pids_less_than_generic/2,
         ports_equal/2,
         ports_less_than/2,
         ports_equal_generic/2,
         ports_less_than_generic/2,
         refs_equal/2,
         refs_less_than/2,
         refs_equal_generic/2,
         refs_less_than_generic/2]).

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

generic_less_than(_witnesses_where_generic_less_than@0_0,
                  _left@1_0, _right@2_0) ->
    _witness_PartialOrd@3_0 = erlang:element(1,
                                             _witnesses_where_generic_less_than@0_0),
    _vtable_PartialOrd@4_0 = erlang:element(1,
                                            _witness_PartialOrd@3_0),
    _fn_PartialOrd_lt@5_0 = erlang:element(1,
                                           _vtable_PartialOrd@4_0),
    _fn_PartialOrd_lt@5_0(_witness_PartialOrd@3_0,
                          _left@1_0,
                          _right@2_0).

kind(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_pid(_value@1_0) -> pid;
        _value@2_0 when erlang:is_port(_value@2_0) -> port;
        _value@3_0 when erlang:is_reference(_value@3_0) -> ref;
        _ -> other
    end.

pids_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

pids_less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.

pids_less_than_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =< _right@1_0.

pids_equal_generic(_left@0_0, _right@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Pid':'core::PartialEq::$patina_witness'(),
    _witnesses_where_generic_equal@3_0 =
        {_witness_PartialEq@2_0},
    generic_equal(_witnesses_where_generic_equal@3_0,
                  _left@0_0,
                  _right@1_0).

pids_less_than_generic(_left@0_0, _right@1_0) ->
    _witness_PartialOrd@2_0 =
        'Elixir.Patina.Core.Pid':'core::PartialOrd::$patina_witness'(),
    _witnesses_where_generic_less_than@3_0 =
        {_witness_PartialOrd@2_0},
    generic_less_than(_witnesses_where_generic_less_than@3_0,
                      _left@0_0,
                      _right@1_0).

ports_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

ports_less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.

ports_equal_generic(_left@0_0, _right@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Port':'core::PartialEq::$patina_witness'(),
    _witnesses_where_generic_equal@3_0 =
        {_witness_PartialEq@2_0},
    generic_equal(_witnesses_where_generic_equal@3_0,
                  _left@0_0,
                  _right@1_0).

ports_less_than_generic(_left@0_0, _right@1_0) ->
    _witness_PartialOrd@2_0 =
        'Elixir.Patina.Core.Port':'core::PartialOrd::$patina_witness'(),
    _witnesses_where_generic_less_than@3_0 =
        {_witness_PartialOrd@2_0},
    generic_less_than(_witnesses_where_generic_less_than@3_0,
                      _left@0_0,
                      _right@1_0).

refs_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

refs_less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.

refs_equal_generic(_left@0_0, _right@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Ref':'core::PartialEq::$patina_witness'(),
    _witnesses_where_generic_equal@3_0 =
        {_witness_PartialEq@2_0},
    generic_equal(_witnesses_where_generic_equal@3_0,
                  _left@0_0,
                  _right@1_0).

refs_less_than_generic(_left@0_0, _right@1_0) ->
    _witness_PartialOrd@2_0 =
        'Elixir.Patina.Core.Ref':'core::PartialOrd::$patina_witness'(),
    _witnesses_where_generic_less_than@3_0 =
        {_witness_PartialOrd@2_0},
    generic_less_than(_witnesses_where_generic_less_than@3_0,
                      _left@0_0,
                      _right@1_0).
