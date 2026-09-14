-file("tests/ui/src/custom_types/repr_recursive.pt", 8).

-module('Elixir.Patina.ReprRecursive.Tree').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Branch']).

-export([]).

-record #'Branch'{'0', '1'}.
