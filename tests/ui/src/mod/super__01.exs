ExUnit.start()

defmodule Super01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "self, super and pkg paths reach the right functions" do
    assert """
           called `my::indirect_call()`, that
           > called `my::function()`
           called `my::function()`
           called `my::cool::function()`
           called `function()`
           called `cool::function()`
           """ == capture_io(fn -> Patina.Super01.main() end)
  end
end
