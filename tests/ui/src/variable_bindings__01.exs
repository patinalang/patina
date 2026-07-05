ExUnit.start()

defmodule VariableBindings01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the bound values" do
    assert """
           An integer: 1
           A boolean: true
           Meet the unit value: {}
           """ == capture_io(fn -> Patina.VariableBindings01.main() end)
  end
end
