ExUnit.start()

defmodule NativeEqTest do
  use ExUnit.Case

  alias Patina.NativeEq

  test "EqByParity's eq differs from comparing its terms" do
    assert NativeEq.eq_by_parity_eq(1, 3)
    refute NativeEq.eq_by_parity_terms_eq(1, 3)
  end

  test "containers of a type with a custom eq use that eq" do
    assert NativeEq.eq_by_parity_lists_eq(1, 3)
    assert NativeEq.eq_by_parity_tuples_eq(1, 3)
    assert NativeEq.eq_by_parity_maps_eq(1, 3)
    assert NativeEq.eq_by_parity_options_eq(1, 3)
  end

  test "containers of NativeEq types compare their terms" do
    assert NativeEq.int_lists_eq([1, 2], [1, 2])
    refute NativeEq.int_lists_eq([1, 2], [1, 3])
    assert NativeEq.tuples_eq({1, "a"}, {1, "a"})
    refute NativeEq.tuples_eq({1, "a"}, {1, "b"})
    assert NativeEq.maps_eq(%{k: [1]}, %{k: [1]})
    refute NativeEq.maps_eq(%{k: [1]}, %{k: [2]})
    assert NativeEq.int_options_eq(1, 1)
    refute NativeEq.int_options_eq(1, 2)
    assert NativeEq.hand_written_native_eq_lists_eq(1, 1)
    refute NativeEq.hand_written_native_eq_lists_eq(1, 2)
  end

  test "a witness comes from the impl selected where it's built" do
    assert NativeEq.int_lists_eq_through_witness([1, 2], [1, 2])
    refute NativeEq.int_lists_eq_through_witness([1, 2], [2, 1])
    assert NativeEq.eq_by_parity_lists_eq_through_witness(1, 3)
  end

  test "generic code uses the general impl" do
    assert NativeEq.eq_by_parity_lists_eq_in_generic_fn(1, 3)
  end
end
