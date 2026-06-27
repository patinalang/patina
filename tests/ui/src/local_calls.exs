ExUnit.start()

defmodule LocalCallsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main executes local and associated calls" do
    output = capture_io(fn -> Patina.LocalCalls.main() end)

    assert output =~ "Age "
    assert output =~ " is 25\n"
  end

  test "associated constructor returns the generated struct" do
    assert %Patina.LocalCalls.Years{"0": 25} = Patina.LocalCalls.Years.new(25)
  end

  test "get_age/1 is private" do
    refute function_exported?(Patina.LocalCalls, :get_age, 1)
  end
end
