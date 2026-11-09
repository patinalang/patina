-file("tests/ui/src/generics/generic_consts.pt", 1).

-module('Elixir.Patina.GenericConsts').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         quadrupled/0,
         none/0,
         doubled/1,
         is_doubled/2,
         doubled_int/0,
         is_doubled_int/1]).

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

quadrupled() ->
    _witness_Konst@0_0 =
        'Elixir.Patina.GenericConsts.$impl.Core.Int':'generic_consts::Konst::$patina_witness'(),
    _const_Konst_K@1_0 =
        'Elixir.Patina.GenericConsts.$impl.Core.Int':'generic_consts::Konst::$patina_const::K'(_witness_Konst@0_0),
    _const_DOUBLED@2_0 = _const_Konst_K@1_0 * 2,
    _const_QUADRUPLED@3_0 = _const_DOUBLED@2_0 * 2,
    _const_QUADRUPLED@3_0.

none() -> [].

doubled(_witnesses_where_doubled@0_0) ->
    _witness_Konst@1_0 = erlang:element(1,
                                        _witnesses_where_doubled@0_0),
    _vtable_Konst@2_0 = erlang:element(1,
                                       _witness_Konst@1_0),
    _const_fn_Konst_K@3_0 = erlang:element(1,
                                           _vtable_Konst@2_0),
    _const_Konst_K@4_0 =
        _const_fn_Konst_K@3_0(_witness_Konst@1_0),
    _const_DOUBLED@5_0 = _const_Konst_K@4_0 * 2,
    _const_DOUBLED@5_0.

is_doubled(_witnesses_where_is_doubled@0_0, _n@1_0) ->
    _witness_Konst@2_0 = erlang:element(1,
                                        _witnesses_where_is_doubled@0_0),
    _vtable_Konst@3_0 = erlang:element(1,
                                       _witness_Konst@2_0),
    _const_fn_Konst_K@4_0 = erlang:element(1,
                                           _vtable_Konst@3_0),
    _const_Konst_K@5_0 =
        _const_fn_Konst_K@4_0(_witness_Konst@2_0),
    _const_DOUBLED@6_0 = _const_Konst_K@5_0 * 2,
    case _n@1_0 of
        _const_DOUBLED@6_0 -> true;
        _ -> false
    end.

doubled_int() ->
    _witness_Konst@0_0 =
        'Elixir.Patina.GenericConsts.$impl.Core.Int':'generic_consts::Konst::$patina_witness'(),
    _witnesses_where_doubled@1_0 = {_witness_Konst@0_0},
    doubled(_witnesses_where_doubled@1_0).

is_doubled_int(_n@0_0) ->
    _witness_Konst@1_0 =
        'Elixir.Patina.GenericConsts.$impl.Core.Int':'generic_consts::Konst::$patina_witness'(),
    _witnesses_where_is_doubled@2_0 = {_witness_Konst@1_0},
    is_doubled(_witnesses_where_is_doubled@2_0, _n@0_0).
