-file("tests/ui/src/custom_types/repr_recursive.pt",
      29).

-module('Elixir.Patina.ReprRecursive.RightFirst').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Next']).

-export(['$patina_constructor__Next'/1]).

-record #'Next'{'0'}.

'$patina_constructor__Next'(_0@0_0) ->
    #'Next'{'0' = _0@0_0}.
