-file("tests/ui/src/mod/split__04.pt", 4).
-module('Elixir.Patina.Split04').
-compile([no_auto_import]).
-export([public_function/0]).
public_function() ->
    begin
        io:put_chars([[<<"called `MyMod::Inaccessible::public_function("
                         ")`">>]]),
        io:nl()
    end.
