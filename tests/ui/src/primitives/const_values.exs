ExUnit.start()

defmodule ConstValuesTest do
  use ExUnit.Case

  alias Patina.ConstValues

  test "named constants are inlined into the returned tuple and map" do
    assert ConstValues.const_values() == {[1, 2, 3], %{"answer" => 42}}
  end

  test "named constants can be binary patterns" do
    assert ConstValues.binary_pattern_values() ==
             {<<"PT", 2::16, ?ł::utf16, 1.5::float-32>>, [<<1::1, 0::2>>, <<1::3>>]}
  end
end
