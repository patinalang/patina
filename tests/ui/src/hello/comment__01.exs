ExUnit.start()

defmodule Comment01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "comments do not affect the surrounding expression" do
    assert """
           Is `x` 10 or 100? x = 10
           """ == capture_io(fn -> Patina.Comment01.main() end)
  end
end
