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

  test "newtypes are transparent" do
    assert Patina.NewTypes01.Years.__info__(:struct) == nil
    assert Patina.NewTypes01.Days.__info__(:struct) == nil
  end

  test "Years and Days conversions round-trip" do
    years = 25

    assert Patina.NewTypes01.Years.to_days(years) == 9125
    assert Patina.NewTypes01.Days.to_years(9125) == years
  end
end
