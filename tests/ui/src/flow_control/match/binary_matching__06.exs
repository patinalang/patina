ExUnit.start()

defmodule BinaryMatching06Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "constructed binaries encode each segment" do
    assert """
           <<"Hello, Patina!">> <<80,84,0,3,63,192,0,0,0,233>> <<20:5>>
           """ == capture_io(fn -> Patina.BinaryMatching06.main() end)
  end
end
