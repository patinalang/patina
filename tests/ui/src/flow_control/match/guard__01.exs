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

  test "payload variants are exported native records" do
    assert {%{is_exported: true}, [:"0"]} = :records.get_definition(Temperature, :Celsius)
    assert {%{is_exported: true}, [:"0"]} = :records.get_definition(Temperature, :Fahrenheit)

    celsius = :records.create(Temperature, :Celsius, [{:"0", 35}], %{is_exported: true})

    assert Temperature = :records.get_module(celsius)
    assert :Celsius = :records.get_name(celsius)
    assert [:"0"] = :records.get_field_names(celsius)
    assert 35 = :records.get(:"0", celsius)
    assert :records.is_exported(celsius)
  end
end
