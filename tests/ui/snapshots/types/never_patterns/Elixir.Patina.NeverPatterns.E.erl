-file("tests/ui/src/types/never_patterns.pt", 1).

-module('Elixir.Patina.NeverPatterns.E').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['A']).

-export(['$patina_constructor__A'/2]).

-record #'A'{'0', '1'}.

'$patina_constructor__A'(_0@0_0, _1@1_0) ->
    #'A'{'0' = _0@0_0, '1' = _1@1_0}.
