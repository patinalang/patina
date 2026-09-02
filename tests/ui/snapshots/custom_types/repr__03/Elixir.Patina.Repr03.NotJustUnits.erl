-file("tests/ui/src/custom_types/repr__03.pt", 11).

-module('Elixir.Patina.Repr03.NotJustUnits').

-compile([no_auto_import]).

-export_record(['VariantA', 'VariantB', 'VariantC']).

-export([]).

-record #'VariantA'{}.

-record #'VariantB'{}.

-record #'VariantC'{'0'}.
