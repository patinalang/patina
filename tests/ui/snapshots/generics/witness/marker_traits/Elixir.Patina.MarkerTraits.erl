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

subtrait_of_marker(_witness@0_0, _t@1_0) ->
    begin
        _witness@2_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0, _t@1_0)
    end.

a_both() ->
    subtrait_of_marker({'Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::$patina_witness'()},
                       'Elixir.Patina.MarkerTraits.A').

a_marked() ->
    'Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::show'('Elixir.Patina.MarkerTraits.A':'MarkerTraits::Show::$patina_witness'(),
                                                              marker_bounds_only('Elixir.Patina.MarkerTraits.A')).
