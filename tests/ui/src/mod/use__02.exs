ExUnit.start()

defmodule Use02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "use bindings shadow only within their block" do
    assert """
           called `deeply::nested::function()`
           Entering block
           called `deeply::nested::function()`
           Leaving block
           called `function()`
           """ == capture_io(fn -> Patina.Use02.main() end)
  end
end
