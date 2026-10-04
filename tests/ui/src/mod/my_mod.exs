ExUnit.start()

defmodule MyModTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "root functions return their values" do
    assert Patina.MyMod.function() == 1
    assert Patina.MyMod.indirect_access() == 2
  end

  test "split submodule functions print" do
    assert """
           called `MyMod::Nested::function()`
           """ == capture_io(fn -> Patina.MyMod.Nested.function() end)

    assert """
           called `MyMod::Inaccessible::public_function()`
           """ == capture_io(fn -> Patina.MyMod.Inaccessible.public_function() end)
  end
end
