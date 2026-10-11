-file("tests/ui/src/flow_control/for_loops.pt", 163).

-module('Elixir.Patina.ForLoops.Bag').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Core::IntoIterator::$patina_type::Item'/1,
         'Core::IntoIterator::$patina_type::IntoIter'/1,
         'Core::IntoIterator::into_iter'/2]).

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

'Core::IntoIterator::$patina_type::Item'(_witness@0_0 =
                                             {'impl Core::IntoIterator for ForLoops::Bag',
                                              _}) ->
    {}.

'Core::IntoIterator::$patina_type::IntoIter'(_witness@0_0 =
                                                 {'impl Core::IntoIterator for ForLoops::Bag',
                                                  _}) ->
    {{'impl Core::Iterator for Core::ListIter<_>',
      'Elixir.Patina.Core.ListIter'}}.

'Core::IntoIterator::into_iter'(_witness@0_0 =
                                    {'impl Core::IntoIterator for ForLoops::Bag',
                                     _},
                                _self@1_0) ->
    'Elixir.Patina.Core.List':iter(_self@1_0).
