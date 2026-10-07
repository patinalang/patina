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
        _ = 'Elixir.Patina.Core.PhantomData',
        maps:iterator(_map@0_0)
    end.

next(_self@0_0) ->
    case maps:next(_self@0_0) of
        {_key@1_0, _value@2_0, _rest@3_0} ->
            _entry@4_0 = begin {_key@1_0, _value@2_0} end,
            {_entry@4_0,
             begin
                 _ = 'Elixir.Patina.Core.PhantomData', _rest@3_0
             end};
        _ -> nil
    end.
