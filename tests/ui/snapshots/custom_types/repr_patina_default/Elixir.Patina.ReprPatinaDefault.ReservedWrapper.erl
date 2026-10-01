-file("tests/ui/src/custom_types/repr_patina_default.pt",
      72).

-module('Elixir.Patina.ReprPatinaDefault.ReservedWrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([open]).

-export(['$patina_constructor__open'/1]).

-record #open{'0'}.

'$patina_constructor__open'(_0@0_0) ->
    #open{'0' = _0@0_0}.
