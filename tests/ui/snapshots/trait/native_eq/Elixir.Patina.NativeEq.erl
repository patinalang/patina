-file("tests/ui/src/trait/native_eq.pt", 1).

-module('Elixir.Patina.NativeEq').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         eq_by_parity_eq/2,
         eq_by_parity_terms_eq/2,
         int_lists_eq/2,
         tuples_eq/2,
         maps_eq/2,
         int_options_eq/2,
         hand_written_native_eq_lists_eq/2,
         eq_by_parity_lists_eq/2,
         eq_by_parity_tuples_eq/2,
         eq_by_parity_maps_eq/2,
         eq_by_parity_options_eq/2,
         int_lists_eq_through_witness/2,
         eq_by_parity_lists_eq_through_witness/2,
         eq_by_parity_lists_eq_in_generic_fn/2]).

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

eq_by_parity_eq(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::eq'(_witness_PartialEq@2_0,
                                                              _a@0_0,
                                                              _b@1_0).

eq_by_parity_terms_eq(_a@0_0, _b@1_0) ->
    _a@0_0 =:= _b@1_0.

int_lists_eq(_a@0_0, _b@1_0) ->
    _self@2_0 = _a@0_0,
    _other@3_0 = _b@1_0,
    _self@2_0 =:= _other@3_0.

tuples_eq(_a@0_0, _b@1_0) ->
    _self@2_0 = _a@0_0,
    _other@3_0 = _b@1_0,
    _self@2_0 =:= _other@3_0.

maps_eq(_a@0_0, _b@1_0) ->
    _self@2_0 = _a@0_0,
    _other@3_0 = _b@1_0,
    _self@2_0 =:= _other@3_0.

int_options_eq(_a@0_0, _b@1_0) ->
    _self@2_0 = _a@0_0,
    _other@3_0 = _b@1_0,
    _self@2_0 =:= _other@3_0.

hand_written_native_eq_lists_eq(_a@0_0, _b@1_0) ->
    _self@3_0 = begin
                    _element@2_0 =
                        'Elixir.Patina.NativeEq.HandWrittenNativeEq':'$patina_constructor__HandWrittenNativeEq'(_a@0_0,
                                                                                                                0),
                    [_element@2_0]
                end,
    _other@5_0 = begin
                     _element@4_0 =
                         'Elixir.Patina.NativeEq.HandWrittenNativeEq':'$patina_constructor__HandWrittenNativeEq'(_b@1_0,
                                                                                                                 0),
                     [_element@4_0]
                 end,
    _self@3_0 =:= _other@5_0.

eq_by_parity_lists_eq(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _witnesses_where_PartialEq@3_0 =
        {_witness_PartialEq@2_0},
    _witness_PartialEq@4_0 =
        'Elixir.Patina.Core.List':'core::PartialEq::$patina_witness'(_witnesses_where_PartialEq@3_0),
    'Elixir.Patina.Core.List':'core::PartialEq::eq'(_witness_PartialEq@4_0,
                                                    begin
                                                        _element@5_0 = _a@0_0,
                                                        [_element@5_0]
                                                    end,
                                                    begin
                                                        _element@6_0 = _b@1_0,
                                                        [_element@6_0]
                                                    end).

eq_by_parity_tuples_eq(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Int':'core::PartialEq::$patina_witness'(),
    _witness_PartialEq@3_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _witnesses_where_PartialEq@4_0 =
        {_witness_PartialEq@2_0, _witness_PartialEq@3_0},
    _witness_PartialEq@5_0 =
        'Elixir.Patina.Core.Tuple':'core::PartialEq::$patina_witness<(_, _)>'(_witnesses_where_PartialEq@4_0),
    'Elixir.Patina.Core.Tuple':'core::PartialEq::eq<(_, _)>'(_witness_PartialEq@5_0,
                                                             begin
                                                                 _element@6_0 =
                                                                     _a@0_0,
                                                                 {0,
                                                                  _element@6_0}
                                                             end,
                                                             begin
                                                                 _element@7_0 =
                                                                     _b@1_0,
                                                                 {0,
                                                                  _element@7_0}
                                                             end).

eq_by_parity_maps_eq(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _witnesses_where_PartialEq@3_0 =
        {_witness_PartialEq@2_0},
    _witness_PartialEq@4_0 =
        'Elixir.Patina.Core.Map':'core::PartialEq::$patina_witness'(_witnesses_where_PartialEq@3_0),
    'Elixir.Patina.Core.Map':'core::PartialEq::eq'(_witness_PartialEq@4_0,
                                                   begin
                                                       _value@5_0 = _a@0_0,
                                                       #{k => _value@5_0}
                                                   end,
                                                   begin
                                                       _value@6_0 = _b@1_0,
                                                       #{k => _value@6_0}
                                                   end).

eq_by_parity_options_eq(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _self@4_0 = begin _0@3_0 = _a@0_0, _0@3_0 end,
    _other@6_0 = begin _0@5_0 = _b@1_0, _0@5_0 end,
    case {_self@4_0, _other@6_0} of
        {_value@8_0 = _left@7_0, _value@10_0 = _right@9_0}
            when erlang:is_integer(_value@8_0),
                 erlang:is_integer(_value@10_0) ->
            'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::eq'(_witness_PartialEq@2_0,
                                                                      _left@7_0,
                                                                      _right@9_0);
        {nil, nil} -> true;
        _ -> false
    end.

generic_eq(_witnesses_where_generic_eq@0_0, _a@1_0,
           _b@2_0) ->
    _witness_PartialEq@3_0 = erlang:element(1,
                                            _witnesses_where_generic_eq@0_0),
    _vtable_PartialEq@4_0 = erlang:element(1,
                                           _witness_PartialEq@3_0),
    _fn_PartialEq_eq@5_0 = erlang:element(1,
                                          _vtable_PartialEq@4_0),
    _fn_PartialEq_eq@5_0(_witness_PartialEq@3_0,
                         _a@1_0,
                         _b@2_0).

int_lists_eq_through_witness(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Int':'core::PartialEq::$patina_witness'(),
    _witnesses_where_PartialEq@3_0 =
        {_witness_PartialEq@2_0},
    _witness_PartialEq@4_0 =
        'Elixir.Patina.Core.List':'core::PartialEq::$patina_witness<core::list_impl::{impl#5}>'(_witnesses_where_PartialEq@3_0),
    _witnesses_where_generic_eq@5_0 =
        {_witness_PartialEq@4_0},
    generic_eq(_witnesses_where_generic_eq@5_0,
               _a@0_0,
               _b@1_0).

eq_by_parity_lists_eq_through_witness(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _witnesses_where_PartialEq@3_0 =
        {_witness_PartialEq@2_0},
    _witness_PartialEq@4_0 =
        'Elixir.Patina.Core.List':'core::PartialEq::$patina_witness'(_witnesses_where_PartialEq@3_0),
    _witnesses_where_generic_eq@5_0 =
        {_witness_PartialEq@4_0},
    generic_eq(_witnesses_where_generic_eq@5_0,
               begin _element@6_0 = _a@0_0, [_element@6_0] end,
               begin _element@7_0 = _b@1_0, [_element@7_0] end).

generic_lists_eq(_witnesses_where_generic_lists_eq@0_0,
                 _a@1_0, _b@2_0) ->
    _witness_PartialEq@3_0 = erlang:element(1,
                                            _witnesses_where_generic_lists_eq@0_0),
    _witnesses_where_PartialEq@4_0 =
        {_witness_PartialEq@3_0},
    _witness_PartialEq@5_0 =
        'Elixir.Patina.Core.List':'core::PartialEq::$patina_witness'(_witnesses_where_PartialEq@4_0),
    _vtable_PartialEq@6_0 = erlang:element(1,
                                           _witness_PartialEq@5_0),
    _fn_PartialEq_eq@7_0 = erlang:element(1,
                                          _vtable_PartialEq@6_0),
    _fn_PartialEq_eq@7_0(_witness_PartialEq@5_0,
                         _a@1_0,
                         _b@2_0).

eq_by_parity_lists_eq_in_generic_fn(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.NativeEq.EqByParity':'core::PartialEq::$patina_witness'(),
    _witnesses_where_generic_lists_eq@3_0 =
        {_witness_PartialEq@2_0},
    generic_lists_eq(_witnesses_where_generic_lists_eq@3_0,
                     begin _element@4_0 = _a@0_0, [_element@4_0] end,
                     begin _element@5_0 = _b@1_0, [_element@5_0] end).
