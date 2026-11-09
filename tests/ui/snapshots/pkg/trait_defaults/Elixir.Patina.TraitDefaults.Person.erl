-file("tests/ui/src/pkg/trait_defaults.pt", 3).

-module('Elixir.Patina.TraitDefaults.Person').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'traits::inner::Greet::$patina_type::Name'/1,
         'traits::inner::Greet::$patina_const::PREFIX'/1,
         'traits::inner::Greet::name'/2,
         'traits::inner::Greet::$patina_witness'/0]).

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

'traits::inner::Greet::$patina_type::Name'(_witness_Greet@0_0) ->
    _witness_Show@1_0 =
        {{fun 'Elixir.Patina.Traits.Label':'traits::Show::show'/2},
         {},
         {}},
    {_witness_Show@1_0}.

'traits::inner::Greet::$patina_const::PREFIX'(_witness_Greet@0_0) ->
    <<"Hello"/utf8>>.

'traits::inner::Greet::name'(_witness_Greet@0_0,
                             _self@1_0) ->
    _0@2_0 = _self@1_0,
    _0@2_0.

'traits::inner::Greet::$patina_witness'() ->
    {{fun 'Elixir.Patina.TraitDefaults.Person':'traits::inner::Greet::$patina_type::Name'/1,
      fun 'Elixir.Patina.TraitDefaults.Person':'traits::inner::Greet::$patina_const::PREFIX'/1,
      fun 'Elixir.Patina.TraitDefaults.Person':'traits::inner::Greet::name'/2,
      fun 'Elixir.Patina.Traits.Inner':'traits::inner::Greet::greet'/2},
     {},
     {}}.
