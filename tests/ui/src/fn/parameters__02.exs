ExUnit.start()

defmodule Parameters02Test do
  use ExUnit.Case

  alias Patina.Parameters02.MathOp

  setup_all do
    Code.ensure_all_loaded!([MathOp])
  end

  test "enum module exports its variant record definitions" do
    Enum.each([:Add, :Mul], fn variant ->
      assert {%{is_exported: true}, []} = :records.get_definition(MathOp, variant)
    end)
  end
end
