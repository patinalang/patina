ExUnit.start()

defmodule ConstValuesTest do
  use ExUnit.Case

  alias Patina.ConstValues

  test "named constants are inlined into the returned tuple and map" do
    assert ConstValues.const_values() == {[1, 2, 3], %{"answer" => 42}}
  end
end
