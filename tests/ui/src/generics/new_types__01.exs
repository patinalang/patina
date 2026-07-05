ExUnit.start()

defmodule NewTypes01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints println! output" do
    assert """
           Is an adult? true
           Is an adult? true
           """ == capture_io(fn -> Patina.NewTypes01.main() end)
  end

  test "compiled newtypes are valid Elixir structs" do
    %Patina.NewTypes01.Years{"0": 25}
    %Patina.NewTypes01.Days{"0": 9125}
  end

  test "Years and Days conversions round-trip" do
    years = %Patina.NewTypes01.Years{"0": 25}

    assert Patina.NewTypes01.Years.to_days(years) == %Patina.NewTypes01.Days{"0": 9125}
    assert Patina.NewTypes01.Days.to_years(%Patina.NewTypes01.Days{"0": 9125}) == years
  end
end
