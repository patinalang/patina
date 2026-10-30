ExUnit.start()

defmodule ForeignStructsTest do
  use ExUnit.Case
  alias Patina.ForeignStructs

  test "phantom fields of foreign structs are left out" do
    assert ForeignStructs.phantom_tuple(5) == 5
    assert ForeignStructs.phantom_tuple_value(5) == 5

    value = ForeignStructs.phantom_record(1, "one")
    assert [:id, :name] = :records.get_field_names(value)
    assert ForeignStructs.phantom_record_name(value) == "one"
    assert ForeignStructs.phantom_record_marker(value) == Patina.Core.PhantomData
    assert :records.get(:name, ForeignStructs.rename_phantom_record(value)) == "renamed"
  end

  test "external generic wrappers forward shape dependencies without query cycles" do
    value = ForeignStructs.recursive()
    assert ForeignStructs.Recursive == :records.get_module(value)
    assert 17 == :records.get(:"0", value)
    assert 17 == ForeignStructs.unwrap_recursive(value)
  end
end
