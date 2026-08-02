ExUnit.start()

defmodule DestructureTuple01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main destructures the tuple in the matching arm" do
    assert """
           Tell me about {0,-2,3}
           First is `0`, `y` is -2, and `z` is 3
           """ == capture_io(fn -> Patina.DestructureTuple01.main() end)
  end
end
