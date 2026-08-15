ExUnit.start()

defmodule MapsNamedConstTest do
  use ExUnit.Case

  alias Patina.MapsNamedConst

  test "a named constant can be used as a map pattern key" do
    assert MapsNamedConst.const_key(%{"name" => 7, "extra" => 9}) == 7
    assert MapsNamedConst.const_key(%{"other" => 9}) == 0
  end

  test "a named constant map is used as a fallback value" do
    assert MapsNamedConst.named_const(%{"name" => 7, "extra" => 9}) == %{"hi" => 7}
    assert MapsNamedConst.named_const(%{"other" => 9}) == %{"name" => 1}
  end
end
