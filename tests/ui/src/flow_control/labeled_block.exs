ExUnit.start()

defmodule LabeledBlockTest do
  use ExUnit.Case

  alias Patina.LabeledBlock

  test "break leaves a labeled block with its value" do
    assert LabeledBlock.labeled(10) == 21
    assert LabeledBlock.labeled(1) == 23
  end

  test "break leaves a labeled block from a loop inside it" do
    assert LabeledBlock.nested(3) == 4
  end
end
