ExUnit.start()

defmodule Hello01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints Hello World!" do
    assert """
           Hello World!
           """ == capture_io(fn -> Patina.Hello01.main() end)
  end
end
