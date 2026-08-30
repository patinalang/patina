-file("tests/ui/src/generics/witness/dispatch.pt", 1).

-module('Elixir.Patina.Dispatch').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1, main/0]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) -> nil;
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

total_area(_T@0_0, _t@1_0) ->
    (erlang:element(1,
                    _T@0_0)):'Dispatch::Area::area'(_T@0_0, _t@1_0).

main() ->
    _p@0_0 =
        ('Elixir.Patina.Dispatch.Point':'__struct__'())#{x := 3,
                                                         y := 4},
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [total_area({'Elixir.Patina.Dispatch.Point',
                                                  {}},
                                                 _p@0_0)])]]),
        io:nl()
    end.
