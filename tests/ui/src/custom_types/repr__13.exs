ExUnit.start()

defmodule Repr13Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "Option wraps only values that collide with None or its own fallback" do
    assert """
           42
           nil
           hi
           {some,nil}
           42
           {some,nil}
           nil
           """ == capture_io(fn -> Patina.Repr13.main() end)
  end
end
