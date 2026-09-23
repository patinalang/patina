-file("tests/ui/src/primitives/bitstrings.pt", 3).

-module('Elixir.Patina.Bitstrings').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         annotated/0,
         named/0,
         escaped/0,
         raw/0,
         bin_equal/2,
         bin_less_than/2,
         bin_concat/2,
         bits_identity/1,
         bits_equal/2,
         bits_less_than/2,
         bits_concat/2,
         bin_classify/1,
         bits_classify/1,
         term_classes/1,
         str_to_bin/1,
         str_to_bits/1,
         bin_to_bits/1,
         literal_to_bits/0,
         pass_str/0,
         bin_branches/1,
         bin_match_arms/1,
         bits_branches/2,
         str_slice/1,
         bin_slice/1,
         tuple/1,
         str_map/1,
         bin_map/1,
         str_to_term/1,
         bits_to_term/1,
         term_str_then_bin/1,
         term_duplicate_str/1,
         bin_str_then_bin/1,
         bits_bin_then_str/1,
         term_atom_and_str/1,
         bin_distinct/1]).

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

annotated() ->
    _value@0_0 = <<113, 119, 101, 114, 116, 121>>,
    _value@0_0.

named() -> <<110, 97, 109, 101, 100>>.

escaped() -> <<0, 255, 10>>.

raw() ->
    <<110, 111, 32, 92, 101, 115, 99, 97, 112, 101, 115>>.

bin_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

bin_less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.

bin_concat(_left@0_0, _right@1_0) ->
    <<_left@0_0/binary, _right@1_0/binary>>.

bits_identity(_value@0_0) -> _value@0_0.

bits_equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.

bits_less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.

bits_concat(_left@0_0, _right@1_0) ->
    <<_left@0_0/bitstring, _right@1_0/bitstring>>.

bin_classify(_value@0_0) ->
    case _value@0_0 of
        <<137, 80, 78, 71>> -> png;
        <<71, 73, 70, 56>> -> gif;
        _ -> unknown
    end.

bits_classify(_value@0_0) ->
    case _value@0_0 of
        <<255>> -> ff;
        <<116, 101, 120, 116>> -> text;
        _ -> unknown
    end.

term_classes(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = <<120>>
            when erlang:is_binary(_value@1_0) ->
            bin_x;
        _value@2_0 = <<121>>
            when erlang:is_bitstring(_value@2_0) ->
            bits_y;
        _value@3_0 when erlang:is_binary(_value@3_0) -> bin;
        _value@4_0 when erlang:is_bitstring(_value@4_0) -> bits;
        _ -> other
    end.

str_to_bin(_value@0_0) -> _value@0_0.

str_to_bits(_value@0_0) -> _value@0_0.

bin_to_bits(_value@0_0) -> _value@0_0.

literal_to_bits() -> <<1, 2>>.

pass_str() ->
    bin_equal(<<"same">>, <<115, 97, 109, 101>>).

bin_branches(_flag@0_0) ->
    case _flag@0_0 of
        true -> <<"text">>;
        false -> <<255>>
    end.

bin_match_arms(_flag@0_0) ->
    case _flag@0_0 of
        true -> <<255>>;
        false -> <<"text">>
    end.

bits_branches(_flag@0_0, _value@1_0) ->
    case _flag@0_0 of
        true -> <<"text">>;
        false -> _value@1_0
    end.

str_slice(_values@0_0) -> _values@0_0.

bin_slice(_values@0_0) -> _values@0_0.

tuple(_value@0_0) -> _value@0_0.

str_map(_values@0_0) -> _values@0_0.

bin_map(_values@0_0) -> _values@0_0.

str_to_term(_value@0_0) ->
    _widened@1_0 = _value@0_0,
    _widened@1_0.

bits_to_term(_value@0_0) -> _value@0_0.

term_str_then_bin(_value@0_0) ->
    case _value@0_0 of
        <<97>> -> 1;
        <<97>> -> 2;
        _ -> 0
    end.

term_duplicate_str(_value@0_0) ->
    case _value@0_0 of
        <<97>> -> 1;
        <<97>> -> 2;
        _ -> 0
    end.

bin_str_then_bin(_value@0_0) ->
    case _value@0_0 of
        <<97>> -> 1;
        <<97>> -> 2;
        _ -> 0
    end.

bits_bin_then_str(_value@0_0) ->
    case _value@0_0 of
        <<97>> -> 1;
        <<97>> -> 2;
        _ -> 0
    end.

term_atom_and_str(_value@0_0) ->
    case _value@0_0 of
        a -> 1;
        <<97>> -> 2;
        <<255>> -> 3;
        _ -> 0
    end.

bin_distinct(_value@0_0) ->
    case _value@0_0 of
        <<97>> -> 1;
        <<97, 98>> -> 2;
        <<255>> -> 3;
        _ -> 0
    end.
