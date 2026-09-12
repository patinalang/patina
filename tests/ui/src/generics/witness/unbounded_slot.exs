ExUnit.start()

defmodule UnboundedSlotTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "unbounded generic witness slots accept concrete types" do
    assert ~S"""
           Tr::method(1)
           """ == capture_io(fn -> Patina.UnboundedSlot.main() end)
  end
end
