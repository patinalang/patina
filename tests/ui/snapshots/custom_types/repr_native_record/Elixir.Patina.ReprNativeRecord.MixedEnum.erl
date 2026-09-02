-file("tests/ui/src/custom_types/repr_native_record.pt",
      6).

-module('Elixir.Patina.ReprNativeRecord.MixedEnum').

-compile([no_auto_import]).

-export_record(['RecordUnit']).

-export([]).

-record #'RecordUnit'{}.
