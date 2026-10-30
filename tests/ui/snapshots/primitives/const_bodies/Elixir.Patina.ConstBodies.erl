-file("tests/ui/src/primitives/const_bodies.pt", 1).

-module('Elixir.Patina.ConstBodies').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         block_value/0,
         nested_block/0,
         arithmetic/0,
         two_uses_share_var/0,
         closure_binds_own_consts/0,
         is_list/1,
         is_pair/1,
         is_header/1,
         is_exactly_map/1,
         is_block_value/1,
         computed_key/1]).

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

block_value() ->
    _const_BLOCK_VALUE@1_0 = begin
                                 _value@0_0 = 1, _value@0_0
                             end,
    _const_BLOCK_VALUE@1_0.

nested_block() ->
    _const_NESTED_BLOCK@2_0 = begin
                                  _value@1_0 = begin
                                                   _value@0_0 = 1, _value@0_0
                                               end,
                                  #{<<"value"/utf8>> => _value@1_0}
                              end,
    _const_NESTED_BLOCK@2_0.

arithmetic() ->
    _const_ARITHMETIC@0_0 = <<(1 + 1):8/integer>>,
    _const_ARITHMETIC@0_0.

two_uses_share_var() ->
    _const_BLOCK_VALUE@1_0 = begin
                                 _value@0_0 = 1, _value@0_0
                             end,
    {_const_BLOCK_VALUE@1_0, _const_BLOCK_VALUE@1_0}.

closure_binds_own_consts() ->
    _const_BLOCK_VALUE@1_0 = begin
                                 _value@0_0 = 1, _value@0_0
                             end,
    _f@2_0 = fun () -> _const_BLOCK_VALUE@1_0 + 1 end,
    _f@2_0().

is_list(_list@0_0) ->
    _const_LIST@1_0 = [1, 2],
    case _list@0_0 of
        _const_LIST@1_0 -> true;
        _ -> false
    end.

is_pair(_pair@0_0) ->
    _const_PAIR@1_0 = {1, <<"name"/utf8>>},
    case _pair@0_0 of
        _const_PAIR@1_0 -> true;
        _ -> false
    end.

is_header(_header@0_0) ->
    _const_HEADER@1_0 = <<80, 84, 2:16/integer>>,
    case _header@0_0 of
        _const_HEADER@1_0 -> true;
        _ -> false
    end.

is_exactly_map(_map@0_0) ->
    _const_MAP@1_0 = #{<<"name"/utf8>> => 1},
    case _map@0_0 of
        _const_MAP@1_0 -> true;
        _ -> false
    end.

is_block_value(_n@0_0) ->
    _const_BLOCK_VALUE@2_0 = begin
                                 _value@1_0 = 1, _value@1_0
                             end,
    case _n@0_0 of
        _const_BLOCK_VALUE@2_0 -> true;
        _ -> false
    end.

computed_key(_map@0_0) ->
    _const_COMPUTED_KEY@2_0 = begin
                                  _key@1_0 = <<"name"/utf8>>, _key@1_0
                              end,
    case _map@0_0 of
        #{_const_COMPUTED_KEY@2_0 := _value@3_0} -> _value@3_0;
        _ -> 0
    end.
