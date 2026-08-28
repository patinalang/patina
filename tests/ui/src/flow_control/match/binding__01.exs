ExUnit.start()

defmodule Binding01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main binds the value matched by an inclusive range" do
    assert """
           Tell me what type of person you are
           I'm a teen of age 15
           """ == capture_io(fn -> Patina.Binding01.main() end)
  end
end
