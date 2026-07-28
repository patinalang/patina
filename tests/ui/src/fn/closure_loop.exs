ExUnit.start()

defmodule ClosureLoopTest do
  use ExUnit.Case

  test "a loop in a closure works correctly" do
    assert Patina.ClosureLoop.closure_loop() == 4
  end
end
