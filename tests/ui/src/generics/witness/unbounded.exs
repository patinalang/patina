ExUnit.start()

defmodule UnboundedTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "unbounded inherent generics do not require witnesses" do
    assert ~S"""
           Pair::len(#'Elixir.Patina.Unbounded.Pair':'Pair'{a = 1,b = 2})
           2
           """ == capture_io(fn -> Patina.Unbounded.main() end)
  end
end
