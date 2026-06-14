-file("tests/ui/src/error/result__02.pt", 4).
-module('Elixir.Patina.Result02').
-compile([no_auto_import]).
-export([main/0]).
main() ->
    begin
        io:put_chars([[<<"Hello World!">>]]),
        io:nl()
    end.
