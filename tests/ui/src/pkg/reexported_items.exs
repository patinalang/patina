ExUnit.start()

defmodule ReexportedItemsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "proc macros reexported through another package still expand" do
    assert """
           4 7
           """ == capture_io(fn -> Patina.ReexportedItems.proc_macros() end)
  end

  test "a type reexported through another package" do
    assert """
           9
           """ == capture_io(fn -> Patina.ReexportedItems.wrapper() end)
  end
end
