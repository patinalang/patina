-file("tests/ui/src/generics/witness/marker_traits.pt",
      1).

-module('Elixir.Patina.MarkerTraits').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         marker_bounds_only/1,
         subtrait_of_marker/2,
         a_both/0,
         a_marked/0]).

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

marker_bounds_only(_t@0_0) -> _t@0_0.

subtrait_of_marker(_witnesses_where_subtrait_of_marker@0_0,
                   _t@1_0) ->
    _witness_Show@2_0 = erlang:element(1,
                                       _witnesses_where_subtrait_of_marker@0_0),
    _vtable_Show@3_0 = erlang:element(1, _witness_Show@2_0),
    _fn_Show_show@4_0 = erlang:element(1, _vtable_Show@3_0),
    begin _fn_Show_show@4_0(_witness_Show@2_0, _t@1_0) end.

a_both() ->
    _witness_Show@0_0 =
        'Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::$patina_witness'(),
    _witnesses_where_subtrait_of_marker@1_0 =
        {_witness_Show@0_0},
    begin
        subtrait_of_marker(_witnesses_where_subtrait_of_marker@1_0,
                           'Elixir.Patina.MarkerTraits.A')
    end.

a_marked() ->
    _witness_Show@0_0 =
        'Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::$patina_witness'(),
    begin
        'Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::show'(_witness_Show@0_0,
                                                                  marker_bounds_only('Elixir.Patina.MarkerTraits.A'))
    end.
