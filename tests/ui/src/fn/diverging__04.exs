ExUnit.start()

defmodule Diverging04Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main sums the odd numbers, skipping even ones with continue" do
    assert """
           Sum of odd numbers up to 9 (excluding): 16
           """ == capture_io(fn -> Patina.Diverging04.main() end)
  end
end
