ExUnit.start()

defmodule BlanketImplTest do
  use ExUnit.Case

  alias Patina.BlanketImpl

  test "a blanket impl is called statically and through a witness" do
    assert "<an int>" == BlanketImpl.describe_int(1)
    assert "<an int>" == BlanketImpl.describe_generic_int(1)
  end

  test "a blanket impl inherits a default body" do
    assert "<hi>!" == BlanketImpl.shout_str("hi")
  end

  test "an impl on a tuple" do
    assert "an int and hi" == BlanketImpl.describe_pair(1, "hi")
    assert "an int and hian int and hi!" == BlanketImpl.shout_pair_generic(1, "hi")
  end

  test "a blanket impl over another blanket impl" do
    assert "<hi><hi>" == BlanketImpl.twice_str("hi")
    assert "an int and hian int and hi" == BlanketImpl.twice_generic_pair(1, "hi")
  end

  test "impls without a module of their own share the crate's host module" do
    assert {:module, _} = Code.ensure_loaded(:"Elixir.Patina.BlanketImpl.$impl")
  end
end
