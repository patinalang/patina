ExUnit.start()

defmodule Structs01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main debug-prints Person struct" do
    assert capture_io(fn -> Patina.Structs01.main() end) =~ ~S"""
           #{'__struct__' => 'Elixir.Patina.Structs01.Person',age => 27,
             name => <<"Peter">>}
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

  test "compiled structs are valid Elixir structs" do
    %Patina.Structs01.Person{name: "Peter", age: 27}
    %Patina.Structs01.Point{x: 5.2, y: 0.4}
    %Patina.Structs01.Pair{"0": 1, "1": 0.1}

    %Patina.Structs01.Rectangle{
      top_left: %Patina.Structs01.Point{x: 5.2, y: 0.4},
      bottom_right: %Patina.Structs01.Point{x: 10.3, y: 0.2}
    }
  end
end
