-file("tests/ui/src/custom_types/repr_patina_default.pt",
      58).

-module('Elixir.Patina.ReprPatinaDefault.LaterExplicit').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Inferred']).

-export(['$patina_constructor__Inferred'/1]).

-record #'Inferred'{'0'}.

'$patina_constructor__Inferred'(_0@0_0) ->
    #'Inferred'{'0' = _0@0_0}.
