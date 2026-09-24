ExUnit.start()

defmodule BinaryPatternTest do
  use ExUnit.Case

  alias Patina.BinaryPattern, as: B

  test "segments take their type from their value by default" do
    assert B.utf8_segments_make_str("Bob", ?é) == "Hi, Bobé!"

    assert B.whole_bytes_make_bin(300, ?é) ==
             <<300::16, ?é::utf16, ?é::utf32-little, 1.5::float-32, 255>>

    assert B.partial_bytes_make_bits(300) == <<1::3, 301::size(2)-unit(4)>>
    assert B.numbers() == <<-1::signed-8, 1.5::float-16, 2.5::float-32-little, 1000::16-little>>
    assert B.nested() == <<1, 2, 3>>
  end

  test "<< is a shift after an operand, and >> closes a binary pattern" do
    assert B.shifts(8) == {32, <<2, 64>>, true}
    assert B.first_byte(<<5>>) == 5
    assert B.first_byte(<<5, 6>>) == 6
    assert B.first_byte(<<>>) == 0
    assert B.captured(<<5>>) == {5, -1, {<<1>>, 2}}
    assert B.captured(<<5, 6>>) == {-1, 6, {<<1>>, 2}}
  end

  test "binary patterns match terms, signed ints, floats, chars and units" do
    assert B.term_matches_like_bits(<<1, 2>>) == {1, <<2>>}
    assert B.term_matches_like_bits(<<1::4>>) == {100, <<1::1>>}
    assert B.term_matches_like_bits(5) == {-1, <<>>}

    assert B.decode(<<255, 1.5::float-16, 0, 0>>) == {-1, 1.5, <<0, 0>>}
    # The rest isn't a multiple of 16 bits.
    assert B.decode(<<255, 1.5::float-16, 0>>) == {0, 0.0, <<>>}

    assert B.chars(<<?é::utf16, ?é::utf32-little, 7>>) == {?é, ?é, <<7>>}
    assert B.bits_unit_is_binary_unit(<<1::4, 2::12>>) == {<<1::4>>, <<2::12>>}
    assert B.pinned_bitstring_matches_exactly(<<1, "ok">>, "ok")
    refute B.pinned_bitstring_matches_exactly(<<1, "ok!">>, "ok")
    refute B.pinned_bitstring_matches_exactly(<<1, "o">>, "ok")
    assert B.bits_unit_is_binary_unit(<<1::6>>) == {<<>>, <<>>}
  end

  test "string literals can be encoded as UTF-16 or UTF-32" do
    assert B.encoded_strings_make_bin() == <<"łącko"::utf16, "ok"::utf32-little>>
    assert B.encoded_prefix(B.encoded_strings_make_bin()) == <<"cko"::utf16, "ok"::utf32-little>>
    assert B.encoded_prefix("łącko") == <<>>
  end

  test "the rest after UTF-8 segments is a str" do
    assert B.str_rest("éłąk") == "ąk"
    assert B.str_rest("abc") == ""
  end

  test "the rest after whole bytes is a bin" do
    assert B.bin_rest("ł!") == <<0x82, ?!>>
  end

  test "the rest after a partial byte is a bits" do
    assert B.bits_rest(<<0xAB, 0xCD>>) == <<0xBCD::12>>
    assert B.term_rest(<<1, 2::4>>) == <<2::4>>
    assert B.term_rest(:atom) == <<>>
  end

  test "the rest after a sized segment" do
    assert B.split(<<2, 1, 2, 3, 4>>) == {<<1, 2>>, <<3, 4>>}
    assert B.split(<<5, 1>>) == {<<>>, <<>>}
  end

  test "a mutable rest" do
    assert B.rotate(<<1, 2, 3>>) == <<2, 3, 1>>
    assert B.rotate(<<>>) == <<>>
  end

  test "an ignored rest" do
    assert B.has_magic("PT and more")
    assert B.has_magic("PT")
    refute B.has_magic("P")
  end
end
