ExUnit.start()

defmodule ForwardTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "witnesses are forwarded between generic functions" do
    assert ~S"""
           Tr::method('Elixir.Patina.Forward.S')
           """ == capture_io(fn -> Patina.Forward.main() end)
  end
end
