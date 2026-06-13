-file("tests/ui/src/mod/split__03.pt", 4).
-module('Elixir.Patina.Split03').
-compile([no_auto_import]).
-export([function/0]).
function() ->
    begin
        io:put_chars([[<<"called `MyMod::Nested::function()`">>]]),
        io:nl()
    end.
private_function() ->
    begin
        io:put_chars([[<<"called `MyMod::Nested::private_function()`">>]]),
        io:nl()
    end.
