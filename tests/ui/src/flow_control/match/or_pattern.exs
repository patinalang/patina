ExUnit.start()

defmodule OrPatternTest do
  use ExUnit.Case

  alias Patina.OrPattern

  test "or-pattern alternatives select the same match arm" do
    assert OrPattern.classify(1) == 10
    assert OrPattern.classify(3) == 10
    assert OrPattern.classify(5) == 10
    assert OrPattern.classify(2) == 20
    assert OrPattern.classify(4) == 20
    assert OrPattern.classify(0) == 0
  end

  test "nested or-patterns expand to every combination" do
    assert OrPattern.classify_nested({0, 2}) == 10
    assert OrPattern.classify_nested({0, 3}) == 10
    assert OrPattern.classify_nested({1, 2}) == 10
    assert OrPattern.classify_nested({1, 3}) == 10
    assert OrPattern.classify_nested({0, 4}) == 0
    assert OrPattern.classify_nested({2, 2}) == 0
  end

  test "nested or-patterns expand through map values" do
    assert OrPattern.classify_nested_map(%{"first" => 0, "second" => 2}) == 10
    assert OrPattern.classify_nested_map(%{"first" => 1, "second" => 3}) == 10
    assert OrPattern.classify_nested_map(%{"first" => 2, "second" => 2}) == 0
  end

  test "if-let accepts an or-pattern" do
    assert OrPattern.classify_if_let(1) == 10
    assert OrPattern.classify_if_let(3) == 10
    assert OrPattern.classify_if_let(5) == 10
    assert OrPattern.classify_if_let(0) == 0
  end

  test "if-let shares bindings between or-pattern alternatives" do
    assert OrPattern.unwrap_zero_pair_if_let({0, 7}) == 7
    assert OrPattern.unwrap_zero_pair_if_let({8, 0}) == 8
    assert OrPattern.unwrap_zero_pair_if_let({8, 9}) == -1
  end

  test "let-else accepts an or-pattern" do
    assert OrPattern.classify_let_else(1) == 10
    assert OrPattern.classify_let_else(3) == 10
    assert OrPattern.classify_let_else(5) == 10
    assert OrPattern.classify_let_else(0) == 0
  end
end
