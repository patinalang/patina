ExUnit.start()

defmodule BinaryMatching03Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "sizes can use earlier bindings, and a bits tail takes the unaligned rest" do
    assert """
           kind = 16, payload = <<6,7>>, rest = <<>>
           header = 53, tail = <<7:3>>
           """ == capture_io(fn -> Patina.BinaryMatching03.main() end)
  end
end
