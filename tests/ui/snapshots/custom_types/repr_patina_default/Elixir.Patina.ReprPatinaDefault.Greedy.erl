-file("tests/ui/src/custom_types/repr_patina_default.pt",
      29).

-module('Elixir.Patina.ReprPatinaDefault.Greedy').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Second']).

-export([]).

-record #'Second'{'0'}.
