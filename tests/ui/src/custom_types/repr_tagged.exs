ExUnit.start()

defmodule ReprTaggedTest do
  use ExUnit.Case

  alias Patina.ReprTagged
  alias Patina.ReprTagged.TaggedEnum

  test "tagged structs construct, expose fields, and match" do
    value = {:struct_http_style, 200, "ok"}

    assert ReprTagged.struct_value() == value
    assert ReprTagged.struct_code(value) == 200
    assert ReprTagged.update_code(value) == {:struct_http_style, 201, "ok"}
    assert ReprTagged.update_fields(value) == {:struct_http_style, 202, "updated"}
    assert ReprTagged.destructure_struct(value) == 200
    assert ReprTagged.unit_value() == {:custom_unit}
  end

  test "tagged enum variants construct and match" do
    unit = {TaggedEnum.Unit}
    tuple = {TaggedEnum.TupleValue, 10}
    record = {TaggedEnum.RecordValue, 20, 30}
    atom = TaggedEnum.AtomOverride

    assert ReprTagged.enum_values() == [unit, tuple, record, atom]
    assert ReprTagged.enum_payload(unit) == 0
    assert ReprTagged.enum_payload(tuple) == 10
    assert ReprTagged.enum_payload(record) == 20
    assert ReprTagged.enum_payload(atom) == -1
  end

  test "Result-shaped enums use idiomatic tags" do
    assert ReprTagged.result_values() == [{:ok, 42}, {:error, "reason"}]
    assert ReprTagged.result_payload({:ok, 42}) == 42
    assert ReprTagged.result_payload({:error, "reason"}) == -1
  end
end
