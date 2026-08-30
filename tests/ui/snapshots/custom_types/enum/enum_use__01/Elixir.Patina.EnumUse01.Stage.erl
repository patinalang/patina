-file("tests/ui/src/custom_types/enum/enum_use__01.pt",
      7).

-module('Elixir.Patina.EnumUse01.Stage').

-compile([no_auto_import]).

-export_record(['Beginner', 'Advanced']).

-export([]).

-record #'Beginner'{}.

-record #'Advanced'{}.
