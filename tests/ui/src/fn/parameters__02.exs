ExUnit.start()

defmodule Parameters02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Parameters02.MathOp

  test "unit variants do not emit record definitions" do
    Code.ensure_loaded!(MathOp)

    Enum.each([:Add, :Mul], fn variant ->
      assert_raise ArgumentError, fn -> :records.get_definition(MathOp, variant) end
    end)
  end

  test "main calls local function references" do
    assert """
           3 op 5 = 8
           """ == capture_io(fn -> Patina.Parameters02.main() end)
  end
end
