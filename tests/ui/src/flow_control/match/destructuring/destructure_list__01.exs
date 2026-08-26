ExUnit.start()

defmodule DestructureList01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main destructures the list in the matching arm" do
    assert """
           list[0] = 1, list[2] = 6 and list[1] was ignored
           """ == capture_io(fn -> Patina.DestructureList01.main() end)
  end
end
