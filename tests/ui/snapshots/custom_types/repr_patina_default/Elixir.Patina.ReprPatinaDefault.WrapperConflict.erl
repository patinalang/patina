-file("tests/ui/src/custom_types/repr_patina_default.pt",
      65).

-module('Elixir.Patina.ReprPatinaDefault.WrapperConflict').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export([]).

-record #'Pair'{'0'}.
