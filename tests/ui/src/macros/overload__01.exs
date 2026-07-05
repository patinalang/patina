ExUnit.start()

defmodule Overload01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the overloaded test! results" do
    assert """
           <<"1 + 1 == 2">> and <<"2 * 2 == 4">> is true
           <<"true">> or <<"false">> is true
           """ == capture_io(fn -> Patina.Overload01.main() end)
  end
end
