ExUnit.start()

defmodule Split03Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "nested split file function runs" do
    assert """
           called `MyMod::Nested::function()`
           """ == capture_io(fn -> Patina.Split03.function() end)
  end
end
