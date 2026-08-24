ExUnit.start()

defmodule CompareTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "generic compare dispatches through a witness" do
    assert """
           true
           """ == capture_io(fn -> Patina.Compare.main() end)
  end
end
