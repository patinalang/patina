-file("tests/ui/src/module_attr.pt", 9).

-module(module_attr_web_event).

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Click']).

-export(['$patina_constructor__Click'/2]).

-record #'Click'{x, y}.

'$patina_constructor__Click'(_x@0_0, _y@1_0) ->
    #'Click'{x = _x@0_0, y = _y@1_0}.
