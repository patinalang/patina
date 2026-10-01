-file("tests/ui/src/custom_types/repr_patina_default.pt",
      36).

-module('Elixir.Patina.ReprPatinaDefault.Small').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Pair']).

-export(['$patina_constructor__Pair'/2]).

-record #'Pair'{'0', '1'}.

'$patina_constructor__Pair'(_0@0_0, _1@1_0) ->
    #'Pair'{'0' = _0@0_0, '1' = _1@1_0}.
