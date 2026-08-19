-file("tests/ui/src/custom_types/enum__03.pt", 4).
-module('Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers').
-compile([no_auto_import]).
-export_record(['Add', 'Subtract']).
-export([]).
-record #'Add'{}.
-record #'Subtract'{}.
run(_self@0_0, _x@1_0, _y@2_0) ->
    case _self@0_0 of
        'Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers.Add' ->
            _x@1_0 + _y@2_0;
        'Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers.Subtract' ->
            _x@1_0 - _y@2_0
    end.
