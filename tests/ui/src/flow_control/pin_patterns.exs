ExUnit.start()

defmodule PinPatternsTest do
  use ExUnit.Case

  alias Patina.PinPatterns

  test "pin patterns select match and mismatch paths" do
    assert PinPatterns.match_pin(7, 7)
    refute PinPatterns.match_pin(7, 8)
    assert PinPatterns.if_let_pin(7, 7)
    refute PinPatterns.if_let_pin(7, 8)
    assert PinPatterns.while_let_pin(7, 7) == 18
    assert PinPatterns.while_let_pin(7, 8) == 7
    assert PinPatterns.let_else_pin(7, 7)
    refute PinPatterns.let_else_pin(7, 8)
  end

  test "pins compose with or, map, and alias patterns" do
    assert PinPatterns.or_pin(7, 7, 8)
    assert PinPatterns.or_pin(8, 7, 8)
    refute PinPatterns.or_pin(9, 7, 8)
    assert PinPatterns.map_value_pin(%{"key" => 7}, 7)
    refute PinPatterns.map_value_pin(%{"key" => 8}, 7)
    assert PinPatterns.alias_pin(7, 7) == 7
    assert PinPatterns.alias_pin(8, 7) == -1
  end

  test "closures and loop helpers capture the current pinned value" do
    assert PinPatterns.closure_pin(7, 7)
    refute PinPatterns.closure_pin(7, 8)
    assert PinPatterns.mutable_closure_pin(8, 7)
    refute PinPatterns.mutable_closure_pin(7, 7)
    assert PinPatterns.loop_rebound_pin(4) == 4
  end

  test "a sibling binding does not shadow the outer pin operand" do
    assert PinPatterns.sibling_shadow_pin(10, 7, 7) == 10
    assert PinPatterns.sibling_shadow_pin(10, 8, 7) == -1
  end

  test "pins work with guards and pat macro fragments" do
    assert PinPatterns.guarded_pin(7, 7, true)
    refute PinPatterns.guarded_pin(7, 7, false)
    assert PinPatterns.macro_pin(7, 7)
    refute PinPatterns.macro_pin(8, 7)
  end
end
