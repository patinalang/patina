-file("tests/ui/src/trait/defaults.pt", 304).

-module('Elixir.Patina.Defaults.Loose').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Container::$patina_type::Item'/1,
         'Defaults::Container::get'/2,
         'Defaults::Container::needs_item_witness'/3]).

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

'Defaults::Container::$patina_type::Item'(_witness@0_0 =
                                              {'impl Defaults::Container for Defaults::Loose<_>',
                                               _}) ->
    {}.

'Defaults::Container::get'(_witness@0_0 =
                               {'impl Defaults::Container for Defaults::Loose<_>',
                                _},
                           _self@1_0) ->
    _self@1_0.

'Defaults::Container::needs_item_witness'(_witness@0_0 =
                                              {'impl Defaults::Container for Defaults::Loose<_>',
                                               _},
                                          _arg@1_0, _arg@2_0) ->
    'Elixir.Patina.Defaults':'Defaults::Container::needs_item_witness'(_witness@0_0,
                                                                       _arg@1_0,
                                                                       _arg@2_0).
