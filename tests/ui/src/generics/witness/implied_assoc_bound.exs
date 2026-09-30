ExUnit.start()

defmodule ImpliedAssocBoundTest do
  use ExUnit.Case

  test "an associated-type bound in a supertrait list has a witness" do
    assert {"tag", "tag"} == Patina.ImpliedAssocBound.crate_labels()
  end
end
