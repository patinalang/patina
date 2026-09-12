ExUnit.start()

defmodule Guard01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Guard01.Temperature

  setup_all do
    Code.ensure_all_loaded!([Temperature])
  end

  test "main selects the guarded enum arm" do
    assert """
           35C is above 30 Celsius
           """ == capture_io(fn -> Patina.Guard01.main() end)
  end

  test "conflicting payload variants preserve the first transparent shape" do
    assert_raise ArgumentError, fn -> :records.get_definition(Temperature, :Celsius) end
    assert {%{is_exported: true}, [:"0"]} = :records.get_definition(Temperature, :Fahrenheit)

    fahrenheit = :records.create(Temperature, :Fahrenheit, [{:"0", 95}], %{is_exported: true})

    assert Temperature = :records.get_module(fahrenheit)
    assert :Fahrenheit = :records.get_name(fahrenheit)
    assert [:"0"] = :records.get_field_names(fahrenheit)
    assert 95 = :records.get(:"0", fahrenheit)
    assert :records.is_exported(fahrenheit)
  end
end
