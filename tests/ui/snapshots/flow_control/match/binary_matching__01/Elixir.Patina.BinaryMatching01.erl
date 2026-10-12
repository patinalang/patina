-file("tests/ui/src/flow_control/match/binary_matchi"
      "ng__01.pt",
      4).

-module('Elixir.Patina.BinaryMatching01').

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
    _payload@0_0 = <<1:8/integer, 17:8/integer,
                     42:8/integer, 99:8/integer>>,
    case _payload@0_0 of
        <<_prefix@1_0:16/integer, _tag@2_0:8/integer,
          _rest@3_0/binary>> ->
            io:put_chars(begin
                             _element@10_0 = begin
                                                 _element@4_0 =
                                                     <<"prefix = "/utf8>>,
                                                 _element@5_0 =
                                                     io_lib:format("~ktp",
                                                                   [_prefix@1_0]),
                                                 _element@6_0 =
                                                     <<", tag = "/utf8>>,
                                                 _element@7_0 =
                                                     io_lib:format("~ktp",
                                                                   [_tag@2_0]),
                                                 _element@8_0 =
                                                     <<", rest = "/utf8>>,
                                                 _element@9_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@3_0]),
                                                 [_element@4_0,
                                                  _element@5_0,
                                                  _element@6_0,
                                                  _element@7_0,
                                                  _element@8_0,
                                                  _element@9_0]
                                             end,
                             [_element@10_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars(begin
                             _element@11_0 = [<<"did not match"/utf8>>],
                             [_element@11_0]
                         end),
            io:nl(),
            {}
    end.
