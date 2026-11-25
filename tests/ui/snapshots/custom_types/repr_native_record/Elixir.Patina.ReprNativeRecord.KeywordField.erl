-file("tests/ui/src/custom_types/repr_native_record.pt",
      59).

-module('Elixir.Patina.ReprNativeRecord.KeywordField').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['KeywordField']).

-export(['__info__'/1,
         '$patina_constructor__KeywordField'/2]).

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

-record #'KeywordField'{type, other}.

'$patina_constructor__KeywordField'(_type@0_0,
                                    _other@1_0) ->
    #'KeywordField'{type = _type@0_0, other = _other@1_0}.
