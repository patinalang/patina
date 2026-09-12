ExUnit.start()

defmodule BoundGenericTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "bound generic witnesses are forwarded through calls" do
    assert ~S"""
           Tr::method(5)
           Bound::b(5)
           5
           """ == capture_io(fn -> Patina.BoundGeneric.main() end)
  end
end
