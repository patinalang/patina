-file("tests/ui/src/flow_control/loops.pt", 257).

-module('Elixir.Patina.Loops.Counter').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '<impl Loops::Count for Loops::Counter<int>>::count$patina_loop_1'/3,
         '<impl Loops::Count for Loops::Counter<str>>::count$patina_loop_1'/3,
         'Loops::Count::count'/3]).

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

'<impl Loops::Count for Loops::Counter<int>>::count$patina_loop_1'(_witness@0_0,
                                                                   _n@3_0,
                                                                   _limit@2_0) ->
    _ = begin
            case _n@3_0 < _limit@2_0 of
                true ->
                    _n@3_1 = _n@3_0 + 1,
                    {};
                false ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@3_0})
                                        end,
                    _n@3_1 = _n@3_0,
                    _branch_value@4_0
            end
        end,
    '$patina_marker':continue(_witness@0_0,
                              _n@3_1,
                              _limit@2_0).

'<impl Loops::Count for Loops::Counter<str>>::count$patina_loop_1'(_witness@0_0,
                                                                   _n@3_0,
                                                                   _limit@2_0) ->
    _ = begin
            case _n@3_0 < _limit@2_0 of
                true ->
                    _n@3_1 = _n@3_0 + 2,
                    {};
                false ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _n@3_0})
                                        end,
                    _n@3_1 = _n@3_0,
                    _branch_value@4_0
            end
        end,
    '$patina_marker':continue(_witness@0_0,
                              _n@3_1,
                              _limit@2_0).

'Loops::Count::count'(_witness@0_0 =
                          {'impl Loops::Count for Loops::Counter<int>', _},
                      _self@1_0, _limit@2_0) ->
    _n@3_0 = 0,
    case
        '<impl Loops::Count for Loops::Counter<int>>::count$patina_loop_1'(_witness@0_0,
                                                                           _n@3_0,
                                                                           _limit@2_0)
        of
        {break, 0, _loop_value@5_0, _n@3_1} -> _loop_value@5_0
    end,
    _n@3_1;
'Loops::Count::count'(_witness@0_0 =
                          {'impl Loops::Count for Loops::Counter<str>', _},
                      _self@1_0, _limit@2_0) ->
    _n@3_0 = 0,
    case
        '<impl Loops::Count for Loops::Counter<str>>::count$patina_loop_1'(_witness@0_0,
                                                                           _n@3_0,
                                                                           _limit@2_0)
        of
        {break, 0, _loop_value@5_0, _n@3_1} -> _loop_value@5_0
    end,
    _n@3_1.
