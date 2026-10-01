-file("tests/ui/src/custom_types/repr_recursive.pt", 8).

-module('Elixir.Patina.ReprRecursive.Tree').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Branch']).

-export(['$patina_constructor__Branch'/2]).

-record #'Branch'{'0', '1'}.

'$patina_constructor__Branch'(_0@0_0, _1@1_0) ->
    #'Branch'{'0' = _0@0_0, '1' = _1@1_0}.
