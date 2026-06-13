ExUnit.start()

defmodule Hello01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints Hello!" do
    assert """
           Hello!
           """ == capture_io(fn -> Patina.Macros01.main() end)
  end
end
