ExUnit.start()

defmodule BinaryMatching04Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "literal segments match their bytes" do
    assert """
           text rest <<"ina!">>
           png data <<"rest">>
           packet tail <<>>
           """ == capture_io(fn -> Patina.BinaryMatching04.main() end)
  end
end
