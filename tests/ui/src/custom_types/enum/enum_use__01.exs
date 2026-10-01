ExUnit.start()

defmodule EnumUse01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.EnumUse01.Role
  alias Patina.EnumUse01.Stage

  test "main matches imported enum variants" do
    assert """
           Beginners are starting their learning journey!
           Students are acquiring knowledge!
           """ == capture_io(fn -> Patina.EnumUse01.main() end)
  end

  test "unit variants do not emit record definitions" do
    Code.ensure_all_loaded!([Stage, Role])

    Enum.each(
      [{Stage, :Beginner}, {Stage, :Advanced}, {Role, :Student}, {Role, :Teacher}],
      fn {module, variant} ->
        assert_raise ArgumentError, fn -> :records.get_definition(module, variant) end
      end
    )
  end
end
