ExUnit.start()

defmodule DispatchTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  setup_all do
    Code.ensure_loaded!(Patina.Dispatch.Point)
    :ok
  end

  test "generic calls dispatch through a witness" do
    assert ~S"""
           Area::area(#'Elixir.Patina.Dispatch.Point':'Point'{x = 3,y = 4})
           12
           """ == capture_io(fn -> Patina.Dispatch.main() end)
  end
end
