ExUnit.start()

defmodule Designators01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "identifier and expression designators expand" do
    assert """
           You called <<"foo">>()
           You called <<"bar">>()
           <<"1 + 1">> = 2
           <<"{ let x = 1; x * x + 2 * x - 1 }">> = 2
           """ == capture_io(fn -> Patina.Designators01.main() end)
  end
end
