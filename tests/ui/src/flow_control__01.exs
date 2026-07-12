ExUnit.start()

defmodule FlowControl01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the rebound value" do
    assert """
           4
           """ == capture_io(fn -> Patina.FlowControl01.main() end)
  end
end
