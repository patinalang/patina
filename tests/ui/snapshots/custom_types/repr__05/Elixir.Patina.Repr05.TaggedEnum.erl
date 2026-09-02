-file("tests/ui/src/custom_types/repr__05.pt", 5).

-module('Elixir.Patina.Repr05.TaggedEnum').

-compile([no_auto_import]).

-export_record(['Unit',
                'TupleStyle',
                'StructStyle',
                'UnitWithAtomOverride',
                'WithCustomTag']).

-export([]).

-record #'Unit'{}.

-record #'TupleStyle'{'0'}.

-record #'StructStyle'{x, y}.

-record #'UnitWithAtomOverride'{}.

-record #'WithCustomTag'{'0'}.
