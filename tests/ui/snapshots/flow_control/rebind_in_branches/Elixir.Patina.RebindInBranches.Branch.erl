-file("tests/ui/src/flow_control/rebind_in_branches.pt",
      116).

-module('Elixir.Patina.RebindInBranches.Branch').

-compile([no_auto_import]).

-export_record(['First', 'Second']).

-export([]).

-record #'First'{}.

-record #'Second'{}.
