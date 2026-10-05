ExUnit.start()

defmodule LoopWitnessTest do
  use ExUnit.Case

  alias Patina.LoopWitness

  test "a loop in a default body uses the Self witness" do
    assert LoopWitness.loop_in_default(3, 4) == 12
  end

  test "a loop in a trait impl method uses the where-clause witnesses" do
    assert LoopWitness.pair(2, 5) == 7
    assert LoopWitness.pair_repeated(2, 5, 3) == 21
  end

  test "a nested loop uses the function's bound witnesses" do
    assert LoopWitness.nested_rounds(4, 3) == 12
  end

  test "a loop in a closure uses the enclosing function's witnesses" do
    assert LoopWitness.closure_loop(6, 2) == 12
  end
end
