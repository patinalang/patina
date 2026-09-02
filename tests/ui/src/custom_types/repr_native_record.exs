ExUnit.start()

defmodule ReprNativeRecordTest do
  use ExUnit.Case

  alias Patina.ReprNativeRecord
  alias Patina.ReprNativeRecord.ElixirUnit
  alias Patina.ReprNativeRecord.MixedEnum
  alias Patina.ReprNativeRecord.WholeEnum

  setup_all do
    Code.ensure_all_loaded!([ElixirUnit, MixedEnum, WholeEnum])
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
end
