-file("tests/ui/src/flow_control/for_loops.pt", 163).

-module('Elixir.Patina.ForLoops.Bag').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'core::IntoIterator::$patina_type::Item'/1,
         'core::IntoIterator::$patina_type::IntoIter'/1,
         'core::IntoIterator::into_iter'/2,
         'core::IntoIterator::$patina_witness'/0]).

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

'core::IntoIterator::$patina_type::Item'(_witness_IntoIterator@0_0) ->
    {}.

'core::IntoIterator::$patina_type::IntoIter'(_witness_IntoIterator@0_0) ->
    _witness_Iterator@1_0 =
        {{fun 'Elixir.Patina.Core.ListIter':'core::Iterator::$patina_type::Item'/1,
          fun 'Elixir.Patina.Core.ListIter':'core::Iterator::next'/2},
         {},
         {}},
    {_witness_Iterator@1_0}.

'core::IntoIterator::into_iter'(_witness_IntoIterator@0_0,
                                _self@1_0) ->
    _self@2_0 = _self@1_0,
    _self@2_0.

'core::IntoIterator::$patina_witness'() ->
    {{fun 'Elixir.Patina.ForLoops.Bag':'core::IntoIterator::$patina_type::Item'/1,
      fun 'Elixir.Patina.ForLoops.Bag':'core::IntoIterator::$patina_type::IntoIter'/1,
      fun 'Elixir.Patina.ForLoops.Bag':'core::IntoIterator::into_iter'/2},
     {},
     {}}.
