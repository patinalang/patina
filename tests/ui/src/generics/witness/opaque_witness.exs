ExUnit.start()

defmodule OpaqueWitnessTest do
  use ExUnit.Case

  test "a generic call takes the witness of an opaque type's hidden type" do
    assert "a" == Patina.OpaqueWitness.show_opaque_return()
  end
end
