-file("tests/ui/src/custom_types/repr__06.pt", 5).

-module('Elixir.Patina.Repr06.SnakeCaseEnum').

-compile([no_auto_import]).

-export_record(['StructHTTPStyle']).

-export([]).

-record #'StructHTTPStyle'{code, body}.
