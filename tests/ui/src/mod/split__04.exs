ExUnit.start()

defmodule Split04Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "inaccessible split file function runs" do
    assert """
           called `MyMod::Inaccessible::public_function()`
           """ == capture_io(fn -> Patina.Split04.public_function() end)
  end
end
