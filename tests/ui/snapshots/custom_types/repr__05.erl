-file("tests/ui/src/custom_types/repr__05.pt", 6).
-module('Elixir.Patina.Repr05').
-compile([no_auto_import]).
-export([main/0]).
main() ->
    begin
        io:put_chars([[io_lib:format("~tp",
                                     ['Elixir.Patina.Repr05.Celcius':'__struct__'(2.345)])]]),
        io:nl()
    end.
