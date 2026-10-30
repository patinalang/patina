ExUnit.start()

defmodule GenericConstsTest do
  use ExUnit.Case

  alias Patina.GenericConsts

  test "generic consts are instantiated at their uses" do
    assert 28 == GenericConsts.quadrupled()
    assert [] == GenericConsts.none()
    assert 14 == GenericConsts.doubled_int()
  end

  test "generic consts can be patterns" do
    assert GenericConsts.is_doubled_int(14)
    refute GenericConsts.is_doubled_int(7)
  end
end
