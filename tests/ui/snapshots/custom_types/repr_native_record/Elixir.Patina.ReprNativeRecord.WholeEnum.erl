-file("tests/ui/src/custom_types/repr_native_record.pt",
      2).

-module('Elixir.Patina.ReprNativeRecord.WholeEnum').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Unit']).

-export([]).

-record #'Unit'{}.
