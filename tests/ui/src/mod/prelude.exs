ExUnit.start()

defmodule PreludeTest do
  use ExUnit.Case

  test "prelude items resolve at the crate root" do
    assert Patina.Prelude.top(3) == 3
    assert Patina.Prelude.top(0) == nil
  end

  test "prelude items resolve in nested modules" do
    assert Patina.Prelude.Outer.outer(3) == 3
    assert Patina.Prelude.Outer.outer(0) == nil
    assert Patina.Prelude.Outer.Inner.inner(5) == 5
    assert Patina.Prelude.Outer.Inner.inner(nil) == 0
  end
end
