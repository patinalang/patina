-file("tests/ui/src/custom_types/enum/enum_use__01.pt",
      12).

-module('Elixir.Patina.EnumUse01.Role').

-compile([no_auto_import]).

-export_record(['Student', 'Teacher']).

-export([]).

-record #'Student'{}.

-record #'Teacher'{}.
