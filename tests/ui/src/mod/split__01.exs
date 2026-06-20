ExUnit.start()

defmodule Split01Test do
  use ExUnit.Case

  test "split modules from separate source files are callable" do
    assert Patina.Split01.MyMod.function() == 1
    assert Patina.Split01.MyMod.Nested.function() == 1
    assert Patina.Split01.MyMod.Inaccessible.public_function() == 3
  end
end
