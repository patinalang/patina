ExUnit.start()

defmodule BinaryMatching01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "segments split a binary into an int prefix, a tag and the rest" do
    assert """
           prefix = 273, tag = 42, rest = <<"c">>
           """ == capture_io(fn -> Patina.BinaryMatching01.main() end)
  end
end
