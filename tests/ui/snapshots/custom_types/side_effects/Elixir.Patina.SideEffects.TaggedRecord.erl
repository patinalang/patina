-file("tests/ui/src/custom_types/side_effects.pt", 65).

-module('Elixir.Patina.SideEffects.TaggedRecord').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         fields/0,
         base/0,
         update/0,
         mixed/0,
         replace/0]).

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

fields() ->
    _x@0_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _y@1_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
    _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    {tagged, _x@0_0, _y@1_0, _z@2_0}.

base() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           {tagged, 1, 2, 3}),
    _field_base@0_0.

update() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           {tagged, 1, 2, 3}),
    erlang:setelement(2, _field_base@0_0, 10).

mixed() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           {tagged, 1, 2, 3}),
    _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    erlang:setelement(4,
                      erlang:setelement(2, _field_base@0_0, _x@1_0),
                      _z@2_0).

replace() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           {tagged, 1, 2, 3}),
    _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _y@2_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
    _z@3_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    erlang:setelement(4,
                      erlang:setelement(3,
                                        erlang:setelement(2,
                                                          _field_base@0_0,
                                                          _x@1_0),
                                        _y@2_0),
                      _z@3_0).
