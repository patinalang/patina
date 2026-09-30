ExUnit.start()

defmodule NestedAssocBoundTest do
  use ExUnit.Case

  test "a nested associated-type bound dispatches through the enclosing witness" do
    assert "tag" == Patina.NestedAssocBound.root_label()
  end
end
