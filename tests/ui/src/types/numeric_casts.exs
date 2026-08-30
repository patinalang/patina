ExUnit.start()

defmodule NumericCastsTest do
  use ExUnit.Case

  alias Patina.NumericCasts

  test "casts between numeric types" do
    assert NumericCasts.float_to_int(3.14) === 3
    assert NumericCasts.int_to_float(42) === 42.0
  end

  test "casts numeric types to term" do
    assert NumericCasts.float_to_int_to_term(3.14) === 3
    assert NumericCasts.int_to_float_to_term(42) === 42.0
  end
end
