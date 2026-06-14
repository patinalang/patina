-file("tests/ui/src/generics/bounds/testcase_empty__01.pt", 4).
-module('Elixir.Patina.TestcaseEmpty01').
-compile([no_auto_import]).
-export([main/0]).
red(_) ->
    <<"red">>.
blue(_) ->
    <<"blue">>.
main() ->
    _var_0 = 'Elixir.Patina.TestcaseEmpty01.Cardinal',
    _var_1 = 'Elixir.Patina.TestcaseEmpty01.BlueJay',
    _var_2 = 'Elixir.Patina.TestcaseEmpty01.Turkey',
    begin
        io:put_chars([[<<"A cardinal is ">>,
                       'Elixir.Patina.TestcaseEmpty01.red'(_var_0)]]),
        io:nl()
    end,
    begin
        io:put_chars([[<<"A blue jay is ">>,
                       'Elixir.Patina.TestcaseEmpty01.blue'(_var_1)]]),
        io:nl()
    end.
