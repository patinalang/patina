-file("tests/ui/src/custom_types/phantom_data.pt", 78).

-module('Elixir.Patina.PhantomData.MapIter').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, new/1, next/1]).

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

new(_map@0_0) ->
    begin
        _0@1_0 = maps:iterator(_map@0_0),
        _1@2_0 = 'Elixir.Patina.Core.PhantomData',
        _0@1_0
    end.

next(_self@0_0) ->
    case maps:next(_self@0_0) of
        {_key@1_0, _value@2_0, _rest@3_0} ->
            _entry@6_0 = begin
                             begin
                                 _element@4_0 = _key@1_0,
                                 _element@5_0 = _value@2_0,
                                 {_element@4_0, _element@5_0}
                             end
                         end,
            begin
                _0@9_0 = begin
                             _element@8_0 = begin
                                                _1@7_0 =
                                                    'Elixir.Patina.Core.PhantomData',
                                                _rest@3_0
                                            end,
                             {_entry@6_0, _element@8_0}
                         end,
                _0@9_0
            end;
        _ -> nil
    end.
