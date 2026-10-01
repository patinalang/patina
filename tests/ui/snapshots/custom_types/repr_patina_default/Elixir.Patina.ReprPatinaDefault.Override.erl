-file("tests/ui/src/custom_types/repr_patina_default.pt",
      82).

-module('Elixir.Patina.ReprPatinaDefault.Override').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Record']).

-export(['$patina_constructor__Record'/1]).

-record #'Record'{'0'}.

'$patina_constructor__Record'(_0@0_0) ->
    #'Record'{'0' = _0@0_0}.
