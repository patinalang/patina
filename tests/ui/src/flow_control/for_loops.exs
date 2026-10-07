ExUnit.start()

defmodule ForLoopsTest do
  use ExUnit.Case

  alias Patina.ForLoops

  test "for iterates lists, and iterating doesn't consume them" do
    assert ForLoops.sum_list([1, 2, 3]) == 6
    assert ForLoops.sum_list([]) == 0
    assert ForLoops.sum_twice([1, 2]) == 303
  end

  test "for iterates ranges" do
    assert ForLoops.sum_range(1, 5) == 10
    assert ForLoops.sum_range(5, 5) == 0
    assert ForLoops.sum_range_inclusive(1, 5) == 15
    assert ForLoops.sum_range_inclusive(5, 4) == 0
  end

  test "for iterates map entries" do
    assert ForLoops.weighted_map_sum(%{2 => 3, 10 => 1}) == 16
    assert ForLoops.map_entry_count(%{"a" => 1, "b" => 2}) == 2
    assert ForLoops.map_entry_count(%{}) == 0
  end

  test "for iterates through IntoIterator bounds" do
    assert ForLoops.count_list(["a", "b"]) == 2
    assert ForLoops.count_map(%{"a" => 1}) == 1
    assert ForLoops.total_range(4) == 6
    assert ForLoops.total_list([5, 6]) == 11
  end

  test "break, continue and return leave for loops" do
    assert ForLoops.skip_odd_until([1, 2, 3, 4, 10, 6], 5) == 6
    assert ForLoops.find_index([4, 5, 6], 6) == 2
    assert ForLoops.find_index([4], 1) == -1
  end

  test "labeled break and continue reach outer for loops" do
    assert ForLoops.first_pair_with_sum([1, 2, 3, 4], 7) == {3, 4}
    assert ForLoops.first_pair_with_sum([1], 7) == {0, 0}
    assert ForLoops.count_skipping_rows([[1, 2], [3, -1, 4], [5]]) == 4
  end

  test "for iterates user iterators and IntoIterator impls" do
    assert ForLoops.countdown(3) == [1, 2, 3]
    assert ForLoops.countdown(0) == []
    assert ForLoops.bag_count(["x", "y", "z"]) == 3
  end
end
