-file("tests/ui/src/custom_types/repr__03.pt", 11).

-module('Elixir.Patina.Repr03.NotJustUnits').

-compile([no_auto_import]).

-export_record([variant_c]).

-export([]).

-record #variant_c{'0'}.
