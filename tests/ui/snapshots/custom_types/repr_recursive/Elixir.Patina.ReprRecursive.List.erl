-file("tests/ui/src/custom_types/repr_recursive.pt", 3).

-module('Elixir.Patina.ReprRecursive.List').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Cons']).

-export(['$patina_constructor__Cons'/2]).

-record #'Cons'{'0', '1'}.

'$patina_constructor__Cons'(_0@0_0, _1@1_0) ->
    #'Cons'{'0' = _0@0_0, '1' = _1@1_0}.
