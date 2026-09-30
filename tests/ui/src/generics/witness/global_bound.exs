ExUnit.start()

defmodule GlobalBoundTest do
  use ExUnit.Case

  test "an impl with a bound without type parameters calls through it" do
    assert 2 == Patina.GlobalBound.base()
  end

  test "a function with only such bounds needs no witness" do
    assert 1 == Patina.GlobalBound.global()
  end
end
