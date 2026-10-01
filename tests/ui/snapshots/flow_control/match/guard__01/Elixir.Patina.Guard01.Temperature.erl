-file("tests/ui/src/flow_control/match/guard__01.pt",
      5).

-module('Elixir.Patina.Guard01.Temperature').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Fahrenheit']).

-export(['$patina_constructor__Fahrenheit'/1]).

-record #'Fahrenheit'{'0'}.

'$patina_constructor__Fahrenheit'(_0@0_0) ->
    #'Fahrenheit'{'0' = _0@0_0}.
