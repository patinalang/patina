-file("tests/ui/src/custom_types/repr_recursive.pt",
      142).

-module('Elixir.Patina.ReprRecursive.Void').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export([]).
