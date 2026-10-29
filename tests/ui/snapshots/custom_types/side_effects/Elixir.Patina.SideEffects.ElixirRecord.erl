-file("tests/ui/src/custom_types/side_effects.pt", 66).

-module('Elixir.Patina.SideEffects.ElixirRecord').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         fields/0,
         base/0,
         update/0,
         mixed/0,
         replace/0]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => x, default => nil, required => false},
     #{field => y, default => nil, required => false},
     #{field => z, default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' =>
          'Elixir.Patina.SideEffects.ElixirRecord',
      x => nil, y => nil, z => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

fields() ->
    _x@0_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _y@1_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
    _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    ('__struct__'())#{x := _x@0_0, y := _y@1_0,
                      z := _z@2_0}.

base() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           ('__struct__'())#{x := 1, y := 2,
                                                             z := 3}),
    _field_base@0_0#{}.

update() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           ('__struct__'())#{x := 1, y := 2,
                                                             z := 3}),
    _field_base@0_0#{x := 10}.

mixed() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           ('__struct__'())#{x := 1, y := 2,
                                                             z := 3}),
    _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _z@2_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    _field_base@0_0#{x := _x@1_0, z := _z@2_0}.

replace() ->
    _field_base@0_0 =
        'Elixir.Patina.SideEffects':effect(base,
                                           ('__struct__'())#{x := 1, y := 2,
                                                             z := 3}),
    _x@1_0 = 'Elixir.Patina.SideEffects':effect(x, 10),
    _y@2_0 = 'Elixir.Patina.SideEffects':effect(y, 20),
    _z@3_0 = 'Elixir.Patina.SideEffects':effect(z, 30),
    _field_base@0_0#{x := _x@1_0, y := _y@2_0, z := _z@3_0}.
