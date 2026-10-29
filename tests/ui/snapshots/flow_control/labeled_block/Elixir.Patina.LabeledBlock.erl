-file("tests/ui/src/flow_control/labeled_block.pt", 3).

-module('Elixir.Patina.LabeledBlock').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, labeled/1, nested/1]).

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

labeled(_x@0_0) ->
    _y@1_0 = 0,
    _r@5_0 = case [] of
                 {'$patina_label_0', _jump_value@3_0, _y@1_1} ->
                     _branch_value@4_0 = _jump_value@3_0,
                     _y@1_2 = _y@1_1,
                     _branch_value@4_0;
                 _ ->
                     _y@1_1 = _y@1_0 + 1,
                     case _x@0_0 > 5 of
                         true ->
                             _jump_value@2_0 = _x@0_0 * 2,
                             '$patina_marker':jump('$patina_label_0',
                                                   _jump_value@2_0,
                                                   _y@1_1),
                             [];
                         _ -> {}
                     end,
                     _y@1_2 = _y@1_1 + 10,
                     _x@0_0 + _y@1_2
             end,
    _r@5_0 + _y@1_2.

nested(_x@0_0) ->
    _n@1_0 = 0,
    case [] of
        {'$patina_label_0', _n@1_1} -> {};
        _ ->
            _branch_value@2_0 = fun _$patina_loop(_n@1_0) ->
                                        _n@1_1 = _n@1_0 + 1,
                                        case _n@1_1 > _x@0_0 of
                                            true ->
                                                '$patina_marker':jump('$patina_label_0',
                                                                      _n@1_1),
                                                [];
                                            _ -> {}
                                        end,
                                        _$patina_loop(_n@1_1)
                                end(_n@1_0),
            _n@1_1 = _n@1_0,
            _branch_value@2_0
    end,
    _n@1_1.
