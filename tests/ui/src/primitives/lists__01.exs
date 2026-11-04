ExUnit.start()

defmodule Lists01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  defp output_lines do
    capture_io(fn -> Patina.Lists01.main() end) |> String.split("\n", trim: true)
  end

  test "get(0) returns the first element" do
    assert "elem 0: 1" in output_lines()
  end

  test "len counts the elements" do
    assert "Number of elements in array: 5" in output_lines()
  end

  test "head returns the first element" do
    assert "First element of the list: 1" in output_lines()
  end

  test "tail returns the rest of the list" do
    assert "Rest of the list: [2,3,4,5]" in output_lines()
  end

  test "prepending keeps the original list as the tail" do
    assert "more_xs.tail().tail() is equal to xs: true" in output_lines()
  end

  test "a list pattern binds the first elements" do
    assert "Sum of first three elements is 0" in output_lines()
  end

  test "a list pattern binds the rest of the list" do
    assert "...and the rest: [2,3,4,5]" in output_lines()
  end
end
