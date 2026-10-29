-file("tests/ui/src/flow_control/match/binary_matchi"
      "ng__05.pt",
      4).

-module('Elixir.Patina.BinaryMatching05').

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
    _png_prefix@0_0 = <<"PNG"/utf8>>,
    _jpg_prefix@1_0 = <<"JPG"/utf8>>,
    case <<"PNG!rest"/utf8>> of
        <<_png_prefix@0_0:(erlang:byte_size(_png_prefix@0_0))/binary,
          _rest@2_0/binary>> ->
            io:put_chars(begin
                             _element@4_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_rest@2_0]),
                                                [<<"png data "/utf8>>,
                                                 _element@3_0]
                                            end,
                             [_element@4_0]
                         end),
            io:nl(),
            {};
        <<_jpg_prefix@1_0:(erlang:byte_size(_jpg_prefix@1_0))/binary,
          _rest@5_0/binary>> ->
            io:put_chars(begin
                             _element@7_0 = begin
                                                _element@6_0 =
                                                    io_lib:format("~ktp",
                                                                  [_rest@5_0]),
                                                [<<"jpg data "/utf8>>,
                                                 _element@6_0]
                                            end,
                             [_element@7_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"unknown data"/utf8>>]]),
            io:nl(),
            {}
    end,
    _bit_prefix@8_0 = 7,
    case <<119:8/integer>> of
        <<_bit_prefix@8_0:16/integer-little,
          _rest@9_0/binary>> ->
            io:put_chars(begin
                             _element@11_0 = begin
                                                 _element@10_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@9_0]),
                                                 [<<"expected "/utf8>>,
                                                  _element@10_0]
                                             end,
                             [_element@11_0]
                         end),
            io:nl(),
            {};
        <<_bit_prefix@8_0:4/integer, _rest@12_0/bitstring>> ->
            io:put_chars(begin
                             _element@14_0 = begin
                                                 _element@13_0 =
                                                     io_lib:format("~ktp",
                                                                   [_rest@12_0]),
                                                 [<<"expected "/utf8>>,
                                                  _element@13_0]
                                             end,
                             [_element@14_0]
                         end),
            io:nl(),
            {};
        _ ->
            io:put_chars([[<<"unexpected prefix"/utf8>>]]),
            io:nl(),
            {}
    end.
