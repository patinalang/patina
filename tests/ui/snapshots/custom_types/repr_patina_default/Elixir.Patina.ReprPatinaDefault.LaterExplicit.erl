-file("tests/ui/src/custom_types/repr_patina_default.pt",
      58).

-module('Elixir.Patina.ReprPatinaDefault.LaterExplicit').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Inferred']).

-export([]).

-record #'Inferred'{'0'}.
