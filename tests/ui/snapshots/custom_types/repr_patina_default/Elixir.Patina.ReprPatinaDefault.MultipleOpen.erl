-file("tests/ui/src/custom_types/repr_patina_default.pt",
      51).

-module('Elixir.Patina.ReprPatinaDefault.MultipleOpen').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Second']).

-export(['$patina_constructor__Second'/1]).

-record #'Second'{'0'}.

'$patina_constructor__Second'(_0@0_0) ->
    #'Second'{'0' = _0@0_0}.
