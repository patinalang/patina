-file("tests/ui/src/generics/witness/supertraits.pt",
      3).

-module('Elixir.Patina.Supertraits.Bob').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Supertraits::Named::name'/2,
         'Supertraits::Named::$patina_witness'/0,
         'Supertraits::Greet::greeting'/2,
         'Supertraits::Greet::$patina_witness'/0,
         'Supertraits::Loud::$patina_witness'/0]).

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

'Supertraits::Named::name'(_witness@0_0, _self@1_0) ->
    <<"Bob"/utf8>>.

'Supertraits::Named::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits.Bob':'Supertraits::Named::name'/2},
     {},
     {}}.

'Supertraits::Greet::greeting'(_witness@0_0,
                               _self@1_0) ->
    <<"Hi"/utf8>>.

'Supertraits::Greet::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits.Bob':'Supertraits::Greet::greeting'/2,
      fun 'Elixir.Patina.Supertraits':'Supertraits::Greet::calls_supertrait'/2},
     {{{fun 'Elixir.Patina.Supertraits.Bob':'Supertraits::Named::name'/2},
       {},
       {}}},
     {}}.

'Supertraits::Loud::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits':'Supertraits::Loud::shout'/2},
     {{{fun 'Elixir.Patina.Supertraits.Bob':'Supertraits::Greet::greeting'/2,
        fun 'Elixir.Patina.Supertraits':'Supertraits::Greet::calls_supertrait'/2},
       {{{fun 'Elixir.Patina.Supertraits.Bob':'Supertraits::Named::name'/2},
         {},
         {}}},
       {}}},
     {}}.
