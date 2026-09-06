-file("tests/ui/src/custom_types/repr_native_record.pt",
      6).

-module('Elixir.Patina.ReprNativeRecord.MixedEnum').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['RecordUnit']).

-export([]).

-record #'RecordUnit'{}.
