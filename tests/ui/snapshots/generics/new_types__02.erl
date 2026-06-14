-file("tests/ui/src/generics/new_types__02.pt", 4).
-module('Elixir.Patina.NewTypes02').
-compile([no_auto_import]).
-export([main/0]).
main() ->
    _var_0 =
        ('Elixir.Patina.NewTypes02.Years':'__struct__'())#{'0' := 42},
    _var_1 = map_get('0', _var_0),
    #{'0' := _var_2} = _var_0.

-file("tests/ui/src/generics/new_types__02.pt", 4).
-module('Elixir.Patina.NewTypes02.Years').
-compile([no_auto_import]).
-export(['__struct__'/0,'__struct__'/1]).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.NewTypes02.Years', '0' => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
