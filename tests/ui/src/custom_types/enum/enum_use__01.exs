ExUnit.start()

defmodule EnumUse01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.EnumUse01.Role
  alias Patina.EnumUse01.Stage

  setup_all do
    Code.ensure_all_loaded!([Stage, Role])
  end

  test "main matches imported enum variants" do
    assert """
           Beginners are starting their learning journey!
           Students are acquiring knowledge!
           """ == capture_io(fn -> Patina.EnumUse01.main() end)
  end

  test "enum modules export their variant record definitions" do
    Enum.each(
      [{Stage, :Beginner}, {Stage, :Advanced}, {Role, :Student}, {Role, :Teacher}],
      fn {module, variant} ->
        assert {%{is_exported: true}, []} = :records.get_definition(module, variant)
      end
    )
  end
end
