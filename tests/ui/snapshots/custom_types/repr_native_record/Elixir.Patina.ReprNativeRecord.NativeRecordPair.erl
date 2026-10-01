-file("tests/ui/src/custom_types/repr_native_record.pt",
      22).

-module('Elixir.Patina.ReprNativeRecord.NativeRecordPair').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['NativeRecordPair']).

-export(['__info__'/1,
         '$patina_constructor__NativeRecordPair'/2]).

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

-record #'NativeRecordPair'{'0', '1'}.

'$patina_constructor__NativeRecordPair'(_0@0_0,
                                        _1@1_0) ->
    #'NativeRecordPair'{'0' = _0@0_0, '1' = _1@1_0}.
