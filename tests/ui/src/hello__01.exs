ExUnit.start()

defmodule Hello01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints Hello World!" do
    assert capture_io(fn -> Hello01.main() end) == "Hello World!\n"
  end
end
