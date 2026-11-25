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
         pair_second/1,
         keyword_field/0,
         keyword_field_value/1,
         keyword_field_updated/1,
         keyword_field_pattern/1]).

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
    'Elixir.Patina.ReprNativeRecord.WholeEnum':'$patina_constructor__Unit'().

overridden_unit() ->
    'Elixir.Patina.ReprNativeRecord.MixedEnum':'$patina_constructor__RecordUnit'().

elixir_unit() ->
    ('Elixir.Patina.ReprNativeRecord.ElixirUnit':'__struct__'())#{}.

record_struct() ->
    'Elixir.Patina.ReprNativeRecord.NativeRecordStruct':'$patina_constructor__NativeRecordStruct'(10,
                                                                                                  20).

struct_x(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.NativeRecordStruct':'NativeRecordStruct'.x.

record_pair() ->
    'Elixir.Patina.ReprNativeRecord.NativeRecordPair':'$patina_constructor__NativeRecordPair'(30,
                                                                                              40).

pair_second(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.NativeRecordPair':'NativeRecordPair'.'1'.

keyword_field() ->
    'Elixir.Patina.ReprNativeRecord.KeywordField':'$patina_constructor__KeywordField'(7,
                                                                                      1).

keyword_field_value(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.KeywordField':'KeywordField'.type.

keyword_field_updated(_value@0_0) ->
    _value@0_0#'Elixir.Patina.ReprNativeRecord.KeywordField':'KeywordField'{type
                                                                                =
                                                                                8}.

keyword_field_pattern(_value@0_0) ->
    #'Elixir.Patina.ReprNativeRecord.KeywordField':'KeywordField'{type
                                                                      =
                                                                      _type@1_0,
                                                                  other = _} =
        _value@0_0,
    _type@1_0.
