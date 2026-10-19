-file("tests/ui/src/flow_control/for_loops.pt", 139).

-module('Elixir.Patina.ForLoops.Countdown').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Core::Iterator::$patina_type::Item'/1,
         'Core::Iterator::next'/2,
         'Core::Iterator::$patina_witness'/0]).

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

'Core::Iterator::$patina_type::Item'(_witness_Iterator@0_0) ->
    {}.

'Core::Iterator::next'(_witness_Iterator@0_0,
                       _self@1_0) ->
    case _self@1_0 =:= 0 of
        true -> nil;
        _ ->
            begin
                _0@5_0 = begin
                             _element@2_0 = _self@1_0,
                             _element@4_0 = begin
                                                _from@3_0 = _self@1_0 - 1,
                                                _from@3_0
                                            end,
                             {_element@2_0, _element@4_0}
                         end,
                _0@5_0
            end
    end.

'Core::Iterator::$patina_witness'() ->
    {{fun 'Elixir.Patina.ForLoops.Countdown':'Core::Iterator::$patina_type::Item'/1,
      fun 'Elixir.Patina.ForLoops.Countdown':'Core::Iterator::next'/2},
     {},
     {}}.
