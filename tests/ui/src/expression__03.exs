ExUnit.start()

defmodule Expression03Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "block expression values are printed" do
    assert """
           x is 5
           y is 155
           z is {}
           """ == capture_io(fn -> Patina.Expression03.main() end)
  end
end
