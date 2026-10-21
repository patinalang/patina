-file("tests/ui/src/flow_control/let_chains.pt", 40).

-module('Elixir.Patina.LetChains.Shape').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['B']).

-export(['$patina_constructor__B'/1]).

-record #'B'{'0'}.

'$patina_constructor__B'(_0@0_0) -> #'B'{'0' = _0@0_0}.
