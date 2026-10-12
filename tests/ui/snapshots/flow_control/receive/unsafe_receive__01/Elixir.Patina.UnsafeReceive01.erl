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
            _n@1_0 when erlang:is_integer(_n@1_0) ->
                io:put_chars(begin
                                 _element@4_0 = begin
                                                    _element@2_0 =
                                                        <<"got integer "/utf8>>,
                                                    _element@3_0 =
                                                        io_lib:format("~ktp",
                                                                      [_n@1_0]),
                                                    [_element@2_0, _element@3_0]
                                                end,
                                 [_element@4_0]
                             end),
                io:nl(),
                {};
            _a@5_0 when erlang:is_atom(_a@5_0) ->
                io:put_chars(begin
                                 _element@8_0 = begin
                                                    _element@6_0 =
                                                        <<"got atom "/utf8>>,
                                                    _element@7_0 =
                                                        io_lib:format("~ktp",
                                                                      [_a@5_0]),
                                                    [_element@6_0, _element@7_0]
                                                end,
                                 [_element@8_0]
                             end),
                io:nl(),
                {};
            nil ->
                io:put_chars(begin
                                 _element@9_0 =
                                     [<<"got an empty string option"/utf8>>],
                                 [_element@9_0]
                             end),
                io:nl(),
                {};
            nil ->
                io:put_chars(begin
                                 _element@10_0 =
                                     [<<"got an empty integer option"/utf8>>],
                                 [_element@10_0]
                             end),
                io:nl(),
                {};
            _ ->
                io:put_chars(begin
                                 _element@11_0 =
                                     [<<"got something else"/utf8>>],
                                 [_element@11_0]
                             end),
                io:nl(),
                {}
            after 'Elixir.Patina.Core.Timeout':from_seconds(5) ->
                      begin
                          io:put_chars(begin
                                           _element@0_0 = [<<"timeout"/utf8>>],
                                           [_element@0_0]
                                       end),
                          io:nl(),
                          {}
                      end
        end
    end.
