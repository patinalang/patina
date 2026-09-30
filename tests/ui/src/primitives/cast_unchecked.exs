ExUnit.start()

defmodule CastUncheckedTest do
  use ExUnit.Case

  alias Patina.CastUnchecked

  setup_all do
    Code.ensure_all_loaded!([Patina.CastUnchecked.Point])
    :ok
  end

  test "the cast result is the value itself" do
    assert Enum.sort(CastUnchecked.inferred_target(%{a: 1, b: 2})) == [:a, :b]
    assert CastUnchecked.explicit_target([1, 2, 3]) == 6
  end

  test "a term can be cast into a Patina struct" do
    assert CastUnchecked.round_trip() == 3
  end
end
