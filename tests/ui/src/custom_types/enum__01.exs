ExUnit.start()

defmodule Enum01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Enum01.WebEvent

  test "main destructures every enum variant" do
    assert """
           pressed '120'.
           pasted "my text".
           clicked at x=20, y=80.
           page loaded
           page unloaded
           """ == capture_io(fn -> Patina.Enum01.main() end)
  end

  test "newtype variants are transparent and multi-field variants are records" do
    Code.ensure_loaded!(WebEvent)

    assert_raise ArgumentError, fn -> :records.get_definition(WebEvent, :KeyPress) end
    assert_raise ArgumentError, fn -> :records.get_definition(WebEvent, :Paste) end
    assert {%{is_exported: true}, [:x, :y]} = :records.get_definition(WebEvent, :Click)
  end
end
