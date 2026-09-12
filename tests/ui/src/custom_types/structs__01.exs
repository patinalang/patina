ExUnit.start()

defmodule Structs01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Structs01.Person
  alias Patina.Structs01.Point
  alias Patina.Structs01.Pair
  alias Patina.Structs01.Rectangle

  setup_all do
    Code.ensure_all_loaded!([Person, Point, Pair, Rectangle])
    :ok
  end

  test "main debug-prints Person struct" do
    assert capture_io(fn -> Patina.Structs01.main() end) =~ ~S"""
           #'Elixir.Patina.Structs01.Person':'Person'{name = <<"Peter">>,age = 27}
           """
  end

  test "main prints println! output" do
    assert capture_io(fn -> Patina.Structs01.main() end) =~ ~S"""
           point coordinates: (5.2, 0.4)
           second point: (10.3, 0.2)
           pair contains 1 and 0.1
           pair contains 1 and 0.1
           """
  end

  test "multi-field structs export native record definitions" do
    assert {%{is_exported: true}, [:name, :age]} = :records.get_definition(Person, :Person)
    assert {%{is_exported: true}, [:x, :y]} = :records.get_definition(Point, :Point)
    assert {%{is_exported: true}, [:"0", :"1"]} = :records.get_definition(Pair, :Pair)
    assert {%{is_exported: true}, [:top_left, :bottom_right]} =
             :records.get_definition(Rectangle, :Rectangle)
  end
end
