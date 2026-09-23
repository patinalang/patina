ExUnit.start()

defmodule BitstringsTest do
  use ExUnit.Case

  alias Patina.Bitstrings

  test "bin literals" do
    assert Bitstrings.annotated() == "qwerty"
    assert Bitstrings.named() == "named"
  end

  test "escaped and raw bin literals" do
    assert Bitstrings.escaped() == <<0, 255, ?\n>>
    assert Bitstrings.raw() == "no \\escapes"
  end

  test "bin comparison" do
    assert Bitstrings.bin_equal(<<1, 2>>, <<1, 2>>)
    refute Bitstrings.bin_equal(<<1, 2>>, <<1, 3>>)
    assert Bitstrings.bin_less_than(<<1>>, <<2>>)
  end

  test "bin concatenation" do
    assert Bitstrings.bin_concat(<<1>>, <<255>>) == <<1, 255>>
  end

  test "bits pass through unchanged" do
    assert Bitstrings.bits_identity(<<5::3>>) == <<5::3>>
  end

  test "bits comparison" do
    assert Bitstrings.bits_equal(<<5::3>>, <<5::3>>)
    refute Bitstrings.bits_equal(<<5::3>>, <<5::4>>)
    assert Bitstrings.bits_less_than(<<0::1>>, <<1::1>>)
  end

  test "bits concatenation" do
    assert Bitstrings.bits_concat(<<5::3>>, <<1::1>>) == <<11::4>>
  end

  test "bin literal patterns" do
    assert Bitstrings.bin_classify(<<0x89, "PNG">>) == :png
    assert Bitstrings.bin_classify("GIF8") == :gif
    assert Bitstrings.bin_classify(<<0>>) == :unknown
  end

  test "bits literal patterns" do
    assert Bitstrings.bits_classify(<<255>>) == :ff
    assert Bitstrings.bits_classify("text") == :text
    assert Bitstrings.bits_classify(<<1::1>>) == :unknown
  end

  test "term class patterns" do
    assert Bitstrings.term_classes("x") == :bin_x
    assert Bitstrings.term_classes("y") == :bits_y
    assert Bitstrings.term_classes("z") == :bin
    assert Bitstrings.term_classes(<<1::1>>) == :bits
    assert Bitstrings.term_classes(:x) == :other
  end

  test "atom and bitstring literals are distinct patterns" do
    assert Bitstrings.term_atom_and_str(:a) == 1
    assert Bitstrings.term_atom_and_str("a") == 2
    assert Bitstrings.term_atom_and_str(<<255>>) == 3
    assert Bitstrings.bin_distinct("a") == 1
    assert Bitstrings.bin_distinct("ab") == 2
    assert Bitstrings.bin_distinct(<<255>>) == 3
  end

  test "str and bin widen to supertypes" do
    assert Bitstrings.str_to_bin("abc") == "abc"
    assert Bitstrings.str_to_bits("abc") == "abc"
    assert Bitstrings.bin_to_bits(<<0, 255>>) == <<0, 255>>
    assert Bitstrings.literal_to_bits() == <<1, 2>>
  end

  test "str arguments pass as bin" do
    assert Bitstrings.pass_str()
  end

  test "branches unify to the wider type" do
    assert Bitstrings.bin_branches(true) == "text"
    assert Bitstrings.bin_branches(false) == <<255>>
    assert Bitstrings.bin_match_arms(true) == <<255>>
    assert Bitstrings.bin_match_arms(false) == "text"
    assert Bitstrings.bits_branches(true, <<1::1>>) == "text"
    assert Bitstrings.bits_branches(false, <<1::1>>) == <<1::1>>
  end

  test "subtyping through slices, tuples and maps" do
    assert Bitstrings.str_slice(["a", "b"]) == ["a", "b"]
    assert Bitstrings.bin_slice([<<1>>, "a"]) == [<<1>>, "a"]
    assert Bitstrings.tuple({"a", 1}) == {"a", 1}
    assert Bitstrings.str_map(%{"k" => "v"}) == %{"k" => "v"}
    assert Bitstrings.bin_map(%{"k" => <<1>>}) == %{"k" => <<1>>}
  end

  test "bitstrings coerce to term" do
    assert Bitstrings.str_to_term("abc") == "abc"
    assert Bitstrings.bits_to_term(<<1::1>>) == <<1::1>>
  end
end
