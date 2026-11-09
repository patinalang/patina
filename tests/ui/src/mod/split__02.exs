ExUnit.start()

defmodule Split02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "root functions print" do
    assert """
           called `my_mod::function()`
           """ == capture_io(fn -> Patina.Split02.function() end)

    assert """
           called `my_mod::indirect_access()`, that
           > called `my_mod::private_function()`
           """ == capture_io(fn -> Patina.Split02.indirect_access() end)
  end

  test "split submodule functions print" do
    assert """
           called `my_mod::nested::function()`
           """ == capture_io(fn -> Patina.Split02.Nested.function() end)

    assert """
           called `my_mod::inaccessible::public_function()`
           """ == capture_io(fn -> Patina.Split02.Inaccessible.public_function() end)
  end
end
