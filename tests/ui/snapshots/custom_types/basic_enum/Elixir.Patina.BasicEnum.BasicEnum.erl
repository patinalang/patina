-file("tests/ui/src/custom_types/basic_enum.pt", 1).

-module('Elixir.Patina.BasicEnum.BasicEnum').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Tuple', 'Struct']).

-export([]).

-record #'Tuple'{'0', '1'}.

-record #'Struct'{x, y}.
