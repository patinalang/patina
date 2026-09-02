-file("tests/ui/src/custom_types/repr__09.pt", 5).

-module('Elixir.Patina.Repr09.TransparentWithTags').

-compile([no_auto_import]).

-export_record(['Untagged', 'MaybeUntagged', 'Tagged']).

-export([]).

-record #'Untagged'{'0'}.

-record #'MaybeUntagged'{'0'}.

-record #'Tagged'{'0'}.
