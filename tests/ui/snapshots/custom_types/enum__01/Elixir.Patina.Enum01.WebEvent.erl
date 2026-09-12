-file("tests/ui/src/custom_types/enum__01.pt", 8).

-module('Elixir.Patina.Enum01.WebEvent').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Click']).

-export([]).

-record #'Click'{x, y}.
