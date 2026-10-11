-file("tests/ui/src/flow_control/match/binary_patter"
      "n.pt",
      1).

-module('Elixir.Patina.BinaryPattern').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         utf8_segments_make_str/2,
         whole_bytes_make_bin/2,
         partial_bytes_make_bits/1,
         numbers/0,
         nested/0,
         shifts/1,
         first_byte/1,
         captured/1,
         term_matches_like_bits/1,
         decode/1,
         chars/1,
         encoded_strings_make_bin/0,
         encoded_prefix/1,
         pinned_bitstring_matches_exactly/2,
         bits_unit_is_binary_unit/1,
         str_rest/1,
         bin_rest/1,
         bits_rest/1,
         term_rest/1,
         split/1,
         rotate/1,
         has_magic/1,
         main/0]).

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

utf8_segments_make_str(_name@0_0, _c@1_0) ->
    <<"Hi, "/utf8, _name@0_0/binary, _c@1_0/utf8,
      33:8/integer>>.

whole_bytes_make_bin(_n@0_0, _c@1_0) ->
    <<_n@0_0:16/integer, _c@1_0/utf16, _c@1_0/utf32-little,
      1.5:32/float, 255>>.

partial_bytes_make_bits(_n@0_0) ->
    <<1:3/integer, (_n@0_0 + 1):2/integer-unit:4>>.

numbers() ->
    <<(-1):8/integer-signed, 1.5:16/float,
      2.5:32/float-little, 1000:16/integer-little>>.

nested() ->
    <<<<1:8/integer, 2:8/integer>>/binary, <<>>/binary,
      3:8/integer>>.

shifts(_a@0_0) ->
    _shifted@1_0 = _a@0_0 bsl 2,
    _b@2_0 = <<(_a@0_0 bsr 2):8/integer,
               (_a@0_0 bsl 3):8/integer>>,
    {_shifted@1_0,
     _b@2_0,
     <<1:8/integer>> =:= <<1:8/integer>>}.

first_byte(_b@0_0) ->
    case _b@0_0 of
        <<_x@1_0:8/integer>> -> _x@1_0;
        <<_x@2_0:8/integer, _:8/integer>> -> _x@2_0 + 1;
        _ -> 0
    end.

captured(_b@0_0) ->
    {case _b@0_0 of
         <<_x@1_0:8/integer>> -> _x@1_0;
         _ -> -1
     end,
     case _b@0_0 of
         <<_:8/integer, _x@2_0:8/integer>> -> _x@2_0;
         _ -> -1
     end,
     {<<1:8/integer>>, 2}}.

term_matches_like_bits(_t@0_0) ->
    case _t@0_0 of
        <<_a@1_0:8/integer, _rest@2_0/binary>> ->
            {_a@1_0, _rest@2_0};
        <<_a@3_0:3/integer, _rest@4_0/bitstring>> ->
            {_a@3_0 + 100, _rest@4_0};
        _ -> {-1, <<>>}
    end.

decode(_b@0_0) ->
    case _b@0_0 of
        <<_x@1_0:8/integer-signed, _f@2_0:16/float,
          _rest@3_0/binary-unit:16>> ->
            {_x@1_0, _f@2_0, _rest@3_0};
        _ -> {0, 0.0, <<>>}
    end.

chars(_b@0_0) ->
    case _b@0_0 of
        <<_a@1_0/utf16, _b@2_0/utf32-little,
          _rest@3_0/binary>> ->
            {_a@1_0, _b@2_0, _rest@3_0};
        _ -> {$?, $?, <<>>}
    end.

encoded_strings_make_bin() ->
    <<"łącko"/utf16, "ok"/utf32-little>>.

encoded_prefix(_b@0_0) ->
    case _b@0_0 of
        <<"łą"/utf16, _rest@1_0/binary>> -> _rest@1_0;
        _ -> <<>>
    end.

pinned_bitstring_matches_exactly(_b@0_0, _tag@1_0) ->
    case _b@0_0 of
        <<1:8/integer,
          _tag@1_0:(erlang:byte_size(_tag@1_0))/binary>> ->
            true;
        _ -> false
    end.

bits_unit_is_binary_unit(_b@0_0) ->
    case _b@0_0 of
        <<_first@1_0:1/binary-unit:4,
          _rest@2_0/binary-unit:4>> ->
            {_first@1_0, _rest@2_0};
        _ -> {<<>>, <<>>}
    end.

str_rest(_s@0_0) ->
    case _s@0_0 of
        <<"é"/utf8, _/utf8, _rest@1_0/binary>> -> _rest@1_0;
        _ -> <<""/utf8>>
    end.

bin_rest(_s@0_0) ->
    case _s@0_0 of
        <<_:8/integer, _rest@1_0/binary>> -> _rest@1_0;
        _ -> <<>>
    end.

bits_rest(_b@0_0) ->
    case _b@0_0 of
        <<_:4/integer, _rest@1_0/bitstring>> -> _rest@1_0;
        _ -> <<>>
    end.

term_rest(_t@0_0) ->
    case _t@0_0 of
        <<_:8/integer, _rest@1_0/bitstring>> -> _rest@1_0;
        _ -> <<>>
    end.

split(_b@0_0) ->
    case _b@0_0 of
        <<_len@1_0:8/integer, _payload@2_0:_len@1_0/binary,
          _rest@3_0/binary>> ->
            {_payload@2_0, _rest@3_0};
        _ -> {<<>>, <<>>}
    end.

rotate(_b@0_0) ->
    case _b@0_0 of
        <<_first@1_0:8/integer, _rest@2_0/binary>> ->
            _rest@2_1 = <<_rest@2_0/binary, _first@1_0:8/integer>>,
            _rest@2_1;
        _ -> _b@0_0
    end.

has_magic(_b@0_0) ->
    case _b@0_0 of
        <<"PT"/utf8, _/binary>> -> true;
        _ -> false
    end.

main() -> {}.
