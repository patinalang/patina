-file("tests/ui/src/custom_types/repr_recursive.pt",
      18).

-module('Elixir.Patina.ReprRecursive.Left').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Next']).

-export([]).

-record #'Next'{'0'}.
