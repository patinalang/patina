ExUnit.start()

defmodule IfElse01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the selected branches and expression result" do
    assert """
           5 is positive, and is a small number, increase ten-fold
           5 -> 50
           """ == capture_io(fn -> Patina.IfElse01.main() end)
  end
end
