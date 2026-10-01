ExUnit.start()

defmodule RangePatternTest do
  use ExUnit.Case

  alias Patina.RangePattern
  alias Patina.RangePattern.Number

  test "integer ranges honor inclusive, exclusive, negative, and open bounds" do
    assert RangePattern.classify_int(-11) == 1
    assert RangePattern.classify_int(-10) == 2
    assert RangePattern.classify_int(-1) == 2
    assert RangePattern.classify_int(0) == 3
    assert RangePattern.classify_int(10) == 3
    assert RangePattern.classify_int(11) == 4
  end

  test "character ranges compare codepoints at their boundaries" do
    assert RangePattern.classify_char(?`) == 1
    assert RangePattern.classify_char(?a) == 2
    assert RangePattern.classify_char(?z) == 2
    assert RangePattern.classify_char(?{) == 3
  end

  test "float ranges honor inclusive and exclusive endpoints" do
    assert RangePattern.classify_float(-1.5) == 1
    assert RangePattern.classify_float(-1.49) == 2
    assert RangePattern.classify_float(1.49) == 2
    assert RangePattern.classify_float(1.5) == 3
  end

  test "or-patterns mix inclusive and exclusive ranges with a literal" do
    assert RangePattern.matches_either_range(1)
    assert RangePattern.matches_either_range(3)
    refute RangePattern.matches_either_range(6)
    assert RangePattern.matches_either_range(7)
    assert RangePattern.matches_either_range(8)
    refute RangePattern.matches_either_range(9)
    refute RangePattern.matches_either_range(10)
    assert RangePattern.matches_either_range(11)
    refute RangePattern.matches_either_range(12)
  end

  test "binding and explicit guards are combined with range guards" do
    assert RangePattern.match_bound_range(-2) == -20
    assert RangePattern.match_bound_range(2) == 20
    assert RangePattern.match_bound_range(-3) == 0
    assert RangePattern.match_bound_range(4) == 0
  end

  test "nested ranges carry guards through compound patterns" do
    assert RangePattern.match_nested_ranges({-2, ?m})
    assert RangePattern.match_nested_ranges({2, ?p})
    refute RangePattern.match_nested_ranges({-3, ?m})
    refute RangePattern.match_nested_ranges({2, ?q})
  end

  test "ranges work inside enum and map patterns" do
    assert RangePattern.match_enum_range(40)
    refute RangePattern.match_enum_range(41)
    assert RangePattern.match_enum_range(44)
    refute RangePattern.match_enum_range(45)

    assert RangePattern.match_map_range(%{"score" => 10})
    assert RangePattern.match_map_range(%{"score" => 19, "extra" => 1})
    refute RangePattern.match_map_range(%{"score" => 20})
    refute RangePattern.match_map_range(%{"other" => 15})
  end

  test "if-let and let-else lower range guards" do
    refute RangePattern.if_let_range(1)
    assert RangePattern.if_let_range(2)
    assert RangePattern.if_let_range(4)
    refute RangePattern.if_let_range(5)

    refute RangePattern.let_else_range(-2)
    assert RangePattern.let_else_range(-1)
    assert RangePattern.let_else_range(1)
    refute RangePattern.let_else_range(2)
  end
end
