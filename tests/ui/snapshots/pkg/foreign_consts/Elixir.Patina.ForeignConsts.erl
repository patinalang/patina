-file("tests/ui/src/pkg/foreign_consts.pt", 1).

-module('Elixir.Patina.ForeignConsts').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         pair/0,
         local_pairs/0,
         header/0,
         origin_y/0,
         computed_unit_x/0,
         foreign_doubled_of_local_impl/0,
         generic_empty/0,
         is_pair/1,
         is_header/1]).

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

pair() -> {[1, 2, 3], #{<<"answer"/utf8>> => 42}}.

local_pairs() ->
    [{[1, 2, 3], #{<<"answer"/utf8>> => 42}},
     {[1, 2, 3], #{<<"answer"/utf8>> => 42}}].

header() -> <<80, 84, 2:16/integer>>.

origin_y() ->
    _const_ORIGIN@0_0 =
        'Elixir.Patina.Consts.Point':'$patina_constructor__Point'(0,
                                                                  0),
    _const_ORIGIN@0_0#'Elixir.Patina.Consts.Point':'Point'.y.

computed_unit_x() ->
    _const_COMPUTED_UNIT@1_0 = begin
                                   _x@0_0 = 2 - 1,
                                   'Elixir.Patina.Consts.Point':'$patina_constructor__Point'(_x@0_0,
                                                                                             1)
                               end,
    _const_COMPUTED_UNIT@1_0#'Elixir.Patina.Consts.Point':'Point'.x.

foreign_doubled_of_local_impl() ->
    _witness_HasValue@0_0 =
        'Elixir.Patina.ForeignConsts.Three':'consts::HasValue::$patina_witness'(),
    _const_HasValue_VALUE@1_0 =
        'Elixir.Patina.ForeignConsts.Three':'consts::HasValue::$patina_const::VALUE'(_witness_HasValue@0_0),
    _const_DOUBLED@2_0 = _const_HasValue_VALUE@1_0 * 2,
    _const_DOUBLED@2_0.

generic_empty() -> [].

is_pair(_value@0_0) ->
    _const_PAIR@1_0 = {[1, 2, 3],
                       #{<<"answer"/utf8>> => 42}},
    case _value@0_0 of
        _const_PAIR@1_0 -> true;
        _ -> false
    end.

is_header(_value@0_0) ->
    _const_HEADER@1_0 = <<80, 84, 2:16/integer>>,
    case _value@0_0 of
        _const_HEADER@1_0 -> true;
        _ -> false
    end.
