-file("tests/ui/src/generics/witness/source_planning.pt",
      1).

-module('Elixir.Patina.SourcePlanning').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'SourcePlanning::Middle::middle'/2,
         'SourcePlanning::Deep::deep'/2,
         late_source/0,
         early_source/0,
         loop_source/1,
         constructor_input/0,
         associated_source/0]).

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

'SourcePlanning::Middle::middle'(_witness_Middle@0_0,
                                 _self@1_0) ->
    _supertraits_Middle@2_0 = erlang:element(2,
                                             _witness_Middle@0_0),
    _witness_Value@3_0 = erlang:element(1,
                                        _supertraits_Middle@2_0),
    _vtable_Value@4_0 = erlang:element(1,
                                       _witness_Value@3_0),
    _fn_Value_value@5_0 = erlang:element(1,
                                         _vtable_Value@4_0),
    begin
        _fn_Value_value@5_0(_witness_Value@3_0, _self@1_0)
    end.

'SourcePlanning::Deep::deep'(_witness_Deep@0_0,
                             _self@1_0) ->
    _supertraits_Deep@2_0 = erlang:element(2,
                                           _witness_Deep@0_0),
    _witness_Middle@3_0 = erlang:element(1,
                                         _supertraits_Deep@2_0),
    _vtable_Middle@4_0 = erlang:element(1,
                                        _witness_Middle@3_0),
    _fn_Middle_middle@5_0 = erlang:element(1,
                                           _vtable_Middle@4_0),
    begin
        _fn_Middle_middle@5_0(_witness_Middle@3_0, _self@1_0)
    end.

late(_witnesses_where_late@0_0, _x@1_0) ->
    _witness_Direct@2_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::$patina_witness'(),
    _supertraits_Direct@3_0 = erlang:element(2,
                                             _witness_Direct@2_0),
    _witness_Value@4_0 = erlang:element(1,
                                        _supertraits_Direct@3_0),
    _vtable_Value@5_0 = erlang:element(1,
                                       _witness_Value@4_0),
    _fn_Value_value@6_0 = erlang:element(1,
                                         _vtable_Value@5_0),
    begin
        _first@7_0 = _fn_Value_value@6_0(_witness_Value@4_0,
                                         _x@1_0),
        _direct@8_0 =
            'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::direct'(_witness_Direct@2_0,
                                                                                    _x@1_0),
        begin
            _element@9_0 = _fn_Value_value@6_0(_witness_Value@4_0,
                                               _x@1_0),
            {_first@7_0, _direct@8_0, _element@9_0}
        end
    end.

early(_witnesses_where_early@0_0, _x@1_0) ->
    _witness_Direct@2_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::$patina_witness'(),
    _supertraits_Direct@3_0 = erlang:element(2,
                                             _witness_Direct@2_0),
    _witness_Value@4_0 = erlang:element(1,
                                        _supertraits_Direct@3_0),
    _vtable_Value@5_0 = erlang:element(1,
                                       _witness_Value@4_0),
    _fn_Value_value@6_0 = erlang:element(1,
                                         _vtable_Value@5_0),
    begin
        _direct@7_0 =
            'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::direct'(_witness_Direct@2_0,
                                                                                    _x@1_0),
        begin
            _element@8_0 = _fn_Value_value@6_0(_witness_Value@4_0,
                                               _x@1_0),
            _element@9_0 = _fn_Value_value@6_0(_witness_Value@4_0,
                                               _x@1_0),
            {_element@8_0, _direct@7_0, _element@9_0}
        end
    end.

'loop_late$patina_loop_1'(_i@9_0, _n@2_0, _total@8_0,
                          _fn_Value_value@7_0, _witness_Value@5_0, _x@1_0) ->
    _ = begin
            case _i@9_0 < _n@2_0 of
                true ->
                    _total@8_1 = _total@8_0 +
                                     _fn_Value_value@7_0(_witness_Value@5_0,
                                                         _x@1_0),
                    _i@9_1 = _i@9_0 + 1,
                    {};
                _ ->
                    _branch_value@10_0 = begin
                                             begin
                                                 '$patina_marker':break({break,
                                                                         0,
                                                                         {},
                                                                         _i@9_0,
                                                                         _total@8_0}),
                                                 []
                                             end
                                         end,
                    _total@8_1 = _total@8_0,
                    _i@9_1 = _i@9_0,
                    _branch_value@10_0
            end
        end,
    begin
        '$patina_marker':continue(_i@9_1,
                                  _n@2_0,
                                  _total@8_1,
                                  _fn_Value_value@7_0,
                                  _witness_Value@5_0,
                                  _x@1_0),
        []
    end.

loop_late(_witnesses_where_loop_late@0_0, _x@1_0,
          _n@2_0) ->
    _witness_Direct@3_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::$patina_witness'(),
    _supertraits_Direct@4_0 = erlang:element(2,
                                             _witness_Direct@3_0),
    _witness_Value@5_0 = erlang:element(1,
                                        _supertraits_Direct@4_0),
    _vtable_Value@6_0 = erlang:element(1,
                                       _witness_Value@5_0),
    _fn_Value_value@7_0 = erlang:element(1,
                                         _vtable_Value@6_0),
    begin
        _total@8_0 = 0,
        _i@9_0 = 0,
        case 'loop_late$patina_loop_1'(_i@9_0,
                                       _n@2_0,
                                       _total@8_0,
                                       _fn_Value_value@7_0,
                                       _witness_Value@5_0,
                                       _x@1_0)
            of
            {break, 0, _loop_value@11_0, _i@9_1, _total@8_1} ->
                _loop_value@11_0
        end,
        _total@8_1 +
            'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Direct::direct'(_witness_Direct@3_0,
                                                                                    _x@1_0)
    end.

bounded(_witnesses_where_bounded@0_0, _x@1_0) ->
    _witness_Deep@2_0 = erlang:element(1,
                                       _witnesses_where_bounded@0_0),
    _supertraits_Deep@3_0 = erlang:element(2,
                                           _witness_Deep@2_0),
    _witness_Middle@4_0 = erlang:element(1,
                                         _supertraits_Deep@3_0),
    _supertraits_Middle@5_0 = erlang:element(2,
                                             _witness_Middle@4_0),
    _witness_Value@6_0 = erlang:element(1,
                                        _supertraits_Middle@5_0),
    _vtable_Value@7_0 = erlang:element(1,
                                       _witness_Value@6_0),
    _fn_Value_value@8_0 = erlang:element(1,
                                         _vtable_Value@7_0),
    _witnesses_where_Direct@9_0 = {_witness_Value@6_0},
    _witness_Direct@10_0 =
        'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Direct::$patina_witness'(_witnesses_where_Direct@9_0),
    begin
        begin
            _element@11_0 = _fn_Value_value@8_0(_witness_Value@6_0,
                                                _x@1_0),
            _element@12_0 =
                'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Direct::direct'(_witness_Direct@10_0,
                                                                                        _x@1_0),
            {_element@11_0, _element@12_0}
        end
    end.

associated(_witnesses_where_associated@0_0, _f@1_0) ->
    _witness_Factory@2_0 = erlang:element(1,
                                          _witnesses_where_associated@0_0),
    _vtable_Factory@3_0 = erlang:element(1,
                                         _witness_Factory@2_0),
    _fn_Factory_item@4_0 = erlang:element(2,
                                          _vtable_Factory@3_0),
    _type_Factory_Item@5_0 = erlang:element(1,
                                            _vtable_Factory@3_0),
    _where_Factory@6_0 =
        _type_Factory_Item@5_0(_witness_Factory@2_0),
    _witness_Value@7_0 = erlang:element(2,
                                        _where_Factory@6_0),
    _vtable_Value@8_0 = erlang:element(1,
                                       _witness_Value@7_0),
    _fn_Value_value@9_0 = erlang:element(1,
                                         _vtable_Value@8_0),
    begin
        _fn_Value_value@9_0(_witness_Value@7_0,
                            _fn_Factory_item@4_0(_witness_Factory@2_0, _f@1_0))
            +
            _fn_Value_value@9_0(_witness_Value@7_0,
                                _fn_Factory_item@4_0(_witness_Factory@2_0,
                                                     _f@1_0))
    end.

late_source() ->
    _witness_Deep@0_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Deep::$patina_witness<int>'(),
    _witnesses_where_late@1_0 = {_witness_Deep@0_0},
    begin late(_witnesses_where_late@1_0, 0) end.

early_source() ->
    _witness_Deep@0_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Deep::$patina_witness<int>'(),
    _witnesses_where_early@1_0 = {_witness_Deep@0_0},
    begin early(_witnesses_where_early@1_0, 0) end.

loop_source(_n@0_0) ->
    _witness_Deep@1_0 =
        'Elixir.Patina.SourcePlanning.Wrapped':'SourcePlanning::Deep::$patina_witness<int>'(),
    _witnesses_where_loop_late@2_0 = {_witness_Deep@1_0},
    begin
        loop_late(_witnesses_where_loop_late@2_0, 0, _n@0_0)
    end.

constructor_input() ->
    _witness_Deep@0_0 =
        'Elixir.Patina.SourcePlanning.Bounded':'SourcePlanning::Deep::$patina_witness<int>'(),
    _witnesses_where_bounded@1_0 = {_witness_Deep@0_0},
    begin bounded(_witnesses_where_bounded@1_0, 0) end.

associated_source() ->
    _witness_Factory@0_0 =
        'Elixir.Patina.SourcePlanning.Maker':'SourcePlanning::Factory::$patina_witness'(),
    _witnesses_where_associated@1_0 =
        {_witness_Factory@0_0},
    begin
        associated(_witnesses_where_associated@1_0,
                   'Elixir.Patina.SourcePlanning.Maker')
    end.
