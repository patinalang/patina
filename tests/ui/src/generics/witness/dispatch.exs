ExUnit.start()

defmodule DispatchTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "generic calls dispatch through a witness" do
    assert ~S"""
           Area::area(#{'__struct__' => 'Elixir.Patina.Dispatch.Point',x => 3,y => 4})
           12
           """ == capture_io(fn -> Patina.Dispatch.main() end)
  end
end
