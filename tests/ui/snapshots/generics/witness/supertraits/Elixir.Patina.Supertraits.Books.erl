-file("tests/ui/src/generics/witness/supertraits.pt",
      77).

-module('Elixir.Patina.Supertraits.Books').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Supertraits::Shelf::size'/2,
         'Supertraits::Shelf::$patina_witness'/0,
         'Supertraits::Library::$patina_witness'/0]).

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

'Supertraits::Shelf::size'(_witness@0_0, _self@1_0) ->
    _self@1_0.

'Supertraits::Shelf::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits.Books':'Supertraits::Shelf::size'/2},
     {},
     {}}.

'Supertraits::Library::$patina_witness'() ->
    {{fun 'Elixir.Patina.Supertraits':'Supertraits::Library::count'/4},
     {{{fun 'Elixir.Patina.Supertraits.Books':'Supertraits::Shelf::size'/2},
       {},
       {}}},
     {}}.
