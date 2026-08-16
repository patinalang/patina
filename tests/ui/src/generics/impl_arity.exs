ExUnit.start()

defmodule ImplArityTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main dispatches value/1 and value/2 by receiver type" do
    assert """
           5 hi
           """ == capture_io(fn -> Patina.ImplArity.main() end)
  end
end
