ExUnit.start()

defmodule AssocBoundTest do
  use ExUnit.Case

  test "a declared bound of an associated type dispatches to the impl for a primitive" do
    assert "an int" == Patina.AssocBound.counter_first(3)
  end

  test "a declared bound of a generic associated type dispatches to the impl" do
    assert "an int" == Patina.AssocBound.wrapper_plain()
  end

  test "a declared bound of a generic associated type gets the witnesses of its parameters" do
    assert "an int" == Patina.AssocBound.wrapper_member()
  end
end
