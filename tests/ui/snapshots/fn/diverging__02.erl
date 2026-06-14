-file("tests/ui/src/fn/diverging__02.pt", 4).
-module('Elixir.Patina.Diverging02').
-compile([no_auto_import]).
-export([main/0]).
some_fn() ->
    {}.
main() ->
    _var_0 = 'Elixir.Patina.Diverging02.some_fn'(),
    begin
        io:put_chars([[<<"This function returns and you can see this li"
                         "ne.">>]]),
        io:nl()
    end.
