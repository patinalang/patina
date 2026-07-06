ExUnit.start()

defmodule Declare01NoerrorTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the bound values" do
    assert """
           a binding: 4
           another binding: 1
           """ == capture_io(fn -> Patina.Declare01Noerror.main() end)
  end
end
