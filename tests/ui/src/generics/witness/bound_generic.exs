ExUnit.start()

defmodule BoundGenericTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "bound generic witnesses are forwarded through calls" do
    assert ~S"""
           Tr::method(#{'__struct__' => 'Elixir.Patina.BoundGeneric.Holder',
             value => #{'__struct__' => 'Elixir.Patina.BoundGeneric.Number',n => 5}})
           Bound::b(#{'__struct__' => 'Elixir.Patina.BoundGeneric.Number',n => 5})
           5
           """ == capture_io(fn -> Patina.BoundGeneric.main() end)
  end
end
