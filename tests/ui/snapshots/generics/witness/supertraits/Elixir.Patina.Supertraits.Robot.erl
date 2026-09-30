-file("tests/ui/src/generics/witness/supertraits.pt",
      4).

-module('Elixir.Patina.Supertraits.Robot').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Supertraits::Named::name'/2,
         'Supertraits::Greet::greeting'/2,
         'Supertraits::Greet::calls_supertrait'/2]).

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

'Supertraits::Named::name'(_witness@0_0 =
                               {'impl Supertraits::Named for Supertraits::Robot',
                                _},
                           _self@1_0) ->
    <<"Robot"/utf8>>.

'Supertraits::Greet::greeting'(_witness@0_0 =
                                   {'impl Supertraits::Greet for Supertraits::Robot',
                                    _,
                                    _witness@1_0},
                               _self@2_0) ->
    <<"Beep"/utf8>>.

'Supertraits::Greet::calls_supertrait'(_witness@0_0 =
                                           {'impl Supertraits::Greet for Supertraits::Robot',
                                            _,
                                            _},
                                       _arg@1_0) ->
    'Elixir.Patina.Supertraits':'Supertraits::Greet::calls_supertrait'(_witness@0_0,
                                                                       _arg@1_0).
