-file("tests/ui/src/flow_control/if_let__03.pt", 5).

-module('Elixir.Patina.IfLet03.Foo').

-compile([no_auto_import]).

-export_record(['Bar', 'Baz', 'Qux']).

-export([]).

-record #'Bar'{}.

-record #'Baz'{}.

-record #'Qux'{'0'}.
