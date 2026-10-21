-file("tests/ui/src/primitives/maps_patterns.pt", 1).

-module('Elixir.Patina.MapsPatterns').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         lookup_name/1,
         lookup_name_in_term_keyed_map/1,
         nested_lookup/1,
         is_map/1,
         lookup_constant_key/1,
         lookup_pinned_key/2,
         lookup_pinned_key_in_closure/2,
         lookup_assoc_const_key/1,
         lookup_generic_name_key/1,
         lookup_tuple_key/2,
         lookup_list_key/1,
         lookup_binary_key/1,
         lookup_exact_map_key/2,
         lookup_enum_key/2,
         lookup_option_key/1]).

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

lookup_name(_map@0_0) ->
    case _map@0_0 of
        #{<<"name"/utf8>> := _name@1_0} -> _name@1_0;
        _ -> -1
    end.

lookup_name_in_term_keyed_map(_map@0_0) ->
    case _map@0_0 of
        #{<<"name"/utf8>> := _name@1_0} -> _name@1_0;
        _ -> -1
    end.

nested_lookup(_map@0_0) ->
    case _map@0_0 of
        #{<<"outer"/utf8>> :=
              #{<<"inner"/utf8>> := _value@1_0}} ->
            _value@1_0;
        _ -> -1
    end.

is_map(_map@0_0) -> case _map@0_0 of #{} -> true end.

lookup_constant_key(_map@0_0) ->
    case _map@0_0 of
        #{<<"name"/utf8>> := _name@1_0} -> _name@1_0;
        _ -> -1
    end.

lookup_pinned_key(_map@0_0, _key@1_0) ->
    case _map@0_0 of
        #{_key@1_0 := _value@2_0} -> _value@2_0;
        _ -> -1
    end.

lookup_pinned_key_in_closure(_map@0_0, _key@1_0) ->
    _lookup@4_0 = fun (_map@2_0) ->
                          case _map@2_0 of
                              #{_key@1_0 := _value@3_0} -> _value@3_0;
                              _ -> -1
                          end
                  end,
    _lookup@4_0(_map@0_0).

lookup_assoc_const_key(_map@0_0) ->
    _witness_Keyed@1_0 =
        'Elixir.Patina.MapsPatterns.NameKey':'MapsPatterns::Keyed::$patina_witness'(),
    _const_Keyed_KEY@2_0 =
        'Elixir.Patina.MapsPatterns.NameKey':'MapsPatterns::Keyed::$patina_const::KEY'(_witness_Keyed@1_0),
    begin
        case _map@0_0 of
            #{_const_Keyed_KEY@2_0 := _value@3_0} -> _value@3_0;
            _ -> -1
        end
    end.

lookup_generic_key(_witnesses_where_lookup_generic_key@0_0,
                   _map@1_0) ->
    _witness_Keyed@2_0 = erlang:element(1,
                                        _witnesses_where_lookup_generic_key@0_0),
    _vtable_Keyed@3_0 = erlang:element(1,
                                       _witness_Keyed@2_0),
    _const_fn_Keyed_KEY@4_0 = erlang:element(1,
                                             _vtable_Keyed@3_0),
    _const_Keyed_KEY@5_0 =
        _const_fn_Keyed_KEY@4_0(_witness_Keyed@2_0),
    begin
        case _map@1_0 of
            #{_const_Keyed_KEY@5_0 := _value@6_0} -> _value@6_0;
            _ -> -1
        end
    end.

lookup_generic_name_key(_map@0_0) ->
    _witness_Keyed@1_0 =
        'Elixir.Patina.MapsPatterns.NameKey':'MapsPatterns::Keyed::$patina_witness'(),
    _witnesses_where_lookup_generic_key@2_0 =
        {_witness_Keyed@1_0},
    begin
        lookup_generic_key(_witnesses_where_lookup_generic_key@2_0,
                           _map@0_0)
    end.

lookup_tuple_key(_map@0_0, _k@1_0) ->
    case _map@0_0 of
        #{{_k@1_0, 1} := _value@2_0} -> _value@2_0;
        _ -> -1
    end.

lookup_list_key(_map@0_0) ->
    case _map@0_0 of
        #{[1, 2] := _value@1_0} -> _value@1_0;
        _ -> -1
    end.

lookup_binary_key(_map@0_0) ->
    case _map@0_0 of
        #{<<1:8/integer, 2:8/integer>> := _value@1_0} ->
            _value@1_0;
        _ -> -1
    end.

lookup_exact_map_key(_map@0_0, _k@1_0) ->
    case _map@0_0 of
        #{#{1 => _k@1_0} := _value@2_0} -> _value@2_0;
        _ -> -1
    end.

lookup_enum_key(_map@0_0, _side@1_0) ->
    case _map@0_0 of
        #{{'Elixir.Patina.MapsPatterns.Shape.Square', _side@1_0}
              := _value@2_0} ->
            _value@2_0;
        #{{'Elixir.Patina.MapsPatterns.Shape.Point'} :=
              _value@3_0} ->
            _value@3_0;
        _ -> -1
    end.

lookup_option_key(_map@0_0) ->
    case _map@0_0 of
        #{1 := _value@1_0} -> _value@1_0;
        #{nil := _value@2_0} -> _value@2_0;
        _ -> -1
    end.
