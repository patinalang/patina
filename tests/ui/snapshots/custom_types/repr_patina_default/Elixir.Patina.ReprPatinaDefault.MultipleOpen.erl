-file("tests/ui/src/custom_types/repr_patina_default.pt",
      51).

-module('Elixir.Patina.ReprPatinaDefault.MultipleOpen').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Second']).

-export([]).

-record #'Second'{'0'}.
