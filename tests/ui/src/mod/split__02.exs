ExUnit.start()

defmodule Split02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "root functions print" do
    assert """
           called `MyMod::function()`
           """ == capture_io(fn -> Patina.Split02.function() end)

    assert """
           called `MyMod::indirect_access()`, that
           > called `MyMod::private_function()`
           """ == capture_io(fn -> Patina.Split02.indirect_access() end)
  end

  test "split submodule functions print" do
    assert """
           called `MyMod::Nested::function()`
           """ == capture_io(fn -> Patina.Split02.Nested.function() end)

    assert """
           called `MyMod::Inaccessible::public_function()`
           """ == capture_io(fn -> Patina.Split02.Inaccessible.public_function() end)
  end
end
