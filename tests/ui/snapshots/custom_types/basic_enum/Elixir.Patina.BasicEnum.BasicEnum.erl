-file("tests/ui/src/custom_types/basic_enum.pt", 1).

-module('Elixir.Patina.BasicEnum.BasicEnum').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Tuple', 'Struct']).

-export(['$patina_constructor__Tuple'/2,
         '$patina_constructor__Struct'/2]).

-record #'Tuple'{'0', '1'}.

'$patina_constructor__Tuple'(_0@0_0, _1@1_0) ->
    #'Tuple'{'0' = _0@0_0, '1' = _1@1_0}.

-record #'Struct'{x, y}.

'$patina_constructor__Struct'(_x@0_0, _y@1_0) ->
    #'Struct'{x = _x@0_0, y = _y@1_0}.
