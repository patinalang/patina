-file("tests/ui/src/custom_types/repr__03.pt", 21).

-module('Elixir.Patina.Repr03.OnlyUnits').

-compile([no_auto_import]).

-export_record(['VariantA', 'VariantB', 'VariantC']).

-export([]).

-record #'VariantA'{}.

-record #'VariantB'{}.

-record #'VariantC'{}.
