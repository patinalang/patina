-file("tests/ui/src/custom_types/repr__10.pt", 18).

-module('Elixir.Patina.Repr10.NativeRecordStruct').

-compile([no_auto_import]).

-export_record(['NativeRecordStruct']).

-export(['__info__'/1]).

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

-record #'NativeRecordStruct'{foo}.
