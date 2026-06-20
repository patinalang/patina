ExUnit.start()

defmodule Alias01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "aliased integer types remain interchangeable" do
    assert """
           5 nanoseconds + 2 inches = 7 unit?
           """ == capture_io(fn -> Patina.Alias01.main() end)
  end
end
