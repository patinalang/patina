-file("tests/ui/src/custom_types/repr__10.pt", 5).

-module('Elixir.Patina.Repr10.NativeRecordEnum').

-compile([no_auto_import]).

-export_record(['Point', 'WithCustomName']).

-export([]).

-record #'Point'{'0', '1'}.

-record #'WithCustomName'{'0'}.
