-file("tests/ui/src/custom_types/repr__05.pt", 5).

-module('Elixir.Patina.Repr05.TaggedEnum').

-compile([no_auto_import]).

-export_record(['TupleStyle',
                'StructStyle',
                'WithCustomTag']).

-export([]).

-record #'TupleStyle'{'0'}.

-record #'StructStyle'{x, y}.

-record #'WithCustomTag'{'0'}.
