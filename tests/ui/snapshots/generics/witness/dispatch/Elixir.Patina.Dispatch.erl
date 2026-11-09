-file("tests/ui/src/generics/witness/dispatch.pt", 1).

-module('Elixir.Patina.Dispatch').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

total_area(_witnesses_where_total_area@0_0, _t@1_0) ->
    _witness_Area@2_0 = erlang:element(1,
                                       _witnesses_where_total_area@0_0),
    _vtable_Area@3_0 = erlang:element(1, _witness_Area@2_0),
    _fn_Area_area@4_0 = erlang:element(1, _vtable_Area@3_0),
    _fn_Area_area@4_0(_witness_Area@2_0, _t@1_0).

main() ->
    _witness_Area@0_0 =
        'Elixir.Patina.Dispatch.Point':'dispatch::Area::$patina_witness'(),
    _witnesses_where_total_area@1_0 = {_witness_Area@0_0},
    _p@2_0 =
        'Elixir.Patina.Dispatch.Point':'$patina_constructor__Point'(3,
                                                                    4),
    io:put_chars(begin
                     _element@4_0 = begin
                                        _element@3_0 = io_lib:format("~ktp",
                                                                     [total_area(_witnesses_where_total_area@1_0,
                                                                                 _p@2_0)]),
                                        [_element@3_0]
                                    end,
                     [_element@4_0]
                 end),
    io:nl(),
    {}.
