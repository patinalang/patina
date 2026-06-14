-file("tests/ui/src/custom_types/repr__05.pt", 6).
-module('Elixir.Patina.Repr05').
-compile([no_auto_import]).
-export([main/0]).
main() ->
    begin
        io:put_chars([[io_lib:format("~tp",
                                     [('Elixir.Patina.Repr05.Celcius':'__struct__'())#{'0' :=
                                                                                          2.345}])]]),
        io:nl()
    end.

-file("tests/ui/src/custom_types/repr__05.pt", 6).
-module('Elixir.Patina.Repr05.Celcius').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Repr05.Celcius', '0' => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
