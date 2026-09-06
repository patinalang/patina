-file("tests/ui/src/custom_types/repr_native_record.pt",
      1).

-module('Elixir.Patina.ReprNativeRecord').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         inherited_unit/0,
         overridden_unit/0,
         elixir_unit/0,
         record_struct/0,
         struct_x/1,
         record_pair/0,
         pair_second/1]).

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

inherited_unit() ->
    #'Elixir.Patina.ReprNativeRecord.WholeEnum':'Unit'{}.

overridden_unit() ->
    #'Elixir.Patina.ReprNativeRecord.MixedEnum':'RecordUnit'{}.

elixir_unit() ->
    ('Elixir.Patina.ReprNativeRecord.ElixirUnit':'__struct__'())#{}.

record_struct() ->
    #'Elixir.Patina.ReprNativeRecord.NativeRecordStruct':'NativeRecordStruct'{x
                                                                                  =
                                                                                  10,
                                                                              y
                                                                                  =
                                                                                  20}.

struct_x(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.NativeRecordStruct':'NativeRecordStruct'.x.

record_pair() ->
    #'Elixir.Patina.ReprNativeRecord.NativeRecordPair':'NativeRecordPair'{'0'
                                                                              =
                                                                              30,
                                                                          '1' =
                                                                              40}.

pair_second(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.NativeRecordPair':'NativeRecordPair'.'1'.
