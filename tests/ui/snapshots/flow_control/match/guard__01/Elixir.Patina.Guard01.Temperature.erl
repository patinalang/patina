-file("tests/ui/src/flow_control/match/guard__01.pt",
      5).

-module('Elixir.Patina.Guard01.Temperature').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Fahrenheit']).

-export([]).

-record #'Fahrenheit'{'0'}.
