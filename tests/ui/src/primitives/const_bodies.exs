ExUnit.start()

defmodule ConstBodiesTest do
  use ExUnit.Case

  alias Patina.ConstBodies

  test "a constant's value can be a block or arithmetic" do
    assert ConstBodies.block_value() == 1
    assert ConstBodies.nested_block() == %{"value" => 1}
    assert ConstBodies.arithmetic() == <<2>>
    assert ConstBodies.two_uses_share_var() == {1, 1}
    assert ConstBodies.closure_binds_own_consts() == 2
  end

  test "a literal constant matches its value" do
    assert ConstBodies.is_list([1, 2])
    refute ConstBodies.is_list([1, 2, 3])
    assert ConstBodies.is_pair({1, "name"})
    refute ConstBodies.is_pair({1, "other"})
    assert ConstBodies.is_header(<<"PT", 2::16>>)
    refute ConstBodies.is_header(<<"PT", 3::16>>)
  end

  test "a map constant matches only an equal map" do
    assert ConstBodies.is_exactly_map(%{"name" => 1})
    refute ConstBodies.is_exactly_map(%{"name" => 1, "extra" => 2})
  end

  test "a constant which isn't a literal matches its value" do
    assert ConstBodies.is_block_value(1)
    refute ConstBodies.is_block_value(2)
    assert ConstBodies.computed_key(%{"name" => 7}) == 7
    assert ConstBodies.computed_key(%{"other" => 7}) == 0
  end
end
