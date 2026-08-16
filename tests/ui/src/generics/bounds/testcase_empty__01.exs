ExUnit.start()

defmodule TestcaseEmpty01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main calls functions bounded on empty traits" do
    assert """
           A cardinal is red
           A blue jay is blue
           """ == capture_io(fn -> Patina.TestcaseEmpty01.main() end)
  end
end
