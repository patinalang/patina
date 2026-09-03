ExUnit.start()

defmodule ReprNativeRecordTest do
  use ExUnit.Case

  alias Patina.ReprNativeRecord
  alias Patina.ReprNativeRecord.ElixirUnit
  alias Patina.ReprNativeRecord.MixedEnum
  alias Patina.ReprNativeRecord.NativeRecordPair
  alias Patina.ReprNativeRecord.NativeRecordStruct
  alias Patina.ReprNativeRecord.WholeEnum

  setup_all do
    Code.ensure_all_loaded!([
      ElixirUnit,
      MixedEnum,
      NativeRecordPair,
      NativeRecordStruct,
      WholeEnum
    ])
  end

  test "native_record is inherited by unit variants" do
    assert {%{is_exported: true}, []} = :records.get_definition(WholeEnum, :Unit)

    assert [] = :records.get_field_names(ReprNativeRecord.inherited_unit())
  end

  test "a variant-level native_record overrides the default atom representation" do
    assert_raise ArgumentError, fn -> :records.get_definition(MixedEnum, :AtomUnit) end
    assert {%{is_exported: true}, []} = :records.get_definition(MixedEnum, :RecordUnit)

    assert [] = :records.get_field_names(ReprNativeRecord.overridden_unit())
  end

  test "Elixir overrides the default atom representation of a unit struct" do
    assert %ElixirUnit{} = ReprNativeRecord.elixir_unit()
  end

  test "native-record struct fields can be read locally and externally" do
    value = ReprNativeRecord.record_struct()

    assert [:x, :y] = :records.get_field_names(value)
    assert 10 = ReprNativeRecord.struct_x(value)
    assert 30 = NativeRecordStruct.sum(value)
  end

  test "native-record tuple struct fields use atomized indexes" do
    value = ReprNativeRecord.record_pair()

    assert [:"0", :"1"] = :records.get_field_names(value)
    assert 40 = ReprNativeRecord.pair_second(value)
  end
end
