ExUnit.start()

defmodule DestructureStructures01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  setup_all do
    Code.ensure_loaded!(:"Elixir.Patina.DestructureStructures01.main.Foo")
    :ok
  end

  test "main destructures structs in match and let patterns" do
    assert """
           First of x is 1, b = 2,  y = 3 
           Outside: x0 = {1,2}, y0 = 3
           Nested: nested_x = {1,2}, nested_y = 3
           """ == capture_io(fn -> Patina.DestructureStructures01.main() end)
  end
end
