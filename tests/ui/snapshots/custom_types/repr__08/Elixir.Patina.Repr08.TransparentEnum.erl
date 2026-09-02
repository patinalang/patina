-file("tests/ui/src/custom_types/repr__08.pt", 11).

-module('Elixir.Patina.Repr08.TransparentEnum').

-compile([no_auto_import]).

-export_record(['IntVariant', 'PointVariant']).

-export([]).

-record #'IntVariant'{'0'}.

-record #'PointVariant'{point}.
