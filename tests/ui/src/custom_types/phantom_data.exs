ExUnit.start()

defmodule PhantomDataTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.PhantomData
  alias Patina.PhantomData.ElixirPoint
  alias Patina.PhantomData.Record

  @marker Patina.Core.PhantomData

  test "a single runtime field makes the struct transparent" do
    assert PhantomData.typed(5) == 5
    assert PhantomData.typed_value(5) == 5
    assert PhantomData.typed_destructure(5) == 5
    assert PhantomData.typed_constant_pattern(5) == 5
    assert PhantomData.typed_or_pattern(2) == 0
    assert PhantomData.typed_or_pattern(5) == 5
    assert PhantomData.typed_update(5) == 7
  end

  test "reading a phantom field or variable gives the PhantomData value" do
    assert PhantomData.typed_marker(5) == @marker
    assert PhantomData.typed_binding(5) == @marker
    assert PhantomData.marker_inner(PhantomData.marker()) == @marker
  end

  test "values of phantom fields are still evaluated" do
    assert """
           made a marker
           """ == capture_io(fn -> assert PhantomData.typed_effect(3) == 3 end)

    assert """
           made a marker
           """ == capture_io(fn -> assert PhantomData.marker_effect() == @marker end)

    record = PhantomData.record(1, "b")

    assert """
           made a marker
           """ == capture_io(fn -> assert PhantomData.record_update_marker(record) == record end)
  end

  test "phantom variables work inside loops and closures" do
    assert PhantomData.phantom_in_loop_and_closure(4) == 3
  end

  test "a lazy map iterator is the bare maps:iterator term" do
    map = %{"a" => 1, "b" => 2, "c" => 3}
    iter = PhantomData.map_iter(map)

    assert {_key, _value, _rest} = :maps.next(iter)
    assert PhantomData.sum_values(map) == 6
    assert {key, value} = PhantomData.first_entry(iter)
    assert map[key] == value
    assert PhantomData.first_entry(PhantomData.map_iter(%{})) == nil
  end

  test "native records leave out phantom fields" do
    record = PhantomData.record(1, "b")

    assert [:a, :b] = :records.get_field_names(record)
    assert PhantomData.record_b(record) == "b"
    assert :records.get(:b, PhantomData.record_update(record)) == "updated"
    assert PhantomData.record_destructure(record) == 1
    assert Record.__info__(:functions) |> Keyword.get(:"$patina_constructor__Record") == 2
  end

  test "tagged tuples leave out phantom fields" do
    assert PhantomData.pair(1, 2) == {Patina.PhantomData.Shape.Pair, 1, 2}
    assert PhantomData.pair_second({Patina.PhantomData.Shape.Pair, 1, 2}) == 2

    tagged = PhantomData.tagged_pair(1, 2)
    assert tagged == {Patina.PhantomData.TaggedPair, 1, 2}
    assert PhantomData.tagged_pair_b(tagged) == 2
    assert PhantomData.tagged_pair_update(tagged) == {Patina.PhantomData.TaggedPair, 1, 99}
  end

  test "Elixir structs leave out phantom fields" do
    point = PhantomData.elixir_point(1, 2)

    assert %ElixirPoint{x: 1, y: 2} = point
    assert point |> Map.from_struct() |> Map.keys() |> Enum.sort() == [:x, :y]
    assert [%{field: :x}, %{field: :y}] = ElixirPoint.__info__(:struct)
    assert PhantomData.elixir_point_sum(point) == 3
  end

  test "a struct with only phantom fields is an atom" do
    assert PhantomData.marker() == Patina.PhantomData.Marker
  end

  test "PhantomData values in generic positions are materialized" do
    assert PhantomData.materialized_marker() == @marker
    assert PhantomData.is_some_marker(@marker)
  end

  test "terms match structs with phantom fields" do
    assert PhantomData.term_matches_phantom_struct(PhantomData.record(4, "b")) == 4
    assert PhantomData.term_matches_phantom_struct(:other) == -1
  end
end
