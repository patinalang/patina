ExUnit.start()

defmodule LoopsTest do
  use ExUnit.Case

  alias Patina.Loops

  test "generated loop helpers are private in the final BEAM" do
    refute Enum.any?(Loops.module_info(:exports), fn {name, _arity} ->
             String.starts_with?(Atom.to_string(name), "$patina_loop_")
           end)
  end

  test "normal fallthrough recurses and continue skips the rest of an iteration" do
    assert Loops.normal_continue(5) == 905
  end

  test "loop values and rebound variables reach the caller" do
    assert Loops.loop_value(4) == 84
  end

  test "loops support no parameters and initialize outer bindings" do
    assert Loops.zero_parameter_loop() == 7
    assert Loops.initialize_in_loop() == 7
  end

  test "while threads state, including its zero-iteration path" do
    assert Loops.while_sum(1, 3) == 303
    assert Loops.while_sum(5, 5) == 5
  end

  test "nested loops keep independent break and continue targets" do
    assert Loops.nested(2, 3) == 3602
  end

  test "return propagates through generated loop helpers" do
    assert Loops.return_from_loop(4) == 40
    assert Loops.return_from_loop(20) == -1
    assert Loops.return_from_nested(2, 2) == 202
    assert Loops.return_from_nested(4, 4) == -1
    assert Loops.return_from_nested_uneven_rebinds(2, 2) == 2202
    assert Loops.return_from_nested_uneven_rebinds(4, 4) == -1
  end

  test "labeled continue returns a continue payload instead of recursing the current helper" do
    assert Loops.continue_outer(3) == 303
    assert Loops.continue_through_middle(3) == 303
  end

  test "a cast around a loop preserves the loop's break target" do
    assert Loops.cast_loop_value() == 1
  end
end
