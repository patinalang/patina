-file("tests/ui/src/custom_types/repr_patina_default.pt",
      65).

-module('Elixir.Patina.ReprPatinaDefault.WrapperConflict').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export(['$patina_constructor__Pair'/1]).

-record #'Pair'{'0'}.

'$patina_constructor__Pair'(_0@0_0) ->
    #'Pair'{'0' = _0@0_0}.
