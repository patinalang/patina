ExUnit.start()

defmodule BinaryMatching05Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "pinned variables match their current value" do
    assert """
           png data <<"!rest">>
           expected <<7:4>>
           """ == capture_io(fn -> Patina.BinaryMatching05.main() end)
  end
end
