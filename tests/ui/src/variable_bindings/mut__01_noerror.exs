ExUnit.start()

defmodule Mut01NoerrorTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the mutated binding" do
    assert """
           Before mutating the binding: 1
           After mutating the binding: 2
           """ == capture_io(fn -> Patina.Mut01Noerror.main() end)
  end
end
