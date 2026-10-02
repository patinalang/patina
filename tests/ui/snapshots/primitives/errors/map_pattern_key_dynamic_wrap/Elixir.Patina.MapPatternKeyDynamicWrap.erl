-file("tests/ui/src/primitives/errors/map_pattern_ke"
      "y_dynamic_wrap.pt",
      2).

-module('Elixir.Patina.MapPatternKeyDynamicWrap').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, generic_option_key/2]).

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

generic_option_key(_map@0_0, _value@1_0) ->
    case _map@0_0 of
        #{case _value@1_0 of
              _value@2_0 when _value@2_0 =:= nil ->
                  {some, _value@2_0};
              _value@2_0
                  when (erlang:is_tuple(_value@2_0) andalso
                            erlang:tuple_size(_value@2_0) =:= 2)
                           andalso erlang:element(1, _value@2_0) =:= some ->
                  {some, _value@2_0};
              _value@2_0 -> _value@2_0
          end
              := _found@3_0} ->
            _found@3_0;
        _ -> -1
    end.
