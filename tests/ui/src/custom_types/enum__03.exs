ExUnit.start()

defmodule Enum03Test do
  use ExUnit.Case

  alias Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers, as: Operation

  test "unit variants do not emit record definitions" do
    Code.ensure_loaded!(Operation)

    Enum.each([:Add, :Subtract], fn variant ->
      assert_raise ArgumentError, fn -> :records.get_definition(Operation, variant) end
    end)
  end
end
