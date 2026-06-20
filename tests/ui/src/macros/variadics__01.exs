ExUnit.start()

defmodule Variadics01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "variadic macro evaluates every expression" do
    assert """
           1 + 2 = 3
           3 + 4 = 7
           (2 * 3) + 1 = 7
           """ == capture_io(fn -> Patina.Variadics01.main() end)
  end
end
