ExUnit.start()

defmodule Hello01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints Hello!" do
    assert capture_io(fn -> Macros01.main() end) == "Hello!\n"
  end
end
