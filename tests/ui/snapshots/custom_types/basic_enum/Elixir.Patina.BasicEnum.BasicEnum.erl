-file("tests/ui/src/custom_types/basic_enum.pt", 1).

-module('Elixir.Patina.BasicEnum.BasicEnum').

-compile([no_auto_import]).

-export_record(['Tuple', 'Struct']).

-export([]).

-record #'Tuple'{'0', '1'}.

-record #'Struct'{x, y}.
