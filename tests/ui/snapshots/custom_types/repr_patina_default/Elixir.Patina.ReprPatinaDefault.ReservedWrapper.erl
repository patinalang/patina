-file("tests/ui/src/custom_types/repr_patina_default.pt",
      72).

-module('Elixir.Patina.ReprPatinaDefault.ReservedWrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([open]).

-export([]).

-record #open{'0'}.
