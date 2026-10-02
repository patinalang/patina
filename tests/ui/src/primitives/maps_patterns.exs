ExUnit.start()

defmodule MapsPatternsTest do
  use ExUnit.Case

  alias Patina.MapsPatterns

  test "a keyed map pattern is partial" do
    assert MapsPatterns.lookup_name(%{"name" => 7, "extra" => 9}) == 7
    assert MapsPatterns.lookup_name(%{"other" => 9}) == -1
  end

  test "a concrete key can match a term-keyed map" do
    assert MapsPatterns.lookup_name_in_term_keyed_map(%{"name" => 7, :extra => 9}) == 7
    assert MapsPatterns.lookup_name_in_term_keyed_map(%{:name => 9}) == -1
  end

  test "map patterns can be nested" do
    map = %{"outer" => %{"inner" => 11, "extra" => 12}, "top_extra" => %{}}
    assert MapsPatterns.nested_lookup(map) == 11
  end

  test "a key can be a constant or a pinned variable" do
    assert MapsPatterns.lookup_constant_key(%{"name" => 7}) == 7
    assert MapsPatterns.lookup_constant_key(%{"other" => 7}) == -1
    assert MapsPatterns.lookup_pinned_key(%{"a" => 1, "b" => 2}, "b") == 2
    assert MapsPatterns.lookup_pinned_key(%{"a" => 1}, "b") == -1
    assert MapsPatterns.lookup_pinned_key_in_closure(%{"a" => 1, "b" => 2}, "a") == 1
  end

  test "a key can be a tuple, list, binary or map" do
    assert MapsPatterns.lookup_tuple_key(%{{7, 1} => 1}, 7) == 1
    assert MapsPatterns.lookup_tuple_key(%{{8, 1} => 3}, 7) == -1
    assert MapsPatterns.lookup_list_key(%{[1, 2] => 4}) == 4
    assert MapsPatterns.lookup_list_key(%{[1, 2, 3] => 4}) == -1
    assert MapsPatterns.lookup_binary_key(%{<<1, 2>> => 5}) == 5
  end

  test "a key can be an enum variant" do
    square = {Patina.MapsPatterns.Shape.Square, 3}
    assert MapsPatterns.lookup_enum_key(%{square => 8}, 3) == 8
    assert MapsPatterns.lookup_enum_key(%{square => 8}, 4) == -1
    assert MapsPatterns.lookup_enum_key(%{{Patina.MapsPatterns.Shape.Point} => 9}, 4) == 9
    assert MapsPatterns.lookup_option_key(%{1 => 10}) == 10
    assert MapsPatterns.lookup_option_key(%{nil => 11}) == 11
  end

  test "a map in a key is matched exactly" do
    assert MapsPatterns.lookup_exact_map_key(%{%{1 => 10} => 7}, 10) == 7
    assert MapsPatterns.lookup_exact_map_key(%{%{1 => 10, 2 => 20} => 7}, 10) == -1
    assert MapsPatterns.lookup_exact_map_key(%{%{1 => 11} => 7}, 10) == -1
  end

  test "a rest-only pattern matches any map" do
    assert MapsPatterns.is_map(%{})
    assert MapsPatterns.is_map(%{"extra" => 1})
  end
end
