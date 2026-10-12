-file("tests/ui/src/custom_types/side_effects.pt", 136).

-module('Elixir.Patina.SideEffects.Maps').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         fields/0,
         base/0,
         update/0,
         mixed/0,
         replace/0,
         duplicate/0]).

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
    begin
        _key@0_0 = 'Elixir.Patina.SideEffects':effect(x_key, x),
        _value@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _key@2_0 = 'Elixir.Patina.SideEffects':effect(y_key, y),
        _value@3_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
        _key@4_0 = 'Elixir.Patina.SideEffects':effect(z_key, z),
        _value@5_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        #{_key@0_0 => _value@1_0, _key@2_0 => _value@3_0,
          _key@4_0 => _value@5_0}
    end.

base() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #{x => 1, y => 2, z => 3}),
        _field_base@0_0#{}
    end.

update() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #{x => 1, y => 2, z => 3}),
        _field_base@0_0#{x => 10}
    end.

mixed() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #{x => 1, y => 2, z => 3}),
        _key@1_0 = 'Elixir.Patina.SideEffects':effect(z_key, z),
        _value@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        _key@3_0 = 'Elixir.Patina.SideEffects':effect(x_key, x),
        _value@4_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _field_base@0_0#{_key@1_0 => _value@2_0,
                         _key@3_0 => _value@4_0}
    end.

replace() ->
    begin
        _field_base@0_0 =
            'Elixir.Patina.SideEffects':effect(base,
                                               #{x => 1, y => 2, z => 3}),
        _key@1_0 = 'Elixir.Patina.SideEffects':effect(z_key, z),
        _value@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
        _key@3_0 = 'Elixir.Patina.SideEffects':effect(x_key, x),
        _value@4_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
        _key@5_0 = 'Elixir.Patina.SideEffects':effect(y_key, y),
        _value@6_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
        _field_base@0_0#{_key@1_0 => _value@2_0,
                         _key@3_0 => _value@4_0, _key@5_0 => _value@6_0}
    end.

duplicate() ->
    begin
        _key@0_0 = 'Elixir.Patina.SideEffects':effect(key, x),
        _value@1_0 = 'Elixir.Patina.SideEffects':effect(value,
                                                        10),
        _key@2_0 = 'Elixir.Patina.SideEffects':effect(key, x),
        _value@3_0 = 'Elixir.Patina.SideEffects':effect(value,
                                                        20),
        #{_key@0_0 => _value@1_0, _key@2_0 => _value@3_0}
    end.
