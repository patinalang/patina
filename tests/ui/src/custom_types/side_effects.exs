ExUnit.start()

defmodule SideEffectsTest do
  use ExUnit.Case

  alias Patina.SideEffects

  # Preserve duplicates while ignoring evaluation order, which is undefined.
  defp effects do
    receive do
      {:side_effect, event} -> [event | effects()]
    after
      0 -> []
    end
  end

  defp assert_effects(expected, call) do
    value = call.()
    assert Enum.sort(effects()) == Enum.sort(expected)
    value
  end

  defp assert_fields(:tagged, value, {x, y, z}) do
    assert value == {:tagged, x, y, z}
  end

  defp assert_fields(:elixir, value, {x, y, z}) do
    assert value == %SideEffects.ElixirRecord{x: x, y: y, z: z}
  end

  defp assert_fields(representation, value, {x, y, z}) do
    {module, name} =
      case representation do
        :patina -> {SideEffects.PatinaRecord, :PatinaRecord}
        :native -> {SideEffects.NativeRecord, :NativeRecord}
      end

    assert :records.get_module(value) == module
    assert :records.get_name(value) == name
    assert :records.get_field_names(value) == [:x, :y, :z]
    assert :records.get(:x, value) == x
    assert :records.get(:y, value) == y
    assert :records.get(:z, value) == z
  end

  for {representation, record} <- [
        {:patina, SideEffects.PatinaRecord},
        {:native, SideEffects.NativeRecord},
        {:tagged, SideEffects.TaggedRecord},
        {:elixir, SideEffects.ElixirRecord}
      ] do
    @representation representation
    @record record

    test "#{representation}: each field call runs exactly once" do
      value = assert_effects([:x, :y, :z], fn -> @record.fields() end)
      assert_fields(@representation, value, {10, 20, 30})
    end

    test "#{representation}: a base call runs once when inheriting all fields" do
      value = assert_effects([:base], fn -> @record.base() end)
      assert_fields(@representation, value, {1, 2, 3})
    end

    test "#{representation}: a base call runs once when inheriting multiple fields" do
      value = assert_effects([:base], fn -> @record.update() end)
      assert_fields(@representation, value, {10, 2, 3})
    end

    test "#{representation}: mixed base and field calls each run once" do
      value = assert_effects([:z, :x, :base], fn -> @record.mixed() end)
      assert_fields(@representation, value, {10, 2, 30})
    end

    test "#{representation}: replacing every field still evaluates the base call" do
      value = assert_effects([:z, :x, :y, :base], fn -> @record.replace() end)
      assert_fields(@representation, value, {10, 20, 30})
    end
  end

  test "transparent: a field call runs exactly once" do
    assert assert_effects([:field], &SideEffects.Transparent.fields/0) == 10
  end

  test "transparent: a base call runs exactly once" do
    assert assert_effects([:base], &SideEffects.Transparent.base/0) == 1
  end

  test "transparent: replacing the payload still evaluates the base call" do
    assert assert_effects([:base], &SideEffects.Transparent.update/0) == 10
  end

  test "transparent: mixed base and field calls each run once" do
    assert assert_effects([:field, :base], &SideEffects.Transparent.mixed/0) == 10
  end

  test "atom: an empty update still evaluates the base call" do
    assert assert_effects([:base], &SideEffects.AtomUnit.base/0) == SideEffects.AtomUnit
  end

  for {payload, expected} <- [
        {42, 42},
        {"text", {:dynamic, "text"}},
        {{:dynamic, 1}, {:dynamic, {:dynamic, 1}}},
        {{:payload, 1}, {:payload, 1}},
        {%{x: 1}, %{x: 1}},
        {[1, 2], [1, 2]}
      ] do
    @payload payload
    @expected expected

    test "dynamic_wrap: the field call runs once for #{inspect(payload)}" do
      assert assert_effects([:field], fn -> SideEffects.Dynamic.fields(@payload) end) ==
               @expected
    end
  end

  test "tuples: each field call runs exactly once" do
    assert assert_effects([:x, :y, :z], &SideEffects.Tuples.fields/0) == {10, 20, 30}
  end

  test "tuples: the base call of field access runs exactly once" do
    assert assert_effects([:base], &SideEffects.Tuples.base/0) == 2
  end

  test "tuples: mixed field and field-access base calls each run once" do
    assert assert_effects([:x, :base, :z], &SideEffects.Tuples.mixed/0) == {10, 2, 30}
  end

  test "maps: each key and value call runs exactly once" do
    value = assert_effects([:x_key, :x, :y_key, :y, :z_key, :z], &SideEffects.Maps.fields/0)
    assert value == %{x: 10, y: 20, z: 30}
  end

  test "maps: a base call runs once when inheriting all entries" do
    assert assert_effects([:base], &SideEffects.Maps.base/0) == %{x: 1, y: 2, z: 3}
  end

  test "maps: a base call runs once when inheriting multiple entries" do
    assert assert_effects([:base], &SideEffects.Maps.update/0) == %{x: 10, y: 2, z: 3}
  end

  test "maps: mixed base, key, and value calls each run once" do
    value = assert_effects([:z_key, :z, :x_key, :x, :base], &SideEffects.Maps.mixed/0)
    assert value == %{x: 10, y: 2, z: 30}
  end

  test "maps: replacing every entry still evaluates the base call" do
    value =
      assert_effects([:z_key, :z, :x_key, :x, :y_key, :y, :base], &SideEffects.Maps.replace/0)

    assert value == %{x: 10, y: 20, z: 30}
  end

  test "maps: overwritten entries still evaluate their key and value calls" do
    assert assert_effects([:key, :value, :key, :value], &SideEffects.Maps.duplicate/0) == %{x: 20}
  end

  test "lists: each element call runs exactly once" do
    assert assert_effects([:x, :y, :z], &SideEffects.Lists.fields/0) == [10, 20, 30]
  end

  test "lists: a tail call runs exactly once" do
    assert assert_effects([:base], &SideEffects.Lists.base/0) == [0, 1, 2, 3]
  end

  test "lists: mixed element and tail calls each run once" do
    assert assert_effects([:x, :y, :base], &SideEffects.Lists.mixed/0) == [10, 20, 3, 4]
  end

  test "lists: an empty tail still evaluates its call" do
    assert assert_effects([:field, :base], &SideEffects.Lists.empty_tail/0) == [10]
  end
end
