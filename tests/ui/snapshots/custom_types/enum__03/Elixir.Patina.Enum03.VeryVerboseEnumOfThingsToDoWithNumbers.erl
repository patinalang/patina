-file("tests/ui/src/custom_types/enum__03.pt", 4).
-module('Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers').
-compile([no_auto_import]).
-export_record(['Add', 'Subtract']).
-export([]).
-record #'Add'{}.
-record #'Subtract'{}.
run(_var_0, _var_1, _var_2) ->
    case _var_0 of
        'Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers.Add' ->
            _var_1 + _var_2;
        'Elixir.Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers.Subtract' ->
            _var_1 - _var_2
    end.
