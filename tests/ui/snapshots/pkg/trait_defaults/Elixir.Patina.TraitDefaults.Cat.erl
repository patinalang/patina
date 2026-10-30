-file("tests/ui/src/pkg/trait_defaults.pt", 15).

-module('Elixir.Patina.TraitDefaults.Cat').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Traits::Inner::Greet::$patina_type::Name'/1,
         'Traits::Inner::Greet::name'/2,
         'Traits::Inner::Greet::$patina_witness'/0]).

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

'Traits::Inner::Greet::$patina_type::Name'(_witness_Greet@0_0) ->
    _witness_Show@1_0 =
        {{fun 'Elixir.Patina.Traits.Label':'Traits::Show::show'/2},
         {},
         {}},
    {_witness_Show@1_0}.

'Traits::Inner::Greet::name'(_witness_Greet@0_0,
                             _self@1_0) ->
    <<"cat"/utf8>>.

'Traits::Inner::Greet::$patina_witness'() ->
    {{fun 'Elixir.Patina.TraitDefaults.Cat':'Traits::Inner::Greet::$patina_type::Name'/1,
      fun 'Elixir.Patina.Traits.Inner':'Traits::Inner::Greet::$patina_const::PREFIX'/1,
      fun 'Elixir.Patina.TraitDefaults.Cat':'Traits::Inner::Greet::name'/2,
      fun 'Elixir.Patina.Traits.Inner':'Traits::Inner::Greet::greet'/2},
     {},
     {}}.
