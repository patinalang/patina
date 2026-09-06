-file("tests/ui/src/custom_types/repr_transparent.pt",
      32).

-module('Elixir.Patina.ReprTransparent.Inner').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export([]).
