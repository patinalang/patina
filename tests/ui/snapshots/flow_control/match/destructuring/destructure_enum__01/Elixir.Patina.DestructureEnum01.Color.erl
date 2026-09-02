-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_enum__01.pt",
      7).

-module('Elixir.Patina.DestructureEnum01.Color').

-compile([no_auto_import]).

-export_record(['RGB', 'HSV', 'HSL', 'CMY', 'CMYK']).

-export([]).

-record #'RGB'{'0', '1', '2'}.

-record #'HSV'{'0', '1', '2'}.

-record #'HSL'{'0', '1', '2'}.

-record #'CMY'{'0', '1', '2'}.

-record #'CMYK'{'0', '1', '2', '3'}.
