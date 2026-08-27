ExUnit.start()

defmodule Match01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the selected match arms and expression result" do
    assert """
           Tell me about 13
           A teen
           true -> 1
           found unexpected value 123
           """ == capture_io(fn -> Patina.Match01.main() end)
  end
end
