ExUnit.start()

defmodule AssocBoundTest do
  use ExUnit.Case

  test "a declared bound of an associated type dispatches to the impl for a primitive" do
    assert "an int" == Patina.AssocBound.counter_first(3)
  end
end
