ExUnit.start()

defmodule Enum03Test do
  use ExUnit.Case

  alias Patina.Enum03.VeryVerboseEnumOfThingsToDoWithNumbers, as: Operation

  setup_all do
    Code.ensure_all_loaded!([Operation])
  end

  test "enum module exports its variant record definitions" do
    Enum.each([:Add, :Subtract], fn variant ->
      assert {%{is_exported: true}, []} = :records.get_definition(Operation, variant)
    end)
  end
end
