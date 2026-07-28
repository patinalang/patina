ExUnit.start()

defmodule LoopOuterLabelTest do
  use ExUnit.Case

  alias Patina.LoopOuterLabel

  test "break unwinds to its statically resolved target loop" do
    assert LoopOuterLabel.break_outer_loop() == 1
    assert LoopOuterLabel.break_two_loops() == 30
    assert LoopOuterLabel.outer_break_initializes() == 7
  end
end
