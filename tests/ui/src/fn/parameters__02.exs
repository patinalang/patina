ExUnit.start()

defmodule Parameters02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Parameters02.MathOp

  setup_all do
    Code.ensure_all_loaded!([MathOp])
  end

  test "enum module exports its variant record definitions" do
    Enum.each([:Add, :Mul], fn variant ->
      assert {%{is_exported: true}, []} = :records.get_definition(MathOp, variant)
    end)
  end

  test "main calls local function references" do
    assert """
           3 op 5 = 8
           """ == capture_io(fn -> Patina.Parameters02.main() end)
  end
end
