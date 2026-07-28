ExUnit.start()

defmodule LoopOuterContinueTest do
  use ExUnit.Case

  alias Patina.LoopOuterContinue

  test "continue unwinds to its statically resolved target loop" do
    assert LoopOuterContinue.continue_outer_loop(3) == 303
    assert LoopOuterContinue.continue_two_loops(3) == 303
  end
end
