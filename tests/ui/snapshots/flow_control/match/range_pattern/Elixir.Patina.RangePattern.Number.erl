-file("tests/ui/src/flow_control/match/range_pattern.pt",
      1).

-module('Elixir.Patina.RangePattern.Number').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Value']).

-export([]).

-record #'Value'{'0'}.
