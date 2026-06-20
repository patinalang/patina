ExUnit.start()

defmodule Dsl01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "DSL macro evaluates its expressions" do
    assert """
           1 + 2 = 3
           (1 + 2) * (3 / 4) = 0
           """ == capture_io(fn -> Patina.Dsl01.main() end)
  end
end
