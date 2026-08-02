ExUnit.start()

defmodule MapsNativeTest do
  use ExUnit.Case

  alias Patina.MapsNative

  test "create returns a map" do
    assert MapsNative.create() == %{"a" => 1, "b" => 2}
  end

  test "empty returns an empty map" do
    assert MapsNative.empty() == %{}
  end

  test "a map can be passed through a function" do
    map = %{"a" => 1, "b" => 2}
    assert MapsNative.param_and_return(map) == map
  end

  test "an empty map's type can be inferred from its usage" do
    assert MapsNative.infer_empty_by_usage() == %{}
  end

  test "a map's type can be inferred from its base" do
    base = %{"a" => 1, "b" => 2}
    assert MapsNative.infer_from_base(base) == base
  end

  test "update inserts and replaces" do
    base = %{"a" => 1, "b" => 2}
    assert MapsNative.update(base) == %{"a" => 10, "b" => 2, "c" => 3}
  end

  test "a base-only update returns the same map" do
    base = %{"a" => 1, "b" => 2}
    assert MapsNative.update_identity(base) == base
  end

  test "duplicate keys: last occurrence wins" do
    assert MapsNative.duplicate() == %{"a" => 2}
  end
end
