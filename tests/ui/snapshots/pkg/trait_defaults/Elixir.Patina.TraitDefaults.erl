-file("tests/ui/src/pkg/trait_defaults.pt", 1).

-module('Elixir.Patina.TraitDefaults').

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

local_greet(_witnesses_where_local_greet@0_0,
            _greeter@1_0) ->
    _witness_Greet@2_0 = erlang:element(1,
                                        _witnesses_where_local_greet@0_0),
    _vtable_Greet@3_0 = erlang:element(1,
                                       _witness_Greet@2_0),
    _fn_Greet_greet@4_0 = erlang:element(4,
                                         _vtable_Greet@3_0),
    _fn_Greet_greet@4_0(_witness_Greet@2_0, _greeter@1_0).

main() ->
    _witness_Greet@0_0 =
        'Elixir.Patina.TraitDefaults.Person':'traits::inner::Greet::$patina_witness'(),
    _witness_Greet@1_0 =
        'Elixir.Patina.TraitDefaults.Cat':'traits::inner::Greet::$patina_witness'(),
    _witnesses_where_local_greet@2_0 = {_witness_Greet@1_0},
    _witnesses_where_greet_all@3_0 = {_witness_Greet@0_0},
    _witness_Greet@4_0 =
        'Elixir.Patina.Traits.Robot':'traits::inner::Greet::$patina_witness'(),
    'Elixir.Patina.Traits.Inner':'traits::inner::Greet::greet'(_witness_Greet@0_0,
                                                               <<"Ann"/utf8>>),
    'Elixir.Patina.Traits.Inner':'traits::inner::Greet::greet'(_witness_Greet@1_0,
                                                               'Elixir.Patina.TraitDefaults.Cat'),
    local_greet(_witnesses_where_local_greet@2_0,
                'Elixir.Patina.TraitDefaults.Cat'),
    'Elixir.Patina.Traits':greet_all(_witnesses_where_greet_all@3_0,
                                     <<"Bob"/utf8>>),
    'Elixir.Patina.Traits':greet_all(_witnesses_where_local_greet@2_0,
                                     'Elixir.Patina.TraitDefaults.Cat'),
    'Elixir.Patina.Traits.Inner':'traits::inner::Greet::greet'(_witness_Greet@4_0,
                                                               'Elixir.Patina.Traits.Robot').
