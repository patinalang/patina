-file("tests/ui/src/flow_control/receive/unsafe_rece"
      "ive__01.pt",
      4).

-module('Elixir.Patina.UnsafeReceive01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

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
        receive
            _n@0_0 when erlang:is_integer(_n@0_0) ->
                io:put_chars([[<<"got integer ">>,
                               io_lib:format("~ktp", [_n@0_0])]]),
                io:nl(),
                {};
            _a@1_0 when erlang:is_atom(_a@1_0) ->
                io:put_chars([[<<"got atom ">>,
                               io_lib:format("~ktp", [_a@1_0])]]),
                io:nl(),
                {};
            nil ->
                io:put_chars([[<<"got an empty string option">>]]),
                io:nl(),
                {};
            nil ->
                io:put_chars([[<<"got an empty integer option">>]]),
                io:nl(),
                {};
            _ ->
                io:put_chars([[<<"got something else">>]]),
                io:nl(),
                {}
            after 'Elixir.Patina.Core.Timeout':from_seconds(5) ->
                      begin io:put_chars([[<<"timeout">>]]), io:nl(), {} end
        end
    end.
