-file("tests/ui/src/macros/variadics__01.pt", 4).

-module('Elixir.Patina.Variadics01').

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

main() ->
    begin
        begin
            _val@0_0 = 1 + 2,
            begin
                io:put_chars([[<<"1 + 2">>,
                               <<" = ">>,
                               io_lib:format("~ktp", [_val@0_0])]]),
                io:nl()
            end
        end,
        begin
            begin
                _val@1_0 = 3 + 4,
                begin
                    io:put_chars([[<<"3 + 4">>,
                                   <<" = ">>,
                                   io_lib:format("~ktp", [_val@1_0])]]),
                    io:nl()
                end
            end,
            begin
                _val@2_0 = 2 * 3 + 1,
                begin
                    io:put_chars([[<<"(2 * 3) + 1">>,
                                   <<" = ">>,
                                   io_lib:format("~ktp", [_val@2_0])]]),
                    io:nl()
                end
            end
        end
    end.
