-file("tests/ui/src/module_attr.pt", 9).

-module(module_attr_web_event).

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Click']).

-export([]).

-record #'Click'{x, y}.
