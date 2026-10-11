-file("tests/ui/src/custom_types/repr_tagged.pt", 1).

-module('Elixir.Patina.ReprTagged').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         struct_value/0,
         struct_code/1,
         update_code/1,
         update_fields/1,
         destructure_struct/1,
         unit_value/0,
         enum_values/0,
         enum_payload/1,
         result_values/0,
         result_payload/1,
         main/0]).

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

struct_value() ->
    {struct_http_style, 200, <<"ok"/utf8>>}.

struct_code(_value@0_0) ->
    erlang:element(2, _value@0_0).

update_code(_value@0_0) ->
    erlang:setelement(2, _value@0_0, 201).

update_fields(_value@0_0) ->
    erlang:setelement(3,
                      erlang:setelement(2, _value@0_0, 202),
                      <<"updated"/utf8>>).

destructure_struct(_value@0_0) ->
    {struct_http_style, _code@1_0, _} = _value@0_0,
    _code@1_0.

unit_value() -> {custom_unit}.

enum_values() ->
    [{'Elixir.Patina.ReprTagged.TaggedEnum.Unit'},
     {'Elixir.Patina.ReprTagged.TaggedEnum.TupleValue', 10},
     {'Elixir.Patina.ReprTagged.TaggedEnum.RecordValue',
      20,
      30},
     'Elixir.Patina.ReprTagged.TaggedEnum.AtomOverride'].

enum_payload(_value@0_0) ->
    case _value@0_0 of
        {'Elixir.Patina.ReprTagged.TaggedEnum.Unit'} -> 0;
        {'Elixir.Patina.ReprTagged.TaggedEnum.TupleValue',
         _value@1_0} ->
            _value@1_0;
        {'Elixir.Patina.ReprTagged.TaggedEnum.RecordValue',
         _x@2_0,
         _} ->
            _x@2_0;
        'Elixir.Patina.ReprTagged.TaggedEnum.AtomOverride' -> -1
    end.

result_values() ->
    [{ok, 42}, {error, <<"reason"/utf8>>}].

result_payload(_value@0_0) ->
    case _value@0_0 of
        {ok, _value@1_0} -> _value@1_0;
        {error, _} -> -1
    end.

main() -> {}.
