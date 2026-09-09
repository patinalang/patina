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

  test "a rest-only pattern matches any map" do
    assert MapsPatterns.is_map(%{})
    assert MapsPatterns.is_map(%{"extra" => 1})
  end
end
