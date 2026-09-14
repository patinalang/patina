-file("tests/ui/src/custom_types/repr_recursive.pt",
      41).

-module('Elixir.Patina.ReprRecursive.Generic').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Next']).

-export([]).

-record #'Next'{'0'}.
