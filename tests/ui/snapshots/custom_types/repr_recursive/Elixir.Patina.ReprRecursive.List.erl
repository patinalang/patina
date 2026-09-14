-file("tests/ui/src/custom_types/repr_recursive.pt", 3).

-module('Elixir.Patina.ReprRecursive.List').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Cons']).

-export([]).

-record #'Cons'{'0', '1'}.
