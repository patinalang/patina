ExUnit.start()

defmodule PrintlnTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "println! prints at the crate root" do
    assert """
           top 1 1
           """ == capture_io(fn -> Patina.Println.top(1) end)
  end

  test "println! and format! work in nested modules" do
    assert """
           outer 2 2
           """ == capture_io(fn -> Patina.Println.Outer.outer(2) end)

    assert Patina.Println.Outer.Inner.inner(3) == "inner 3"
  end
end
