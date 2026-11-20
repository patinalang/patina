ExUnit.start()

defmodule TryCatchAfter02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the int caught from a badmatch error" do
    assert """
           42
           """ == capture_io(fn -> Patina.TryCatchAfter02.main() end)
  end
end
