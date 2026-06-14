-file("tests/ui/src/macros__01.pt", 5).
-module('Elixir.Patina.Macros01').
-compile([no_auto_import]).
-export([main/0]).
main() ->
    begin
        io:put_chars([[<<"Hello!">>]]),
        io:nl()
    end.
