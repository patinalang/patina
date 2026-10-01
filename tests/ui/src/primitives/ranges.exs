ExUnit.start()

defmodule RangesTest do
  use ExUnit.Case

  alias Patina.Ranges

  test "range bounds are stored as given" do
    assert Ranges.half_open_bounds() == {1, 101}
    assert Ranges.closed_bounds() == {1, 100}
  end

  test "ranges are Core native records" do
    assert :records.get_module(Ranges.half_open(1, 5)) == Patina.Core.Range
    assert :records.get_module(Ranges.closed(1, 5)) == Patina.Core.RangeInclusive
    assert :records.get_module(Ranges.char_range()) == Patina.Core.RangeInclusive
  end

  test "a range can be destructured" do
    assert Ranges.destructure(Ranges.half_open(3, 10)) == 7
  end
end
