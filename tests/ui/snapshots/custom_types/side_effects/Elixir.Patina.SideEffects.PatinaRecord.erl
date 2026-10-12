-file("tests/ui/src/custom_types/side_effects.pt", 63).

-module('Elixir.Patina.SideEffects.PatinaRecord').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['PatinaRecord']).

-export(['__info__'/1,
         '$patina_constructor__PatinaRecord'/3,
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

-record #'PatinaRecord'{x, y, z}.

'$patina_constructor__PatinaRecord'(_x@0_0, _y@1_0,
                                    _z@2_0) ->
    #'PatinaRecord'{x = _x@0_0, y = _y@1_0, z = _z@2_0}.

fields() ->
    begin
        _x@0_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _y@1_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
        _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        #'PatinaRecord'{x = _x@0_0, y = _y@1_0, z = _z@2_0}
    end.

base() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #'PatinaRecord'{x = 1, y = 2,
                                                               z = 3}),
        _field_base@0_0#'PatinaRecord'{}
    end.

update() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #'PatinaRecord'{x = 1, y = 2,
                                                               z = 3}),
        _field_base@0_0#'PatinaRecord'{x = 10}
    end.

mixed() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #'PatinaRecord'{x = 1, y = 2,
                                                               z = 3}),
        _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        _field_base@0_0#'PatinaRecord'{x = _x@1_0, z = _z@2_0}
    end.

replace() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #'PatinaRecord'{x = 1, y = 2,
                                                               z = 3}),
        _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _y@2_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
        _z@3_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        _field_base@0_0#'PatinaRecord'{x = _x@1_0, y = _y@2_0,
                                       z = _z@3_0}
    end.
