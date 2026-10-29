-file("tests/ui/src/custom_types/side_effects.pt", 119).

-module('Elixir.Patina.SideEffects.Tuples').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, fields/0, base/0, mixed/0]).

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
    _element@0_0 = 'Elixir.Patina.SideEffects':effect(x,
                                                      10),
    _element@1_0 = 'Elixir.Patina.SideEffects':effect(y,
                                                      20),
    _element@2_0 = 'Elixir.Patina.SideEffects':effect(z,
                                                      30),
    {_element@0_0, _element@1_0, _element@2_0}.

base() ->
    erlang:element(2,
                   'Elixir.Patina.SideEffects':effect(base, {1, 2, 3})).

mixed() ->
    _element@0_0 = 'Elixir.Patina.SideEffects':effect(x,
                                                      10),
    _element@1_0 = erlang:element(2,
                                  'Elixir.Patina.SideEffects':effect(base,
                                                                     {1,
                                                                      2,
                                                                      3})),
    _element@2_0 = 'Elixir.Patina.SideEffects':effect(z,
                                                      30),
    {_element@0_0, _element@1_0, _element@2_0}.
