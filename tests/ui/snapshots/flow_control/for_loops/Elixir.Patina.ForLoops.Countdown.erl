-file("tests/ui/src/flow_control/for_loops.pt", 139).

-module('Elixir.Patina.ForLoops.Countdown').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Core::Iterator::type::Item'/1,
         'Core::Iterator::next'/2]).

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

'Core::Iterator::type::Item'(_witness@0_0 =
                                 {'impl Core::Iterator for ForLoops::Countdown',
                                  _}) ->
    {}.

'Core::Iterator::next'(_witness@0_0 =
                           {'impl Core::Iterator for ForLoops::Countdown', _},
                       _self@1_0) ->
    case _self@1_0 =:= 0 of
        true -> nil;
        false -> {_self@1_0, _self@1_0 - 1}
    end.
