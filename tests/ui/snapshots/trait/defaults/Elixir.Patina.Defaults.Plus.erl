-file("tests/ui/src/trait/defaults.pt", 381).

-module('Elixir.Patina.Defaults.Plus').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Numbered::const::C'/1,
         'Defaults::Numbered::const::D'/1,
         'Defaults::Numbered::consts'/2]).

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

'Defaults::Numbered::const::C'(_witness@0_0 =
                                   {'impl Defaults::Numbered for Defaults::Plus<_>',
                                    _,
                                    _witness@1_0}) ->
    (erlang:element(2,
                    _witness@1_0)):'Defaults::Numbered::const::C'(_witness@1_0)
        + 10.

'Defaults::Numbered::const::D'(_witness@0_0 =
                                   {'impl Defaults::Numbered for Defaults::Plus<_>',
                                    _,
                                    _}) ->
    'Elixir.Patina.Defaults':'Defaults::Numbered::const::D'(_witness@0_0).

'Defaults::Numbered::consts'(_witness@0_0 =
                                 {'impl Defaults::Numbered for Defaults::Plus<_>',
                                  _,
                                  _},
                             _arg@1_0) ->
    'Elixir.Patina.Defaults':'Defaults::Numbered::consts'(_witness@0_0,
                                                          _arg@1_0).
