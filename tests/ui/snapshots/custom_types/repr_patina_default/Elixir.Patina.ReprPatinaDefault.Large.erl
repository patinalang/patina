-file("tests/ui/src/custom_types/repr_patina_default.pt",
      43).

-module('Elixir.Patina.ReprPatinaDefault.Large').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Open', 'Pair']).

-export(['$patina_constructor__Open'/1,
         '$patina_constructor__Pair'/2]).

-record #'Open'{'0'}.

'$patina_constructor__Open'(_0@0_0) ->
    #'Open'{'0' = _0@0_0}.

-record #'Pair'{'0', '1'}.

'$patina_constructor__Pair'(_0@0_0, _1@1_0) ->
    #'Pair'{'0' = _0@0_0, '1' = _1@1_0}.
